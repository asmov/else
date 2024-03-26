use core::net;
use std::time::Duration;

use reqwasm::websocket;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use yew::{platform::pinned::mpsc::UnboundedReceiver, Callback};
use model::{ClientToZoneMessage, ZoneToClientMessage, UID};
use asmov_else_network_common::*;
use asmov_else_model as model;

use crate::ui::terminal::EntryCategory;

pub enum Stream {
    Outgoing(WebSocket),
}

#[derive(Debug)]
pub enum NetToUIMsg {
    AuthChallenge(model::AuthChallengeMsg),
    Connected(UID),
    Disconnected,
    Frame(model::Frame),
    Synchronized(model::InterfaceView, model::Frame),
}

#[derive(Debug)]
pub enum UItoNetMsg {
    Disconnect,
    AuthAnswer(model::AuthAnswerMsg),
}

impl StreamTrait for Stream {
    async fn send(&mut self, bytes: Vec<u8>) -> SendResult {
        let msg = websocket::Message::Bytes(bytes);
        let result = match self {
            Self::Outgoing(ref mut s) => s.send(msg).await,
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                self.halt().await;
                Err(NetworkError::StreamIO(e.to_string()))
            }
        }
    }

    async fn receive(&mut self) -> ReceiveResult<Vec<u8>> {
        let result = match self {
            Stream::Outgoing(ref mut s) => s.next().await,
        };

        match result {
            Some(Ok(websocket_msg)) => {
                match websocket_msg {
                    websocket::Message::Bytes(bytes) => Ok(bytes),
                    websocket::Message::Text(_) => {
                        self.close_invalid("Expected binary websocket message").await;
                        Err(NetworkError::StreamIO("Unexpected websocket message type".to_string()))
                    },
                }
            },
            Some(Err(e)) => {
                self.halt().await;
                Err(NetworkError::StreamIO(e.to_string()))
            }
            None => {
                self.halt().await;
                Err(NetworkError::StreamDisconnected)
            }
        }
    }

    async fn close_invalid(&mut self, _reason: &str) {
        match self {
            Stream::Outgoing(s) => {
                let _ = s.close().await;
            }
        }
    }

    async fn halt(&mut self) {
        let _error = match self {
            Stream::Outgoing(s) => s.close()
        };
    }
}

pub struct Connection {
    who: Who,
    stream: Stream,
}

impl ConnectionTrait for Connection {
    type StreamType = Stream;

    fn new(who: Who, stream: Self::StreamType) -> Self {
        Self {
            who,
            stream,
        }
    }

    fn who(&self) -> &Who {
        &self.who
    }

    fn stream(&mut self) -> &mut Self::StreamType {
        &mut self.stream
    }
}

pub type ConnectionResult = Result<Connection, NetworkError>;
pub type LogCallback = Callback<(String, EntryCategory)>;

/// auth_msg must be either [model::ClientToZoneMessage::AuthRequest] or [model::ClientToZoneMessage::AuthRegister]
/// On success, returns either [model::ZoneToClientMessage::AuthChallenge] or [model::ZoneToClientMessage::Authorized].
async fn negotiate_session(
    mut conn: Connection,
    net_to_ui_callback: &Callback<NetToUIMsg>,
    ui_to_net_rx: &mut UnboundedReceiver<UItoNetMsg>,
    auth_msg: &model::ClientToZoneMessage,
    _log: &LogCallback
) -> Result<(Connection, model::AuthorizedMsg), NetworkError> {
    negotiate_protocol(&mut conn, false, model::Protocol::ClientToZone, model::Protocol::ZoneToClient).await?;

    // Receive either Connected or ConnectRejected
    let msg: model::ZoneToClientMessage = conn.receive().await?;
    match msg {
        model::ZoneToClientMessage::Connected => {},
        model::ZoneToClientMessage::ConnectRejected => {
            return Err(NetworkError::Rejected{who: conn.who().clone()})
        },
        _ => {
            return Err(NetworkError::UnexpectedResponse{
                who: conn.who().clone(),
                expected: "ZoneToClientMessage::[Connected, ConnectRejected]".to_string() })
        }
    }

    negotiate_auth(conn, net_to_ui_callback, ui_to_net_rx, auth_msg.clone(), _log).await
}

