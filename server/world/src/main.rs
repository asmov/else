
use std::{borrow::Cow, fs::{self, File}, io::{self, BufReader, Read}, path::{Path, PathBuf}, sync::Arc};

use elsezone_server_common::certs_dir;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{error::Error, protocol::{frame::coding::CloseCode, CloseFrame}, Message}};
use elsezone_model::{identity, message::*};
use elsezone_network as elsenet;
use bytes::Bytes;
use bincode;
use native_tls::{self as tls};
use tokio_native_tls::{self, TlsStream};
use anyhow::{self, Context};
use elsezone_server_common as server;

fn load_identity(password: String) -> tls::Identity {
    let filepath = certs_dir().join("identity.p12");
    let bytes = &fs::read(filepath).unwrap();
    tls::Identity::from_pkcs12(bytes, &password).unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut zone_stream_tasks = Vec::new();
    let identity_password = String::from("mypass");
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_WORLD_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");
    let mut next_connection_id: usize = 1;

    let identity = load_identity(identity_password);
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        native_tls::TlsAcceptor::builder(identity)
            .build()
            .unwrap());

    let zone_tcp_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_WORLD_ADDR).await.unwrap();
    server::log!("Listening for zone server connections on {bind_address}.");

    while let Ok((tcp_stream, addr)) = zone_tcp_listener.accept().await {
        let acceptor = tls_acceptor.clone();
        let tls_stream = acceptor.accept(tcp_stream).await.unwrap();
        let websocket_stream = tokio_tungstenite::accept_async(tls_stream).await?;

        let zone_who = server::Who::Zone(next_connection_id, format!("{}:{}", addr.ip(), addr.port()));
        next_connection_id += 1;
        server::log!("Established connection with {zone_who}.");

        let task = tokio::spawn(zone_stream_task(zone_who.clone(), websocket_stream));
        zone_stream_tasks.push((zone_who, task));
    }

    Ok(())
}

async fn zone_stream_task(who: server::Who, mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>) -> Result<(), ()> {
    // Receive a protocol header from the connecting socket
    if let Some(Ok(received)) = websocket_stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return payload_error(&who, websocket_stream, "Expected ProtocolHeader").await;
                    }
                };

                // Send our protocol header regardless
                let our_protocol_header = ProtocolHeader::current(Protocol::WorldToZone);
                match websocket_stream.send(
                    Message::binary(Bytes::from(bincode::serialize(&our_protocol_header).unwrap()))).await
                {
                    Ok(_) => {},
                    Err(e) => {
                        return connection_send_error(&who, e);
                    }
                }

                // If their header isn't compatible, disconnect
                if !protocol_header.compatible(Protocol::ZoneToWorld) {
                    return payload_error(&who, websocket_stream, "Incompatible protocol").await;
                }
            },
            Message::Close(_) => {
                return connection_close(&who, websocket_stream).await;
            },
            _ => {
                return payload_error(&who, websocket_stream, "Expected binary websocket frame").await;
            }
        }
    }

    while let Some(Ok(received)) = websocket_stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let msg: ZoneToWorldMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return payload_error(&who, websocket_stream, "Expected ZonetoWorldMessage").await;
                    }
                };

                dbg!(&msg);

                match msg {
                    ZoneToWorldMessage::Connect => {
                        let response = WorldToZoneMessage::Connected;
                        let res = websocket_stream.send(Message::binary(Bytes::from(bincode::serialize(&response).unwrap()))).await;
                        match res {
                            Ok(_) => {},
                            Err(e) => {
                                return connection_send_error(&who, e);
                            }
                        }
                    },
                    _ => {
                        return payload_error(&who, websocket_stream, "Expected ZoneToWorldMessage::Connect").await;
                    }
                }
            },
            Message::Close(_) =>  {
                return connection_close(&who, websocket_stream).await;
            },
            _ => {
                return payload_error(&who, websocket_stream, "Expected binary websocket frame").await;
            }
        }
    }

    println!("Communication with {who} finished.");
    let _ = websocket_stream.close(None).await;
    Ok(())
}

async fn payload_error(who: &server::Who, mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>, reason: &str) -> Result<(), ()> {
    connection_error(who, reason);

    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;

    Err(())
}

fn connection_error(who: &server::Who, reason: &str) {
    server::log_error!("Connection with {who} failed :> {reason}");
}

fn connection_send_error(who: &server::Who, error: tokio_tungstenite::tungstenite::error::Error) -> Result<(),()> {
    server::log_error!("Connection with {who} failed :> Error while sending data :> {}", error.to_string());
    Err(())
}

async fn connection_close(who: &server::Who, mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>) -> Result<(),()> {
    server::log!("Closed connection with {who}.");
    websocket_stream.close(None).await;
    Ok(())
}

