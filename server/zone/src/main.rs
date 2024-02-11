use std::{borrow::Cow, fs, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{self, tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Message}, MaybeTlsStream, WebSocketStream};
use native_tls as tls;
use tokio_native_tls::{self, TlsStream};
use bincode;

use elsezone_network_common as elsenet;
use elsezone_model::{self as model, message::*};
use elsezone_server_common as server;

fn load_certs() -> Vec<tls::Certificate> {
    const FILENAMES: [&'static str; 2] = ["cert.der", "root-ca.der"];
    let certs_dir = server::certs_dir();
    let mut certs = Vec::new();

    for filename in FILENAMES {
        let bytes = &fs::read(certs_dir.join(filename)).unwrap();
        let cert = tls::Certificate::from_der(bytes).unwrap();
        certs.push(cert);
    }

    certs
}

fn build_tls_connector() -> tokio_tungstenite::Connector {
    let mut native_tls_connector_builder = tls::TlsConnector::builder();

    #[cfg(debug_assertions)]
    native_tls_connector_builder.danger_accept_invalid_hostnames(true);

    for cert in load_certs() {
        native_tls_connector_builder.add_root_certificate(cert);
    }
    
    let native_tls_connector = native_tls_connector_builder.build().unwrap();
    tokio_tungstenite::Connector::NativeTls(native_tls_connector)
}

struct ZoneRuntime {
    world: Option<model::World>,
    timeframe: Option<model::TimeFrame>
}

impl ZoneRuntime {
    pub fn new() -> Self {
        Self {
            world: None,
            timeframe: None
        }
    }

    pub fn ready(&self) -> bool {
        self.world.is_some()
    }

    pub fn timeframe(&self) -> Option<&model::TimeFrame> {
        self.timeframe.as_ref()
    }

    pub fn world(&self) -> Option<&model::World> {
        self.world.as_ref()
    }

    pub fn sync_world(&mut self, bytes: Vec<u8>) -> Result<&model::World, ()> {
        let world: model::World = bincode::deserialize(&bytes)
            .map_err(|e| ())?;
        self.world = Some(world);
        Ok(self.world.as_ref().unwrap())
    }

    pub fn sync_timeframe(&mut self, timeframe: model::TimeFrame) {
        self.timeframe = Some(timeframe);
    }
}

type ZoneRuntimeSync = std::sync::Arc<tokio::sync::Mutex<ZoneRuntime>>;

#[tokio::main]
async fn main() {
    let runtime = Arc::new(tokio::sync::Mutex::new(ZoneRuntime::new()));

    let _world_connector_task = tokio::spawn(world_connector_task(Arc::clone(&runtime)));
    let _client_listener_task = tokio::spawn(client_listener_task());

    let default_duration = tokio::time::Duration::from_secs(30);
    let sleep = tokio::time::sleep(default_duration);
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                sleep.as_mut().reset(tokio::time::Instant::now() + default_duration);
            }
        }
    }
}

async fn world_connector_task(runtime: ZoneRuntimeSync) -> Result<(), ()> {
    let mut next_world_connection_num: usize = 1;
    let world_server_ip = elsenet::LOCALHOST_IP;
    let world_server_port = elsenet::ELSE_WORLD_PORT;
    let world_server_url = format!("wss://{world_server_ip}:{world_server_port}");

    loop {
        if next_world_connection_num > 1 {
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
        }

        let tls_connector = build_tls_connector();
        let (world_websocket_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
            world_server_url.clone(),
            None,
            false,
            Some(tls_connector)
        ).await.unwrap();

        let world_server_who = server::Who::World(next_world_connection_num, format!("{world_server_ip}:{world_server_port}"));
        next_world_connection_num += 1;
        server::log!("Established connection with {world_server_who}.");

        let conn = server::Connection::new_outgoing(world_server_who, world_websocket_stream);
        let conn = match negotiate_world_session(conn).await {
            Err(e) => {
                server::log_error!("{e}");
                continue;
            },
            Ok(conn) => conn
        };

        match world_stream_task(conn, Arc::clone(&runtime)).await {
            Err(e) => {
                server::log_error!("{e}");
            },
            Ok(who) => {
                server::log!("Session finished with {who}");
            }
        }
    }
}

async fn client_listener_task() -> Result<(), ()> {
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_ZONE_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");
    let mut next_client_connection_num: usize = 1;
    let mut client_websocket_stream_tasks = Vec::new();

    let identity_password = String::from("mypass");
    let tls_acceptor = server::build_tls_acceptor(identity_password);

    let client_websocket_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    server::log!("Listening for client connections on {bind_address}.");

    while let Ok((tcp_stream, addr)) = client_websocket_listener.accept().await {
        let acceptor = tls_acceptor.clone();
        let tls_stream = acceptor.accept(tcp_stream).await.unwrap();
        let websocket_stream = tokio_tungstenite::accept_async(tls_stream).await.unwrap();

        let client_who = server::Who::Client(next_client_connection_num, format!("{}:{}", addr.ip(), addr.port()));
        next_client_connection_num += 1;
        server::log!("Established connection with {client_who}.");

        let task = tokio::spawn(client_stream_task(client_who.clone(), websocket_stream));
        client_websocket_stream_tasks.push((client_who, task));
    }

    Ok(())
}

