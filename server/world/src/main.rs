
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{error::Error, Message}};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use bytes::Bytes;
use bincode;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut zone_stream_tasks = Vec::new();

    let zone_tcp_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_WORLD_ADDR).await.unwrap();
    while let Ok((tcp_stream, addr)) = zone_tcp_listener.accept().await {
        let websocket_stream = tokio_tungstenite::accept_async(tcp_stream).await?;
        dbg!(addr);
        let task = tokio::spawn(client_websocket_stream_task(websocket_stream));
        zone_stream_tasks.push(task);
    }

    Ok(())
}

async fn client_websocket_stream_task(mut websocket_stream: WebSocketStream<TcpStream>) -> Result<(), ()> {
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

async fn payload_error(mut websocket_stream: WebSocketStream<TcpStream>) {
    dbg!("PAYLOAD ERROR()");

    let msg = ZoneToClientMessage::Error(ErrorMsg {
        message_id: 0,
        error_code: ErrorCodes::IllegalWebsocketFrame as u8 });

    let msg = bytes::Bytes::from(bincode::serialize(&msg).unwrap());
    let _ = websocket_stream.send(Message::binary(msg)).await;
    let _ = websocket_stream.close(None).await;
}

