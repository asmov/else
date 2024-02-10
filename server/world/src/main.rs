
use tokio_tungstenite;
use elsezone_model::message::*;
use elsezone_network_common as elsenet;
use tokio_native_tls;
use anyhow;
use elsezone_server_common as server;
use elsezone_behavior as behavior;

#[tokio::main]
async fn main() {
    let identity_password = String::from("mypass"); //todo
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_WORLD_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");

    let mut runtime = behavior::WorldRuntime::load().unwrap();
    
    let tls_acceptor = server::build_tls_acceptor(identity_password);
    let zone_tcp_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    
    server::log!("Listening for zone server connections on {bind_address}.");
    let _listener_task = tokio::spawn(listener_task(zone_tcp_listener, tls_acceptor));

    let sleep = tokio::time::sleep(runtime.frame_duration());
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                runtime.tick().unwrap();
                server::log!("Frame: {}", runtime.timeframe().frame());
                sleep.as_mut().reset(tokio::time::Instant::now() + runtime.frame_duration());
            }
        }
    }
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

        let conn = server::Connection::new_incoming(zone_who.clone(), websocket_stream);
        let task = tokio::spawn(async move {
            let conn = match negotiate_zone_session(conn).await {
                Err(e) => {
                    server::log_error!("{e}");
                    return Err(());
                },
                Ok(conn) => conn
            };

            server::log!("Negotiated session with {}", conn.who);

            match zone_stream_task(conn).await {
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
    conn.send_binary(our_protocol_header).await?;

    // If their header isn't compatible, disconnect
    if !their_protocol_header.compatible(Protocol::ZoneToWorld) {
        return Err(conn.error_payload("compatible protocol").await);
    }

    let msg: ZoneToWorldMessage = conn.receive().await?;
    match msg {
        ZoneToWorldMessage::Connect => {
            let response = WorldToZoneMessage::Connected;
            conn.send_zone(response).await?;
            Ok(conn)
        },
        _ => Err(conn.error_payload("ZoneToWorldMessage::Connect").await)
    }
}

async fn zone_stream_task(mut conn: server::Connection) -> Result<server::Who, server::NetworkError> {
    conn.halt().await;
    Ok(conn.who.to_owned())
}