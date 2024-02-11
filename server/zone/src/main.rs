use std::sync::Arc;

use tokio_tungstenite;

use elsezone_network_common as elsenet;
use elsezone_model::message::*;
use elsezone_server_common as server;
use elsezone_zone_server::*;

#[tokio::main]
async fn main() {
    let runtime = Arc::new(tokio::sync::Mutex::new(ZoneRuntime::new()));

    let _world_connector_task = tokio::spawn(world_connector_task(Arc::clone(&runtime)));
    let _client_listener_task = tokio::spawn(client_listener_task(Arc::clone(&runtime)));

    let default_duration = tokio::time::Duration::from_secs(30);
    let sleep = tokio::time::sleep(default_duration);
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                sleep.as_mut().reset(tokio::time::Instant::now() + default_duration);
            }
        }
    }
}

async fn world_connector_task(runtime: ZoneRuntimeSync) {
    let mut next_world_connection_num: usize = 1;
    let world_server_ip = elsenet::LOCALHOST_IP;
    let world_server_port = elsenet::ELSE_WORLD_PORT;
    let world_server_url = format!("wss://{world_server_ip}:{world_server_port}");
    let mut reconnect_attempts: i32 = -1;

    loop {
        if reconnect_attempts > -1 {
            let wait = 15 + 3 * reconnect_attempts as u64;
            server::log!("Reconnecting in {wait} seconds ...");
            tokio::time::sleep(tokio::time::Duration::from_secs(15 + wait)).await;
            reconnect_attempts += 1;
        } else {
            reconnect_attempts = 0;
        }

        let tls_connector = build_tls_connector();
        let result = tokio_tungstenite::connect_async_tls_with_config(
            world_server_url.clone(),
            None,
            false,
            Some(tls_connector)
        ).await;

        let world_websocket_stream = match result {
            Ok((stream, _)) => stream,
            Err(e) => {
                server::log_error!("Unable to connect to world server at {world_server_url} :> {e}");
                continue
            }
        };

        let world_server_who = server::Who::World(next_world_connection_num, format!("{world_server_ip}:{world_server_port}"));
        next_world_connection_num += 1;
        server::log!("Established connection with {world_server_who}.");

        let conn = server::Connection::new_outgoing(world_server_who, world_websocket_stream);
        let conn = match negotiate_world_session(conn).await {
            Err(e) => {
                server::log_error!("{e}");
                continue;
            },
            Ok(conn) => conn
        };

        reconnect_attempts = 0; // reset after a successful handshake

        match world_stream_task(conn, Arc::clone(&runtime)).await {
            Err(e) => {
                server::log_error!("{e}");
            },
            Ok(who) => {
                server::log!("Session finished with {who}");
            }
        }
    }
}

async fn bind_client_listener() -> std::io::Result<tokio::net::TcpListener> {
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_ZONE_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");
    tokio::net::TcpListener::bind(&bind_address).await
        .and_then(|listener| {
            server::log!("Listening for client connections on {bind_address}.");
            Ok(listener)
        })
        .map_err(|e| {
            server::log_error!("Unable to bind to address {bind_address}. :> {e}");
            e
        })
}

async fn client_listener_task(runtime: ZoneRuntimeSync) {
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_ZONE_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");
 
    let mut next_client_connection_num: usize = 1;
    let mut client_websocket_stream_tasks = Vec::new();

    let identity_password = String::from("mypass");
    let tls_acceptor = server::build_tls_acceptor(identity_password);

    let client_websocket_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    server::log!("Listening for client connections on {bind_address}.");

    loop {
        let (tcp_stream, addr) = match client_websocket_listener.accept().await {
            Ok(r) => r,
            Err(e) => {
                server::log_error!("Unable to accept client connection :> {e}");
                break;
            }
        };

        let acceptor = tls_acceptor.clone();
        let tls_stream = match acceptor.accept(tcp_stream).await {
            Ok(s) => s,
            Err(e) => {
                server::log_error!("Unable to accept TLS connection from client ({addr}). :> {e}");
                continue
            }
        };

        let websocket_stream = match tokio_tungstenite::accept_async(tls_stream).await {
            Ok(s) => s,
            Err(e) => {
                server::log_error!("Unable to accept TLS websocket connection from client ({addr}). :> {e}");
                continue
            }
        };

        let client_who = server::Who::Client(next_client_connection_num, format!("{}:{}", addr.ip(), addr.port()));
        next_client_connection_num += 1;
        server::log!("Established connection with {client_who}.");

        let conn = server::Connection::new_incoming(client_who.clone(), websocket_stream);
        let runtime_clone = Arc::clone(&runtime);
        let task = tokio::spawn(async move {
            let conn = match negotiate_client_session(conn).await {
                Err(e) => {
                    server::log_error!("{e}");
                    return
                },
                Ok(conn) => {
                    server::log!("Negotiated session with {}", conn.who);
                    conn
                }
            };

            match client_stream_task(conn, runtime_clone).await {
                Ok(who) => {
                    server::log!("Session finished with {who}");
                    return
                },
                Err(e) => {
                    server::log_error!("{e}");
                    return
                }
            }
        });

        client_websocket_stream_tasks.push((client_who, task));
    }
}