async fn negotiate_auth(
    mut conn: Connection,
    net_to_ui_callback: &Callback<NetToUIMsg>,
    ui_to_net_rx: &mut UnboundedReceiver<UItoNetMsg>,
    auth_msg: model::ClientToZoneMessage,
    _log: &LogCallback
) -> Result<(Connection, model::AuthorizedMsg), NetworkError> {
    // send the auth request/register message
    conn.send(auth_msg).await?;

    // Receive either an an auth rejection, challenge, or successful authorization
    let msg: model::ZoneToClientMessage = conn.receive().await?;
    match msg {
        model::ZoneToClientMessage::AuthChallenge(challenge_msg) => {
            // request the an answer to the challenge from the UI
            net_to_ui_callback.emit(NetToUIMsg::AuthChallenge(challenge_msg));

            // send the UI's answer to the challenge
            let answer = match ui_to_net_rx.next().await.expect("No answer to AuthChallenge") {
                UItoNetMsg::AuthAnswer(answer) => model::ClientToZoneMessage::AuthAnswer(answer),
                _ => panic!("Expected AuthAnswer message from UI"),
            };

            conn.send(answer).await?;

            // receive either an auth rejection or successful authorization
            let msg: model::ZoneToClientMessage = conn.receive().await?;
            match msg {
                model::ZoneToClientMessage::Authorized(authorization) => Ok((conn, authorization)),
                model::ZoneToClientMessage::AuthRejected => Err(NetworkError::Rejected{who: conn.who().clone()}),
                _ => {
                    Err(NetworkError::UnexpectedResponse {
                        who: conn.who().clone(),
                        expected: "ZoneToClientMessage::[AuthAccepted, AuthRejected]".to_string() })
                }
            }
        },
        model::ZoneToClientMessage::Authorized(authorization) => Ok((conn, authorization)),
        model::ZoneToClientMessage::AuthRejected => Err(NetworkError::Rejected{who: conn.who().clone()}),
        _ => {
            Err(NetworkError::UnexpectedResponse {
                who: conn.who().clone(),
                expected: "ZoneToClientMessage::[AuthAccepted, AuthRejected]".to_string() })
        }
    }
}

pub async fn zone_connector_task(
    net_to_ui_callback: Callback<NetToUIMsg>,
    mut ui_to_net_rx: UnboundedReceiver<UItoNetMsg>,
    auth_request: model::ClientToZoneMessage,
    log: Callback<(String,EntryCategory)>
) {
    #[cfg(debug_assertions)]
    matches!(auth_request, model::ClientToZoneMessage::AuthRequest(_) | model::ClientToZoneMessage::AuthRegister(_));

    let mut connect_attempts: isize = -1;

    loop {
        if connect_attempts > -1 {
            net_to_ui_callback.emit(NetToUIMsg::Disconnected);
            let wait = std::cmp::min(MAX_RECONNECT_WAIT, 15 + 3 * connect_attempts as u64);
            log.emit((format!("Reconnecting to zone server in {wait} seconds ..."), EntryCategory::Technical));
            yew::platform::time::sleep(Duration::from_secs(wait)).await;
            connect_attempts += 1;
        } else {
            log.emit((format!("Establishing connection to zone server ({ELSE_LOCALHOST_ZONE_URL})."), EntryCategory::Technical));
            connect_attempts = 0;
        }

        let websocket = match WebSocket::open(ELSE_LOCALHOST_ZONE_URL) {
            Ok(s) => s,
            Err(e) => {
                log.emit((format!("Unable to connect to zone server at `{ELSE_LOCALHOST_ZONE_URL}`. :> {e}"), EntryCategory::Error));
                continue;
            }
        };

        let who = Who::Zone(1, ELSE_LOCALHOST_ZONE_URL.to_string());
        let conn = Connection::new(who.clone(), Stream::Outgoing(websocket));

        log.emit((format!("Connected to {who}."), EntryCategory::Technical));

        let result = negotiate_session(conn, &net_to_ui_callback, &mut ui_to_net_rx, &auth_request, &log).await; 
        let (conn, authorization) = match result {
            Ok((conn, authorization)) => {
                log.emit((format!("Negotiated session with {}.", who.what()), EntryCategory::Technical));
                (conn, authorization)
            },
            Err(e) => {
                log.emit((e.to_string(), EntryCategory::Error));
                continue;
            },
        };

        net_to_ui_callback.emit(NetToUIMsg::Connected(authorization.interface_uid));
        connect_attempts = 0; // reset now that we're connected

        match zone_stream_loop(conn, &net_to_ui_callback, &mut ui_to_net_rx, &log).await {
            Ok(who) => {
                log.emit((format!("Finished session with {who}."), EntryCategory::Technical));
            },
            Err(e) => {
                log.emit((format!("{e}"), EntryCategory::Error));
            }
        }
    }
}

pub async fn zone_stream_loop(mut conn: Connection, status: &Callback<NetToUIMsg>, ui_to_net_rx: &mut UnboundedReceiver<UItoNetMsg>, log: &LogCallback) -> StreamResult {
    loop {
        let msg: ZoneToClientMessage = conn.receive().await?;
        match msg {
            ZoneToClientMessage::InitInterfaceView(timeframe, interface_view_bytes) => {
                let interface_view: model::InterfaceView = bincode::serde::decode_from_slice(&interface_view_bytes, bincode::config::standard())
                    .map_err(|_| NetworkError::StreamIO("Unable to decode interface view".to_string()))?
                    .0;
                status.emit(NetToUIMsg::Synchronized(interface_view, timeframe.frame()));
            },
            ZoneToClientMessage::TimeFrame(newtimeframe) => {
                let timeframe = newtimeframe.timeframe;
                let frame = timeframe.frame();
                status.emit(NetToUIMsg::Frame(frame));
            },
            _ => {
                log.emit((format!("Received message: {:?}", msg), EntryCategory::Technical))
            }
        }
    }
}
