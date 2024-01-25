use std::{borrow::Cow, fs::{self, File}, io::{self, BufReader}, path::{Path, PathBuf}, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{self, tungstenite::{self, error::Error, protocol::{frame::coding::CloseCode, CloseFrame}, Message}, Connector, MaybeTlsStream, WebSocketStream};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use elsezone_server_common as server;
use bincode;
use native_tls as tls;
use anyhow;
use tokio_native_tls;

fn load_certs() -> Vec<tls::Certificate> {
    const FILENAMES: [&'static str; 2] = ["cert.der", "root-ca.der"];
    let certs_dir = server::certs_dir();
    let mut certs = Vec::new();

    for filename in FILENAMES {
        let bytes = &fs::read(certs_dir.join("cert.der")).unwrap();
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

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let mut client_websocket_stream_tasks = Vec::new();
    let mut next_connection_id: usize = 1;
    let world_server_ip = elsenet::LOCALHOST_IP;
    let world_server_port = elsenet::ELSE_WORLD_PORT;
    let world_server_url = format!("wss://{world_server_ip}:{world_server_port}");

    let tls_connector = build_tls_connector();

    let (world_websocket_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
        world_server_url,
        None,
        false,
        Some(tls_connector)
    ).await.unwrap();


    let world_server_who = server::Who::World(next_connection_id, format!("{world_server_ip}:{world_server_port}"));
    next_connection_id += 1;
    server::log!("Established connection with {world_server_who}.");

    let _world_websocket_stream_task = tokio::spawn(world_stream_task(world_server_who, world_websocket_stream));

    let client_websocket_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_ZONE_ADDR).await.unwrap();
    while let Ok((stream, addr)) = client_websocket_listener.accept().await {
        let websocket_stream = tokio_tungstenite::accept_async(stream).await?;

        let client_who = server::Who::Client(next_connection_id, format!("{}:{}", addr.ip(), addr.port()));
        next_connection_id += 1;
        server::log!("Established connection with {client_who}.");

        let task = tokio::spawn(client_stream_task(client_who, websocket_stream));
        client_websocket_stream_tasks.push(task);
    }

    Ok(())
}

async fn client_stream_task(who: server::Who, mut websocket_stream: WebSocketStream<TcpStream>) -> Result<(), ()> {
    // protocol verification: connector sends their protocol header to us
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        client_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
                        return Err(());
                    }
                };

                if !protocol_header.compatible(Protocol::ClientToZone) {
                    let msg = Message::Binary(bincode::serialize(
                        &ProtocolHeader::current(Protocol::Unsupported)).unwrap());
                    websocket_stream.send(msg).await.map_err(|_| ())?;
                    let _ = websocket_stream.close(None).await;
                    return Err(());
                }
            },
            Message::Close(_) => {
                let _ = websocket_stream.close(None).await;
                return Ok(());
            },
            _ => {
                client_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
                return Err(());
            }
        }
    } else {
        let _ = websocket_stream.close(None).await;
        return Ok(());
    }

    // the client should send a connection request
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let msg: ClientToZoneMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        client_payload_error(who, websocket_stream, "Expected ClientToZoneMessage").await;
                        return Err(());
                    }
                };

                match msg {
                    ClientToZoneMessage::Connect => {
                        let msg = Message::Binary(bincode::serialize(&ZoneToClientMessage::Connected).unwrap());
                        websocket_stream.send(msg).await.map_err(|_| ())?;
                    },
                    _ => {
                        client_payload_error(who, websocket_stream, "Expected ClientToZoneMessage::Connect").await;
                        return Err(())
                    }
                }
            },
            Message::Close(_) => {
                let _ = websocket_stream.close(None);
                return Ok(());
            }
            _ => {
                let _ = client_payload_error(who, websocket_stream, "Expected a binary websocket message");
                return Err(());
            }
        }
    } else {
        let _ = websocket_stream.close(None).await;
        return Ok(());
    }

    let _ = websocket_stream.close(None);
    dbg!("DISCONNECT");

    Ok(())
}

async fn world_stream_task(who: server::Who, mut websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), ()> {
    // protocol verification: 1. the connector sends its protocol header
    let msg = Message::Binary(bincode::serialize(
                &ProtocolHeader::current(Protocol::ZoneToWorld)).unwrap());
    websocket_stream.send(msg).await.map_err(|_| ())?;

    // protocol verification: 2. server sends the expected corresponding protocol header or Protocol::Unsupported
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        world_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
                        return Err(())
                    }
                };

                if !protocol_header.compatible(Protocol::WorldToZone) {
                    // either the protocol is Unsupported or the version is wrong
                    let _ = websocket_stream.close(None).await;
                    return Err(())
                }
            },
            Message::Close(_) => {
                let _ = websocket_stream.close(None).await;
                return Ok(())
            },
            _ => {
                world_payload_error(who, websocket_stream, "Expected ProtocolHeader").await;
                return Err(())
            }
        }
    }
     
    // send a connection request
    let msg = Message::Binary(bincode::serialize(&ZoneToWorldMessage::Connect).unwrap());
    websocket_stream.send(msg).await.map_err(|_| ())?;

    // receive a connection response
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let msg: WorldToZoneMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        world_payload_error(who, websocket_stream, "Expected WorldToZoneMessage").await;
                        return Err(());
                    }
                };

                dbg!(&msg);

                match msg {
                    WorldToZoneMessage::Connected => {
                        println!("CONNECTED TO WORLD");
                    },
                    WorldToZoneMessage::ConnectRejected => {
                        eprintln!("Connection to World rejected.");
                        let _ = websocket_stream.close(None).await;
                        return Err(());
                    }
                    _ => {
                        let _ = world_payload_error(who, websocket_stream,
                            "Expected WorldToZoneMessage::[Connected, ConnectRejected]");
                        return Err(());
                    }
                }
            },
            Message::Close(_) =>  {
                let _ = websocket_stream.close(None).await;
                println!("CLOSE");
                return Ok(())
            },
            _ => {
                let _ = world_payload_error(who, websocket_stream, "Expected a binary websocket frame");
                return Err(());
            }
        }
    }

    let _ = websocket_stream.close(None).await;
    dbg!("DISCONNECT");

    Ok(())
}

async fn world_payload_error(
    who: server::Who,
    mut websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    reason: &str)
{
    server::log_error!("Connection with {who} failed :> {reason}");

    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;
}

async fn client_payload_error(who: server::Who, mut websocket_stream: WebSocketStream<TcpStream>, reason: &str) {
    server::log_error!("Connection with {who} failed :> {reason}");
    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;
}