async fn negotiate_world_session(mut conn: server::Connection) -> server::ConnectionResult {
    // protocol verification: 1. the connector sends its protocol header
    let msg = ProtocolHeader::current(Protocol::ZoneToWorld);
    conn.send(msg).await?;

    // protocol verification: 2. server sends the expected corresponding protocol header or Protocol::Unsupported
    let their_protocol_header: ProtocolHeader = conn.receive().await?;
    if !their_protocol_header.compatible(Protocol::WorldToZone) {
        // either the protocol is Unsupported or the version is wrong
        return Err(conn.error_payload("compatible protocol").await);
    }
     
    // send a connection request
    let msg = ZoneToWorldMessage::Connect;
    conn.send(msg).await?;

    // receive a connection response
    let msg: WorldToZoneMessage = conn.receive().await?;
    match msg {
        WorldToZoneMessage::Connected => {
            server::log!("Connection negotiated with {}.", conn.who);
            Ok(conn)
        },
        WorldToZoneMessage::ConnectRejected => {
            server::log_error!("Connection negotiation rejected by {}.", conn.who);
            conn.halt().await;
            Err(server::NetworkError::Rejected{who: conn.who.clone()})
        },
        _ => Err(conn.error_payload("WorldToZoneMessage::[Connected, ConnectRejected]").await)
    }
}

async fn world_stream_task(mut conn: server::Connection, runtime: ZoneRuntimeSync) -> server::StreamResult {
    loop {
        let msg: WorldToZoneMessage = conn.receive().await?;
        match msg {
            WorldToZoneMessage::WorldBytes(timeframe, bytes) => {
                let frame = timeframe.frame();
                {
                    let mut runtime_lock = runtime.lock().await;
                    runtime_lock.sync_world(bytes).unwrap(); //todo: Don't Panic
                    runtime_lock.sync_timeframe(timeframe);
                }

                server::log!("Synchronized world at frame {frame}.");
            },
            WorldToZoneMessage::TimeFrame(newtimeframe) => {
                let timeframe = newtimeframe.timeframe;
                let frame = timeframe.frame();
                {
                    let mut runtime_lock = runtime.lock().await;
                    runtime_lock.sync_timeframe(timeframe);
                };

                server::log!("Frame: {frame}");
            },
            _ => {
                server::log!("Received message ::: {:?}", msg);
            }
        }
    }
}

async fn negotiate_client_session(mut conn: server::Connection) -> server::ConnectionResult {
    // protocol verification: connector sends their protocol header to us
    let their_protocol_header: ProtocolHeader = conn.receive().await?;
    // send our protocol header regardless
    let our_protocol_header = ProtocolHeader::current(Protocol::ZoneToClient);
    conn.send(our_protocol_header).await?;

    if !their_protocol_header.compatible(Protocol::ClientToZone) {
        return Err(conn.error_payload("compatible protocol").await);
    }

    let msg: ClientToZoneMessage = conn.receive().await?; 
    match msg {
        ClientToZoneMessage::Connect => {
            let msg = ZoneToClientMessage::Connected;
            conn.send(msg).await?;
            Ok(conn)
        },
        _ => {
            Err(conn.error_payload("ClientToZoneMessage::Connect").await)
        }
    }
}

async fn client_stream_task(mut conn: server::Connection, _runtime: ZoneRuntimeSync) -> server::StreamResult {
    loop {
        tokio::select! {
            result = conn.receive::<ClientToZoneMessage>() => {
                let msg = result?;
                match msg {
                    _ => {
                        server::log!("Message received {:?}", msg)
                    }
                }
            }
        }
    }
}

