use std::{borrow::Cow, fmt::Display, fs, path::PathBuf};
use chrono::{Datelike, Timelike};
use native_tls as tls;
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_tungstenite::{self, tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Message}, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt, StreamExt};

pub use asmov_else_network_common::*;

pub type ConnectionResult = Result<Connection, NetworkError>;
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

pub enum Stream {
    Outgoing(WebSocketStream<MaybeTlsStream<TcpStream>>),
    Incoming(WebSocketStream<TlsStream<TcpStream>>)
}

impl StreamTrait for Stream {
    async fn send(&mut self, bytes: Vec<u8>) -> SendResult {
        let msg = Message::Binary(bytes);
        let result = match self {
            Self::Outgoing(ref mut s) => s.send(msg).await,
            Self::Incoming(ref mut s) => s.send(msg).await,
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
            Stream::Incoming(ref mut s) => s.next().await,
        };

        match result {
            Some(Ok(websocket_msg)) => {
                match websocket_msg {
                    Message::Binary(bytes) => Ok(bytes),
                    Message::Close(_close_frame) => {
                        self.halt().await;
                        Err(NetworkError::StreamDisconnected)
                    },
                    _ => {
                        self.close_invalid("Binary").await;
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

    async fn close_invalid(&mut self, reason: &str) {
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

    async fn halt(&mut self) {
        let _error = match self {
            Stream::Outgoing(s) => s.close(None).await,
            Stream::Incoming(s) => s.close(None).await,
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