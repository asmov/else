use std::{borrow::Cow, fmt::Display, fs, path::PathBuf};
use chrono::{Datelike, Timelike};
use native_tls as tls;
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_tungstenite::{self, tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Message}, MaybeTlsStream, WebSocketStream};
use elsezone_model::{self as model, message::*};
use elsezone_behavior as behavior;
use bincode;
use futures_util::{SinkExt, StreamExt};
use thiserror;

pub enum Log {
    Standard,
    Error,
    Warning,
    Debug,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, " "),
            Self::Error => write!(f, " ERROR "),
            Self::Warning => write!(f, " WARNING "),
            Self::Debug => write!(f, " DEBUG "),
        }
    }
}

pub fn log(log: Log, output: &str) {
    let now = chrono::offset::Local::now();
    let prefix = format!("[{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}]{log}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_subsec_millis());

    match log {
        Log::Standard | Log::Debug => println!("{prefix}{output}"),
        Log::Error | Log::Warning => eprintln!("{prefix}{output}"),
    }
}

#[macro_export]
macro_rules!log {
    ($($arg:tt)*) => {
        $crate::log($crate::Log::Standard, &format!($($arg)*))
    }
}

#[macro_export]
macro_rules!log_error {
    ($($arg:tt)*) => {
        $crate::log($crate::Log::Error, &format!($($arg)*));
    }
}

pub fn data_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    }
}

pub fn certs_dir() -> PathBuf {
    data_dir().join("certs")
}

pub fn load_identity(password: String) -> tls::Identity {
    let filepath = certs_dir().join("identity.p12");
    let bytes = &fs::read(filepath).unwrap();
    tls::Identity::from_pkcs12(bytes, &password).unwrap()
}

