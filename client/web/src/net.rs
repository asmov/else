use std::time::Duration;

use reqwasm::websocket;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use yew::Callback;
use model::{AuthChallengeMsg, ClientToZoneMessage, ZoneToClientMessage};
use asmov_else_network_common::*;
use asmov_else_model as model;

use crate::ui::terminal::EntryCategory;

pub enum Stream {
    Outgoing(WebSocket),
}

pub enum Status {
    Connected,
    Disconnected,
    Frame(model::Frame),
    Synchronized(model::InterfaceView, model::Frame),
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
    auth_msg: model::ClientToZoneMessage,
    _log: &LogCallback
) -> Result<(Connection, ClientToZoneMessage), NetworkError> {
    negotiate_protocol(&mut conn, false, model::Protocol::ClientToZone, model::Protocol::ZoneToClient).await?;

    // Receive either Connected or ConnectRejected
    let msg: model::ZoneToClientMessage = conn.receive().await?;
    match msg {
        model::ZoneToClientMessage::Connected => {},
        model::ZoneToClientMessage::ConnectRejected => {
            return Err(NetworkError::Rejected{who: conn.who().clone()})
        },
        _ => {
            Err(NetworkError::UnexpectedResponse{
                who: conn.who().clone(),
                expected: "ZoneToClientMessage::[Connected, ConnectRejected]".to_string() })
        }
    }

    negotiate_auth(conn, auth_msg, log).await
}

async fn negotiate_auth(
    mut conn: Connection,
    auth_msg: model::ClientToZoneMessage,
    _log: &LogCallback
) -> Result<(Connection, ClientToZoneMessage), NetworkError> {
    // ensure that login is either AuthRequest or AuthRegister
    let (is_login, is_registration, is_challenge_answer) = match auth_msg {
        model::ClientToZoneMessage::AuthRequest(_) => (true, false, false),
        model::ClientToZoneMessage::AuthRegister(_) => (false, true, false), 
        model::ClientToZoneMessage::AuthAnswer(_) => (false, false, true), 
        _ => panic!("Expected auth_msg to be either AuthRequest, AuthRegister, or AuthAnswer."),
    };

    // send the auth request/register message
    conn.send(auth_msg).await?;

    // Receive either an an auth rejection, challenge, or successful authorization
    let msg: model::ZoneToClientMessage = conn.receive().await?;
    match msg {
        model::ZoneToClientMessage::AuthChallenge(challenge_msg) => Ok((conn, msg)),
        model::ZoneToClientMessage::Authorized(authorization_msg) => Ok((conn, msg)),
        model::ZoneToClientMessage::AuthRejected => Err(NetworkError::Rejected{who: conn.who().clone()}),
        _ => {
            Err(NetworkError::UnexpectedResponse {
                who: conn.who().clone(),
                expected: "ZoneToClientMessage::[AuthAccepted, AuthRejected]".to_string() })
        }
    }
}

pub async fn zone_connector_task(status: Callback<Status>, log: Callback<(String,EntryCategory)>) {
    let mut connect_attempts: isize = -1;

    loop {
        if connect_attempts > -1 {
            status.emit(Status::Disconnected);
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

        let conn = match negotiate_session(conn, &log).await {
            Ok(conn) => {
                log.emit((format!("Negotiated session with {}.", who.what()), EntryCategory::Technical));
                conn
            },
            Err(e) => {
                log.emit((e.to_string(), EntryCategory::Error));
                continue;
            },
        };

        status.emit(Status::Connected);
        connect_attempts = 0; // reset now that we're connected

        match zone_stream_loop(conn, &status, &log).await {
            Ok(who) => {
                log.emit((format!("Finished session with {who}."), EntryCategory::Technical));
            },
            Err(e) => {
                log.emit((format!("{e}"), EntryCategory::Error));
            }
        }
    }
}

pub async fn zone_stream_loop(mut conn: Connection, status: &Callback<Status>, log: &LogCallback) -> StreamResult {
    loop {
        let msg: ZoneToClientMessage = conn.receive().await?;
        match msg {
            ZoneToClientMessage::InitInterfaceView(timeframe, interface_view_bytes) => {
                let interface_view: model::InterfaceView = bincode::serde::decode_from_slice(&interface_view_bytes, bincode::config::standard())
                    .map_err(|_| NetworkError::StreamIO("Unable to decode interface view".to_string()))?
                    .0;
                status.emit(Status::Synchronized(interface_view, timeframe.frame()));
            },
            ZoneToClientMessage::TimeFrame(newtimeframe) => {
                let timeframe = newtimeframe.timeframe;
                let frame = timeframe.frame();
                status.emit(Status::Frame(frame));
            },
            _ => {
                log.emit((format!("Received message: {:?}", msg), EntryCategory::Technical))
            }
        }
    }
}
