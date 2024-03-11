use thiserror;
use bincode;
use asmov_else_model as model;

pub const LOCALHOST_IP: &'static str = "127.0.0.1";
pub const ELSE_UNIVERSE_PORT: u16 = 3152;
pub const ELSE_WORLD_PORT: u16 = 3153;
pub const ELSE_ZONE_PORT: u16 = 8443;
pub const ELSE_LOCALHOST_ZONE_ADDR: &'static str = "127.0.0.1:8443";
pub const ELSE_LOCALHOST_WORLD_ADDR: &'static str = "127.0.0.1:3153";
pub const ELSE_LOCALHOST_UNIVERSE_ADDR: &'static str = "127.0.0.1:3152";
pub const ELSE_LOCALHOST_ZONE_URL: &'static str = "wss://127.0.0.1:8443";
pub const ELSE_LOCALHOST_WORLD_URL: &'static str = "wss://127.0.0.1:3153";
pub const ELSE_LOCALHOST_UNIVERSE_URL: &'static str = "wss://127.0.0.1:3152";
pub const WEBSOCKET_PAYLOAD_ERROR: u16 = 1007;

pub const MAX_RECONNECT_WAIT: u64 = 120;

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
    Disconnected{who: Who},

    #[error("Stream IO error: {0}")]
    StreamIO(String),

    #[error("Stream disconnected")]
    StreamDisconnected
}

pub type SendResult = Result<(), NetworkError>;
pub type ReceiveResult<M> = Result<M, NetworkError>;
pub type StreamResult = Result<Who, NetworkError>;
/// Void Ok and Err
pub type TaskResult = Result<(),()>;
/// Error is logged via `log_error!()` prior to returning. Success may, optionally, be logged as well.
pub type LoggedResult<T> = Result<T, ()>;



#[derive(Clone, Debug)]
pub enum Who {
    Client (usize, String),
    Zone (usize, String),
    World (usize, String),
    Universe (usize, String)
}

impl Who {
    pub fn what(&self) -> &'static str {
        match self {
            Self::Client(_,_) => "client",
            Self::Zone(_,_) => "zone server",
            Self::World(_,_) => "world server",
            Self::Universe(_,_) => "universe server",
        }
    }
}

impl std::fmt::Display for Who {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Who::Client(num, addr) => write!(f, "client #{num} ({addr})"),
            Who::Zone(num, addr) => write!(f, "zone server #{num} ({addr})"),
            Who::World(num, addr) => write!(f, "world server #{num} ({addr})"),
            Who::Universe(num, addr) => write!(f, "universe server #{num} ({addr})")
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait StreamTrait {
    /// Will throw only NetworkError variants: StreamIO
    async fn send(&mut self, bytes: Vec<u8>) -> SendResult;
    /// Will throw only NetworkError variants: StreamIO and StreamDisconnected
    async fn receive(&mut self) -> ReceiveResult<Vec<u8>>;
    async fn close_invalid(&mut self, reason: &str);
    async fn halt(&mut self);
}

#[allow(async_fn_in_trait)]
pub trait ConnectionTrait {
    type StreamType: StreamTrait;

    fn new(who: Who, stream: Self::StreamType) -> Self;

    fn who(&self) -> &Who;
    fn stream(&mut self) -> &mut Self::StreamType;

    async fn send<M: model::Messaging>(&mut self, serializable: M) -> SendResult {
        let config = bincode::config::standard();
        let bytes = bincode::serde::encode_to_vec(&serializable, config).unwrap();
        let result = self.stream().send(bytes).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                self.halt().await;
                Err(NetworkError::Send{who: self.who().clone(), msg_name: serializable.message_name(), reason: e.to_string()})
            }
        }
    }

    async fn receive<M: model::Messaging>(&mut self) -> ReceiveResult<M> {
        match self.stream().receive().await {
            Ok(bytes) => {
                let msg: M = match bincode::serde::decode_from_slice(&bytes, bincode::config::standard()) {
                    Ok(m) => m.0,
                    Err(_) => return Err(self.error_payload(M::message_type_name()).await)
                };

                Ok(msg) 
            },
            Err(NetworkError::StreamIO(e)) => {
                Err(NetworkError::Receive{who: self.who().clone(), msg_name: M::message_type_name(), reason: e.to_string()})
            },
            Err(NetworkError::StreamDisconnected) => {
                Err(NetworkError::Disconnected{who: self.who().clone()})
            },
            Err(_) => unreachable!("Unexpected NetworkError variant from StreamTrait::send()")
        }
    }

    async fn halt(&mut self) {
        self.stream().halt().await;
    }

    async fn error_payload(&mut self, expected: &str) -> NetworkError {
        let error = NetworkError::UnexpectedResponse {who: self.who().clone(), expected: expected.to_string() };
        self.stream().close_invalid(&error.to_string()).await;
        error
    }
}






