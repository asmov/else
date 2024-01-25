
use std::{fs::{self, File}, io::{self, BufReader, Read}, path::{Path, PathBuf}, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{error::Error, Message}};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use bytes::Bytes;
use bincode;
use native_tls::{self as tls};
use tokio_native_tls::{self, TlsStream};
use anyhow::{self, Context};

fn read_file(path: &Path) {

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut zone_stream_tasks = Vec::new();

    let cert = tls::Identity::from_pkcs12(
            &fs::read("cert/identity.p12").unwrap(),
            "mypass")
        .unwrap();

    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        native_tls::TlsAcceptor::builder(cert)
            .build()
            .unwrap());


    let zone_tcp_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_WORLD_ADDR).await.unwrap();
    while let Ok((tcp_stream, addr)) = zone_tcp_listener.accept().await {
        let acceptor = tls_acceptor.clone();
        let tls_stream = acceptor.accept(tcp_stream).await.unwrap();
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

