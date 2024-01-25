use std::{borrow::Cow, fs::File, io::{self, BufReader}, path::{Path, PathBuf}, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{self, tungstenite::{self, error::Error, protocol::{frame::coding::CloseCode, CloseFrame}, Message}, Connector, MaybeTlsStream, WebSocketStream};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use bincode;
use rustls::{self, pki_types::CertificateDer, ClientConfig};
use rustls_pemfile::certs;

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}


#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut client_websocket_stream_tasks = Vec::new();

    let certs = load_certs(&PathBuf::from("/tmp/end.chain")).unwrap();
    // TLS
    let mut root_cert_store = rustls::RootCertStore::empty();
    for cert in certs {
        root_cert_store.add(cert).unwrap();
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let connector = Connector::Rustls(Arc::new(config));

    //let (world_websocket_stream, _) = tokio_tungstenite::connect_async(elsenet::ELSE_LOCALHOST_WORLD_URL).await.unwrap();
    let (world_websocket_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
        elsenet::ELSE_LOCALHOST_WORLD_URL,
        None,
        false,
        Some(connector)
    ).await.unwrap();
    let _world_websocket_stream_task = tokio::spawn(world_stream_task(world_websocket_stream));

    let client_websocket_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_ZONE_ADDR).await.unwrap();
    while let Ok((stream, addr)) = client_websocket_listener.accept().await {
        let websocket_stream = tokio_tungstenite::accept_async(stream).await?;
        dbg!(addr);
        let task = tokio::spawn(client_stream_task(websocket_stream));
        client_websocket_stream_tasks.push(task);
    }

    Ok(())
}

async fn client_stream_task(mut websocket_stream: WebSocketStream<TcpStream>) -> Result<(), ()> {
    // protocol verification: connector sends their protocol header to us
    if let Some(Ok(message)) = websocket_stream.next().await {
        match message {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        client_payload_error(websocket_stream, "Expected ProtocolHeader").await;
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
                client_payload_error(websocket_stream, "Expected ProtocolHeader").await;
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
                        client_payload_error(websocket_stream, "Expected ClientToZoneMessage").await;
                        return Err(());
                    }
                };

                match msg {
                    ClientToZoneMessage::Connect => {
                        let msg = Message::Binary(bincode::serialize(&ZoneToClientMessage::Connected).unwrap());
                        websocket_stream.send(msg).await.map_err(|_| ())?;
                    },
                    _ => {
                        let _ = client_payload_error(websocket_stream, "Expected ClientToZoneMessage::Connect");
                        return Err(());
                    }
                }
            },
            Message::Close(_) => {
                let _ = websocket_stream.close(None);
                return Ok(());
            }
            _ => {
                let _ = client_payload_error(websocket_stream, "Expected a binary websocket message");
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

async fn client_payload_error(mut websocket_stream: WebSocketStream<TcpStream>, reason: &str) {
    dbg!("PAYLOAD ERROR()");
    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;
}

async fn world_stream_task(mut websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), ()> {
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
                        world_payload_error(websocket_stream, "Expected ProtocolHeader").await;
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
                world_payload_error(websocket_stream, "Expected ProtocolHeader").await;
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
                        world_payload_error(websocket_stream, "Expected WorldToZoneMessage").await;
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
                        let _ = world_payload_error(websocket_stream,
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
                let _ = world_payload_error(websocket_stream, "Expected a binary websocket frame");
                return Err(());
            }
        }
    }

    let _ = websocket_stream.close(None).await;
    dbg!("DISCONNECT");

    Ok(())
}

async fn world_payload_error(mut websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, reason: &str) {
    dbg!("PAYLOAD ERROR()");
    let _ = websocket_stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;
}

