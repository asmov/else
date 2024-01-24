use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_websockets::{self, CloseCode, WebSocketStream};
use tokio_websockets::{Error, Message, ServerBuilder};
use tokio_tungstenite::{self, MaybeTlsStream, tungstenite};
use elsezone_model::message::*;
use elsezone_network as elsenet;
use bytes::Bytes;
use bincode;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut client_websocket_stream_tasks = Vec::new();

    let (world_websocket_stream, _) = tokio_tungstenite::connect_async(elsenet::ELSE_LOCALHOST_WORLD_URL).await.unwrap();
    let world_websocket_stream_task = tokio::spawn(world_websocket_stream_task(world_websocket_stream));

    let client_websocket_listener = tokio::net::TcpListener::bind(elsenet::ELSE_LOCALHOST_ZONE_ADDR).await.unwrap();
    while let Ok((stream, addr)) = client_websocket_listener.accept().await {
        let websocket_stream = ServerBuilder::new().accept(stream).await?;
        dbg!(addr);
        let task = tokio::spawn(client_websocket_stream_task(websocket_stream));
        client_websocket_stream_tasks.push(task);
    }

    Ok(())
}

async fn client_websocket_stream_task(mut websocket_stream: WebSocketStream<TcpStream>) -> Result<(), ()> {
    while let Some(Ok(received)) = websocket_stream.next().await {
        if received.is_binary() {
            dbg!(&received);
            let msg: ClientToZoneMessage = match bincode::deserialize(received.as_payload()) {
                Ok(msg) => msg,
                Err(_) => {
                    payload_error(websocket_stream).await;
                    return Err(());
                }
            };
            dbg!(&msg);

            match msg {
                ClientToZoneMessage::Connect => {
                    let response = ZoneToClientMessage::Connected;
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
        } else if received.is_close() {
            let _ = websocket_stream.close();
            println!("CLOSE");
            return Ok(())
        }
    }

    let _ = websocket_stream.close();
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
    let _ = websocket_stream.send(Message::close(Some(CloseCode::INVALID_FRAME_PAYLOAD_DATA), "")).await;
    let _ = websocket_stream.close().await;
}

async fn world_websocket_stream_task(mut websocket_stream: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), ()> {
    websocket_stream.send(
        tungstenite::Message::binary(
            bincode::serialize(&ZoneToWorldMessage::Connect).unwrap()
        )
    ).await.unwrap();

    while let Some(Ok(received)) = websocket_stream.next().await {
        match received {
            tungstenite::Message::Binary(bytes) => {
                let msg: WorldToZoneMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        tls_payload_error(websocket_stream).await;
                        return Err(());
                    }
                };

                dbg!(&msg);

                match msg {
                    WorldToZoneMessage::Connected => {
                        println!("CONNECTED TO WORLD");
                    },
                    //ClientToZoneMessage::Transfer => todo!(),
                    _ => {
                        let _ = tls_payload_error(websocket_stream);
                        return Err(());
                    }
                }
            },
            tungstenite::Message::Close(_) =>  {
                let _ = websocket_stream.close(None);
                println!("CLOSE");
                return Ok(())
            },
            _ => {
                let _ = tls_payload_error(websocket_stream);
                return Err(());
            }
        }
    }

    let _ = websocket_stream.close(None);
    dbg!("DISCONNECT");

    Ok(())
}

async fn tls_payload_error(mut websocket_stream: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>) {
    dbg!("PAYLOAD ERROR()");

    let msg = ZoneToClientMessage::Error(ErrorMsg {
        message_id: 0,
        error_code: ErrorCodes::IllegalWebsocketFrame as u8 });

    let msg = bytes::Bytes::from(bincode::serialize(&msg).unwrap());
    let _ = websocket_stream.send(tungstenite::Message::binary(msg)).await;
    let _ = websocket_stream.close(None).await;
}

