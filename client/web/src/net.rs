use std::time::Duration;

use reqwasm::websocket;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use gloo_console::log;
use elsezone_model as model;
use yew::Callback;
use elsezone_network_common::*;

use crate::ui::terminal::EntryCategory;

pub enum Stream {
    Outgoing(WebSocket),
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

async fn negotiate_session(mut conn: Connection, log: &Callback<(String, EntryCategory)>) -> ConnectionResult {
    // Send the protocol header
    let our_protocol_header = model::ProtocolHeader::current(model::Protocol::ClientToZone);
    conn.send(our_protocol_header).await?;
    
    // The server should send a protocol header back
    let their_protocol_header: model::ProtocolHeader = conn.receive().await?;
    log.emit((format!("Got protocol :> {:?}", their_protocol_header).to_string(), EntryCategory::Debug));

    if !their_protocol_header.compatible(model::Protocol::ZoneToClient) {
        log!(format!("Incompatible protocol: {:?}", their_protocol_header));
        return Err(conn.error_payload("compatible protocol").await);
    }

    // Send the connection request
    let msg = model::ClientToZoneMessage::Connect;
    conn.send(msg).await?;

    // Receive either Connected or ConnectRejected
    let msg: model::ZoneToClientMessage = conn.receive().await?;
    match msg {
        model::ZoneToClientMessage::Connected => {
            Ok(conn)
        },
        model::ZoneToClientMessage::ConnectRejected => {
            Err(NetworkError::Rejected{who: conn.who().clone()})
        },
        _ => {
            Err(NetworkError::UnexpectedResponse{
                who: conn.who().clone(),
                expected: "ZoneToClientMessage::[Connected, ConnectRejected]".to_string() })
        }
    }
}

pub async fn connect(log: Callback<(String,EntryCategory)>) -> TaskResult {
    let mut connect_attempts: isize = -1;
    loop {
        if connect_attempts > -1 {
            let wait = std::cmp::min(MAX_RECONNECT_WAIT, 15 + 3 * connect_attempts as u64);
            yew::platform::time::sleep(Duration::from_secs(wait)).await;
            connect_attempts += 1;
        } else {
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

        log.emit((format!("Connected to {who}"), EntryCategory::Debug));

        let mut conn = match negotiate_session(conn, &log).await {
            Ok(conn) => {
                log.emit((format!("Negotiated session with {who}."), EntryCategory::Debug));
                conn
            },
            Err(e) => {
                log.emit((e.to_string(), EntryCategory::Error));
                return Err(())
            },
        };

        connect_attempts = 0; // reset now that we're connected
        conn.send(model::ClientToZoneMessage::Disconnect).await.unwrap();
        conn.halt().await;
        break; //todo: message loop
    }

    Ok(())
}