async fn negotiate_world_session(mut conn: server::Connection) -> server::ConnectionResult {
    // protocol verification: 1. the connector sends its protocol header
    let msg = ProtocolHeader::current(Protocol::ZoneToWorld);
    conn.send(msg).await?;

    // protocol verification: 2. server sends the expected corresponding protocol header or Protocol::Unsupported
    let their_protocol_header: ProtocolHeader = conn.receive().await?;
    if !their_protocol_header.compatible(Protocol::WorldToZone) {
        // either the protocol is Unsupported or the version is wrong
        return Err(conn.error_payload("compatible protocol").await);
    }
     
    // send a connection request
    let msg = ZoneToWorldMessage::Connect;
    conn.send(msg).await?;

    // receive a connection response
    let msg: WorldToZoneMessage = conn.receive().await?;
    match msg {
        WorldToZoneMessage::Connected => {
            server::log!("Connection negotiated with {}.", conn.who);
            Ok(conn)
        },
        WorldToZoneMessage::ConnectRejected => {
            server::log_error!("Connection negotiation rejected by {}.", conn.who);
            conn.halt().await;
            Err(server::NetworkError::Rejected{who: conn.who.clone()})
        },
        _ => Err(conn.error_payload("WorldToZoneMessage::[Connected, ConnectRejected]").await)
    }
}

async fn world_stream_task(mut conn: server::Connection, runtime: ZoneRuntimeSync) -> server::StreamResult {
    loop {
        let msg: WorldToZoneMessage = conn.receive().await?;
        match msg {
            WorldToZoneMessage::WorldBytes(timeframe, bytes) => {
                let frame = timeframe.frame();
                {
                    let mut runtime_lock = runtime.lock().await;
                    runtime_lock.sync_world(bytes).unwrap();
                    runtime_lock.sync_timeframe(timeframe);
                }

                server::log!("Synchronized world at frame {frame}.");
            },
            WorldToZoneMessage::TimeFrame(newtimeframe) => {
                let timeframe = newtimeframe.timeframe;
                let frame = timeframe.frame();
                {
                    let mut runtime_lock = runtime.lock().await;
                    runtime_lock.sync_timeframe(timeframe);
                };

                server::log!("Frame: {frame}");
            },
            _ => {
                server::log!("Received message ::: {:?}", msg);
            }
        }
    }
}

async fn client_payload_error(who: server::Who, mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>, reason: &str) -> Result<(),()> {
    server::log_error!("Connection with {who} failed :> {reason}");
    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;

    Err(())
}

async fn client_stream_task(who: server::Who, mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>) -> Result<(), ()> {
    // protocol verification: connector sends their protocol header to us
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return client_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
                    }
                };

                if !protocol_header.compatible(Protocol::ClientToZone) {
                    server::log_error!("Connection protocol `{protocol_header}` for {who} is incompatible.");
                    let response_protocol_header = ProtocolHeader::current(Protocol::Unsupported);
                    let msg = Message::binary(bincode::serialize(&response_protocol_header).unwrap());
                    if let Err(e) = websocket_stream.send(msg).await {
                        return server::connection_send_error(&who, e);
                    }

                    return server::connection_close(&who, websocket_stream).await;
                }

                let protocol_header = ProtocolHeader::current(Protocol::ZoneToClient);
                let msg = Message::binary(bincode::serialize(&protocol_header).unwrap());
                if let Err(e) = websocket_stream.send(msg).await {
                        return server::connection_send_error(&who, e);
                }
            },
            Message::Close(_) => {
                return server::connection_close(&who, websocket_stream).await;
            },
            _ => {
                return client_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
            }
        }
    } else {
        return server::connection_close(&who, websocket_stream).await;
    }

    // the client should send a connection request
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let msg: ClientToZoneMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return client_payload_error(who, websocket_stream, "Expected ClientToZoneMessage").await;
                    }
                };

                match msg {
                    ClientToZoneMessage::Connect => {
                        let msg = Message::Binary(bincode::serialize(&ZoneToClientMessage::Connected).unwrap());
                        if let Err(e) = websocket_stream.send(msg).await {
                            return server::connection_send_error(&who, e);
                        }

                        server::log!("Session negotiated with {who}.")
                    },
                    _ => {
                        return client_payload_error(who, websocket_stream, "Expected ClientToZoneMessage::Connect").await;
                    }
                }
            },
            Message::Close(_) => {
                return server::connection_close(&who, websocket_stream).await;
            }
            _ => {
                return client_payload_error(who, websocket_stream, "Expected a binary websocket message").await;
            }
        }
    } else {
        return server::connection_close(&who, websocket_stream).await;
    }

    println!("Session has ended with {who}.");
    let _ = websocket_stream.close(None).await;
    Ok(())
}

