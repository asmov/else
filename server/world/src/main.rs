use std::sync::Arc;
use tokio;
use tokio_native_tls;
use elsezone_model::message::*;
use elsezone_network_common as elsenet;
use elsezone_server_common as server;

#[tokio::main]
async fn main() {
    let identity_password = String::from("mypass"); //todo
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_WORLD_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");

    let runtime = server::load_runtime().unwrap();
    
    let tls_acceptor = server::build_tls_acceptor(identity_password);
    let zone_tcp_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    
    server::log!("Listening for zone server connections on {bind_address}.");
    let _listener_task = tokio::spawn(listener_task(zone_tcp_listener, tls_acceptor, Arc::clone(&runtime)));

    let sleep = {
        let runtime_lock = runtime.lock().await;
        tokio::time::sleep(runtime_lock.frame_duration())
    };
        
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                let mut runtime_lock = runtime.lock().await;
                runtime_lock.tick().unwrap();
                server::log!("Frame: {}", runtime_lock.timeframe().frame());
                sleep.as_mut().reset(tokio::time::Instant::now() + runtime_lock.frame_duration());
            }
        }
    }
}

async fn listener_task(
    zone_tcp_listener: tokio::net::TcpListener,
    tls_acceptor: tokio_native_tls::TlsAcceptor,
    runtime: server::WorldRuntimeSync
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

        let conn = server::Connection::new_incoming(zone_who.clone(), websocket_stream);
        let runtime_clone = Arc::clone(&runtime);
        let task = tokio::spawn(async move {
            let conn = match negotiate_zone_session(conn).await {
                Err(e) => {
                    server::log_error!("{e}");
                    return Err(());
                },
                Ok(conn) => conn
            };

            server::log!("Negotiated session with {}", conn.who);

            match zone_stream_task(conn, runtime_clone).await {
                Err(e) => {
                    server::log_error!("{e}");
                    Err(())
                },
                Ok(who) => {
                    server::log!("Session has ended with {who}.");
                    Ok(())
                }
            }
        });

        zone_stream_tasks.push((zone_who, task));
    }

    Ok(())
}

async fn negotiate_zone_session(mut conn: server::Connection) -> server::ConnectionResult {
    // Receive a protocol header from the connecting socket
    let their_protocol_header: ProtocolHeader = conn.receive().await?;

    // Send our protocol header regardless
    let our_protocol_header = ProtocolHeader::current(Protocol::WorldToZone);
    conn.send(our_protocol_header).await?;

    // If their header isn't compatible, disconnect
    if !their_protocol_header.compatible(Protocol::ZoneToWorld) {
        return Err(conn.error_payload("compatible protocol").await);
    }

    let msg: ZoneToWorldMessage = conn.receive().await?;
    match msg {
        ZoneToWorldMessage::Connect => {
            let response = WorldToZoneMessage::Connected;
            conn.send(response).await?;
        },
        _ => return Err(conn.error_payload("ZoneToWorldMessage::Connect").await)
    }

    Ok(conn)
}

async fn zone_stream_task(
    mut conn: server::Connection,
    runtime: server::WorldRuntimeSync
) -> Result<server::Who, server::NetworkError> {

    let world_bytes = {
        let runtime_lock = runtime.lock().await;
        bincode::serialize(runtime_lock.world()).unwrap()
    };

    let msg = WorldToZoneMessage::WorldBytes(world_bytes);
    conn.send(msg).await?;

    while let Ok(message) = conn.receive::<ZoneToWorldMessage>().await {
        server::log!("Received a message! {:?}", message);
    }

    Ok(conn.who.to_owned())
}