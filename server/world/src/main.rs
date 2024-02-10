
use std::{borrow::Cow, time::Duration};

use futures_util::{SinkExt, StreamExt, TryFuture, TryFutureExt};
use server::Connection;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Message}};
use elsezone_model::message::*;
use elsezone_network_common as elsenet;
use bytes::Bytes;
use bincode;
use tokio_native_tls::{self, TlsStream};
use anyhow;
use elsezone_server_common::{self as server, connection_close, connection_send_error};
use elsezone_behavior as behavior;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let identity_password = String::from("mypass");
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_WORLD_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");

    let mut runtime = behavior::WorldRuntime::load()?;
    
    let tls_acceptor = server::build_tls_acceptor(identity_password);
    let zone_tcp_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    
    server::log!("Listening for zone server connections on {bind_address}.");
    let _listener_task = tokio::spawn(listener_task(zone_tcp_listener, tls_acceptor));

    let sleep = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                runtime.tick().unwrap();
                server::log!("Frame: {}", runtime.timeframe().frame());
                sleep.as_mut().reset(tokio::time::Instant::now() + Duration::from_secs(10))
            }
        }
    }

    Ok(())
}

async fn listener_task(
    zone_tcp_listener: tokio::net::TcpListener,
    tls_acceptor: tokio_native_tls::TlsAcceptor
) -> anyhow::Result<()> {
    let mut next_connection_id: usize = 1;
    let mut zone_stream_tasks = Vec::new();

    while let Ok((tcp_stream, addr)) = zone_tcp_listener.accept().await {
        let acceptor = tls_acceptor.clone();
        let tls_stream = acceptor.accept(tcp_stream).await.unwrap();
        let websocket_stream = tokio_tungstenite::accept_async(tls_stream).await?;

        let zone_who = server::Who::Zone(next_connection_id, format!("{}:{}", addr.ip(), addr.port()));
        next_connection_id += 1;
        server::log!("Established connection with {zone_who}.");

        let conn = Connection { who: zone_who.clone(), stream: websocket_stream };
        let task = tokio::spawn(zone_stream_handshake_task(conn))
            .and_then(|result| tokio::spawn(zone_stream_task(result.unwrap().unwrap())));
        zone_stream_tasks.push((zone_who, task));
    }

    Ok(())
}

async fn zone_stream_handshake_task(mut conn: Connection) -> Result<Option<Connection>, ()> {
    // Receive a protocol header from the connecting socket
    if let Some(Ok(received)) = conn.stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let protocol_header: ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return payload_error(conn, "Expected ProtocolHeader").await;
                    }
                };

                // Send our protocol header regardless
                let our_protocol_header = ProtocolHeader::current(Protocol::WorldToZone);
                match conn.stream.send(
                    Message::binary(bincode::serialize(&our_protocol_header).unwrap())).await
                {
                    Ok(_) => {},
                    Err(e) => {
                        return connection_send_error(&conn.who, e);
                    }
                }

                // If their header isn't compatible, disconnect
                if !protocol_header.compatible(Protocol::ZoneToWorld) {
                    return payload_error(conn, "Incompatible protocol").await;
                }
            },
            Message::Close(_) => {
                return connection_close(conn).await;
            },
            _ => {
                return payload_error(conn, "Expected binary websocket frame").await;
            }
        }
    }

    while let Some(Ok(received)) = conn.stream.next().await {
        match received {
            Message::Binary(bytes) => {
                let msg: ZoneToWorldMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        return payload_error(conn, "Expected ZonetoWorldMessage").await;
                    }
                };

                match msg {
                    ZoneToWorldMessage::Connect => {
                        let response = WorldToZoneMessage::Connected;
                        if let Err(e) = conn.stream.send(Message::binary(Bytes::from(bincode::serialize(&response).unwrap()))).await {
                            return connection_send_error(&conn.who, e);
                        }

                        server::log!("Session negotiated with {}.", conn.who);
                        return Ok(Some(conn))
                    },
                    _ => {
                        return payload_error(conn, "Expected ZoneToWorldMessage::Connect").await;
                    }
                }
            },
            Message::Close(_) =>  {
                return connection_close(conn).await;
            },
            _ => {
                return payload_error(conn, "Expected binary websocket frame").await;
            }
        }
    }

    unreachable!("Connection not handled")
}

async fn zone_stream_task(mut conn: Connection) -> Result<(), ()> {
    println!("Communication with {} finished.", conn.who);
    let _ = conn.stream.close(None).await;
    Ok(())
}

async fn payload_error(mut conn: Connection, reason: &str) -> Result<Option<Connection>, ()> {
    connection_error(&conn.who, reason);

    let _ = conn.stream.close(Some(CloseFrame {
        code: CloseCode::Invalid,
        reason: Cow::Borrowed(reason) 
    })).await;

    Err(())
}

fn connection_error(who: &server::Who, reason: &str) {
    server::log_error!("Connection with {who} failed :> {reason}");
}