pub fn build_tls_acceptor(identity_password: String) -> tokio_native_tls::TlsAcceptor {
    let identity = load_identity(identity_password);
    let acceptor = native_tls::TlsAcceptor::builder(identity).build().unwrap();
    tokio_native_tls::TlsAcceptor::from(acceptor)
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Failed to send message '{msg_name}' to {who} :> {reason}")]
    Send{who: Who, msg_name: &'static str, reason: String},

    #[error("Failed to receive message '{msg_name}' from {who} :> {reason}")]
    Receive{who: Who, msg_name: &'static str, reason: String},

    #[error("Unexpected response received from {who} when expecting a {expected}.")]
    UnexpectedResponse{who: Who, expected: String},

    #[error("Connection rejected from {who}.")]
    Rejected{who: Who},

    #[error("Abrupt disconnection from {who}")]
    Disconnected{who: Who}
}

pub type SendResult = Result<(), NetworkError>;
pub type ReceiveResult<M> = Result<M, NetworkError>;
pub type ConnectionResult = Result<Connection, NetworkError>;
pub type StreamResult = Result<Who, NetworkError>;

enum Stream {
    Outgoing(WebSocketStream<MaybeTlsStream<TcpStream>>),
    Incoming(WebSocketStream<TlsStream<TcpStream>>)
}

impl Stream {
    pub fn incoming(&mut self) -> &mut WebSocketStream<TlsStream<TcpStream>> {
        match self {
            Self::Outgoing(_) => unreachable!("Stream is not a Server"),
            Self::Incoming(ref mut s) => s
        }
    }

    pub async fn close_invalid(&mut self, reason: &str) {
        match self {
            Stream::Outgoing(s) => {
                let _ = s.close(Some(CloseFrame {
                    code: CloseCode::Invalid,
                    reason: Cow::Borrowed(reason) 
                })).await;
            },
            Stream::Incoming(s) => {
                let _ = s.close(Some(CloseFrame {
                    code: CloseCode::Invalid,
                    reason: Cow::Borrowed(reason) 
                })).await;
            },
        }
    }

    pub async fn halt(&mut self) {
        let _error = match self {
            Stream::Outgoing(s) => s.close(None).await,
            Stream::Incoming(s) => s.close(None).await,
        };
    }
}

pub struct Connection {
    pub who: Who,
    stream: Stream,
}

impl Connection {
    pub fn new_incoming(who: Who, stream: WebSocketStream<TlsStream<TcpStream>>) -> Self {
        Self {
            who,
            stream: Stream::Incoming(stream),
        }
    }

    pub fn new_outgoing(who: Who, stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self {
            who,
            stream: Stream::Outgoing(stream),
        }
    }

    pub fn incoming_stream(&mut self) -> &mut WebSocketStream<TlsStream<TcpStream>> {
        self.stream.incoming()
    }

    pub async fn send_client(&mut self, message: ZoneToClientMessage) -> SendResult {
        let bytes = bincode::serialize(&message).unwrap();
        let msg = Message::Binary(bytes);
        match self.incoming_stream().send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => {
                self.halt().await;
                Err(NetworkError::Send{who: self.who.clone(), msg_name: message.message_name(), reason: e.to_string()})
            }
        }
    }

    pub async fn send<M: Messaging>(&mut self, serializable: M) -> SendResult {
        let bytes = bincode::serialize(&serializable).unwrap();
        let msg = Message::Binary(bytes);
        let result = match self.stream {
            Stream::Outgoing(ref mut s) => s.send(msg).await,
            Stream::Incoming(ref mut s) => s.send(msg).await,
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                self.halt().await;
                Err(NetworkError::Send{who: self.who.clone(), msg_name: serializable.message_name(), reason: e.to_string()})
            }
        }
    }

    pub async fn receive<M: Messaging>(&mut self) -> ReceiveResult<M> {
        let result = match self.stream {
            Stream::Outgoing(ref mut s) => s.next().await,
            Stream::Incoming(ref mut s) => s.next().await,
        };

        match result {
            Some(Ok(websocket_msg)) => {
                match websocket_msg {
                    Message::Binary(bytes) => {
                        let msg: M = match bincode::deserialize(&bytes) {
                            Ok(m) => m,
                            Err(_) => return Err(self.error_payload(M::message_type_name()).await)
                        };

                       Ok(msg) 
                    },
                    Message::Close(_) => {
                        self.halt().await;
                        Err(NetworkError::Disconnected{who: self.who.clone()})
                    },
                    _ => {
                        Err(self.error_payload(M::message_type_name()).await)
                    },
                }
            },
            Some(Err(e)) => {
                self.halt().await;
                Err(NetworkError::Receive{who: self.who.clone(), msg_name: M::message_type_name(), reason: e.to_string()})
            }
            None => {
                self.halt().await;
                Err(NetworkError::Disconnected{who: self.who.clone()})
            }
        }
    }

    pub async fn halt(&mut self) {
        log!("Halted connection with {}.", self.who);
        let _ = self.stream.halt().await;
    }

    pub async fn error_payload(&mut self, expected: &str) -> NetworkError {
        let error = NetworkError::UnexpectedResponse {who: self.who.clone(), expected: expected.to_string() };
        log!("Closed connection with {}.", self.who);
        self.stream.close_invalid(&error.to_string()).await;
        error
    }


}

pub fn connection_send_error(who: &Who, error: tokio_tungstenite::tungstenite::error::Error) -> Result<(),()> {
    log_error!("Connection with {who} failed :> Error while sending data :> {}", error.to_string());
    Err(())
}

pub async fn connection_close(
    who: &Who,
    mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>
) -> Result<(),()> {
    log!("Closed connection with {who}.");
    let _ = websocket_stream.close(None).await;
    Ok(())
}


pub async fn host_connection_close(
    who: &Who,
    mut websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>
) -> Result<(),()> {
    log!("Closed connection with {who}.");
    let _ = websocket_stream.close(None).await;
    Ok(())
}


#[derive(Clone, Debug)]
pub enum Who {
    Client (usize, String),
    Zone (usize, String),
    World (usize, String)
}

impl Display for Who {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Who::Client(num, addr) => write!(f, "client #{num} ({addr})"),
            Who::Zone(num, addr) => write!(f, "zone server #{num} ({addr})"),
            Who::World(num, addr) => write!(f, "world server #{num} ({addr})")
        }
    }
}

