use std::{fs, fmt::Display, path::PathBuf};
use native_tls as tls;
use tokio::net::TcpStream;
use tokio_native_tls::{self as tokio_tls, TlsStream};
use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream};

#[macro_export]
macro_rules!log {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

#[macro_export]
macro_rules!log_error {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
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

pub struct Connection {
    pub who: Who,
    pub stream: WebSocketStream<TlsStream<TcpStream>>
}

pub fn connection_send_error(who: &Who, error: tokio_tungstenite::tungstenite::error::Error) -> Result<Option<Connection>,()> {
    log_error!("Connection with {who} failed :> Error while sending data :> {}", error.to_string());
    Err(())
}

pub async fn connection_close(mut conn: Connection) -> Result<Option<Connection>,()> {
    log!("Closed connection with {}.", conn.who);
    let _ = conn.stream.close(None).await;
    Ok(None)
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

