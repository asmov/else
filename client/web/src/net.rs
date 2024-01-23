use reqwasm::websocket::Message;
use reqwasm::websocket::WebSocketError;
use wasm_bindgen_futures;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use gloo_console::log;
use elsezone_model as model;

const LOCALHOST_WS_URL: &'static str = "ws://127.0.0.1:6432";

pub async fn connect() -> Result<(), WebSocketError> {
    let websocket = WebSocket::open(LOCALHOST_WS_URL).unwrap();
    let (mut tx, mut rcv) = websocket.split();
    let msg = "Hello, server. This is client.".to_string();
    tx.send(Message::Text(msg)).await?;
    if let Some(msg) = rcv.next().await {
        match msg {
            Ok(Message::Bytes(b)) => {
                let world = model::testing::world_from_binary(b).unwrap();
                log!(format!("RCV: {:?}", world));
            },
            _ => panic!("AHHHHH"),
        }
    }

    Ok(())
}
