use futures_util::{SinkExt, StreamExt};
use tokio::{self, io::AsyncReadExt};
use tokio_websockets;
use tokio_websockets::{Error, Message, ServerBuilder};
use elsezone_model as model;

const LOCALHOST_BIND: &'static str = "127.0.0.1:6432";

#[tokio::main]
async fn main() -> Result<(), Error>{
    let websocket_listener = tokio::net::TcpListener::bind(LOCALHOST_BIND).await.unwrap();
        while let Ok((stream, _)) = websocket_listener.accept().await {
          let mut ws_stream = ServerBuilder::new()
            .accept(stream)
            .await?;
    
          tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_stream.next().await {
              if msg.is_text() || msg.is_binary() {
                println!("RECV: {:?}", msg);
                let world = model::testing::create_world();
                let world_bytes = model::testing::world_to_bytes(&world).unwrap();
                ws_stream.send(Message::binary(world_bytes)).await?;
              }
            }
    
            Ok::<_, Error>(())
          });
        }
    
    Ok(())
}

async fn accept_websocket(mut websocket: tokio::net::TcpStream) -> tokio::io::Result<()> {
    let mut s = String::new();
    websocket.read_to_string(&mut s).await?;
    println!("RECV: {s}");
    Ok(())
}