
use std::{fs::File, io::{self, BufReader}, path::{Path, PathBuf}, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{error::Error, Message}};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use bytes::Bytes;
use bincode;
use rustls_pemfile::{certs, rsa_private_keys};
use rustls::{self, server::Acceptor, pki_types::{CertificateDer, PrivateKeyDer}, ServerConfig};
use tokio_rustls::{self, server::TlsStream, TlsAcceptor};

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

fn load_keys(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .next()
        .unwrap()
        .map(Into::into)
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut zone_stream_tasks = Vec::new();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            load_certs(&PathBuf::from("/tmp/end.cert")).unwrap(),
            load_keys(&PathBuf::from("/tmp/end.rsa")).unwrap()).unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));


    let zone_tcp_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_WORLD_ADDR).await.unwrap();
    while let Ok((tcp_stream, addr)) = zone_tcp_listener.accept().await {
        let acceptor = acceptor.clone();
        let tls_stream = acceptor.accept(tcp_stream).await?;
        let websocket_stream = tokio_tungstenite::accept_async(tls_stream).await?;
        dbg!(addr);
        let task = tokio::spawn(zone_stream_task(websocket_stream));
        zone_stream_tasks.push(task);
    }

    Ok(())
}

async fn zone_stream_task(mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>) -> Result<(), ()> {
    // Receive a protocol header from the connecting socket
    if let Some(Ok(received)) = websocket_stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        payload_error(websocket_stream).await;
                        return Err(());
                    }
                };

                // Send our protocol header regardless
                let our_protocol_header = ProtocolHeader::current(Protocol::WorldToZone);
                match websocket_stream.send(
                    Message::binary(Bytes::from(bincode::serialize(&our_protocol_header).unwrap()))).await
                {
                    Ok(_) => {},
                    Err(_) => {
                        let _ = payload_error(websocket_stream);
                        return Err(());
                    }
                }

                // If their header isn't compatible, disconnect
                if !protocol_header.compatible(Protocol::ZoneToWorld) {
                    let _ = payload_error(websocket_stream);
                    return Err(());
                }
            },
            Message::Close(_) => {
                let _ = websocket_stream.close(None).await;
                return Ok(());
            },
            _ => {
                payload_error(websocket_stream).await;
                return Err(());
            }
        }
    }

    while let Some(Ok(received)) = websocket_stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let msg: ZoneToWorldMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        payload_error(websocket_stream).await;
                        return Err(());
                    }
                };

                dbg!(&msg);

                match msg {
                    ZoneToWorldMessage::Connect => {
                        let response = WorldToZoneMessage::Connected;
                        let res = websocket_stream.send(Message::binary(Bytes::from(bincode::serialize(&response).unwrap()))).await;
                        match res {
                            Ok(_) => {},
                            Err(_) => {
                                let _ = payload_error(websocket_stream);
                                return Err(());
                            }
                        }
                    },
                    //ClientToZoneMessage::Transfer => todo!(),
                    _ => {
                        let _ = payload_error(websocket_stream);
                        return Err(());
                    }
                }
            },
            Message::Close(_) =>  {
                let _ = websocket_stream.close(None);
                println!("CLOSE");
                return Ok(())
            },
            _ => {
                let _ = payload_error(websocket_stream);
                return Err(());
            }
        }
    }

    let _ = websocket_stream.close(None);
    dbg!("DISCONNECT");

    Ok(())
}

async fn payload_error(mut websocket_stream: WebSocketStream<TlsStream<TcpStream>>) {
    dbg!("PAYLOAD ERROR()");

    let msg = ZoneToClientMessage::Error(ErrorMsg {
        message_id: 0,
        error_code: ErrorCodes::IllegalWebsocketFrame as u8 });

    let msg = bytes::Bytes::from(bincode::serialize(&msg).unwrap());
    let _ = websocket_stream.send(Message::binary(msg)).await;
    let _ = websocket_stream.close(None).await;
}

