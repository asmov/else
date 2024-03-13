use std::{process, sync::Arc};
use tokio;
use tokio_native_tls;
use asmov_else_model::{self as model, message::*};
use asmov_else_network_common as elsenet;
use asmov_else_server_common as server;
use server::ConnectionTrait;
use crate::{universe_runtime::*, universe_service::*};

pub type UniverseRuntimeSync = std::sync::Arc<tokio::sync::Mutex<UniverseRuntime>>;

pub(crate) async fn run() -> process::ExitCode {
    let identity_password = String::from("mypass"); //todo
    let bind_ip = elsenet::LOCALHOST_IP;
    let bind_port = elsenet::ELSE_UNIVERSE_PORT;
    let bind_address = format!("{bind_ip}:{bind_port}");

    let runtime = Arc::new(tokio::sync::Mutex::new(UniverseRuntime::load().unwrap()));
    
    let tls_acceptor = server::build_tls_acceptor(identity_password);
    let zone_tcp_listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    
    server::log!("Listening for zone server connections on {bind_address}.");
    let _listener_task = tokio::spawn(zone_listener_task(zone_tcp_listener, tls_acceptor, Arc::clone(&runtime)));

    let sleep = {
        let runtime_lock = runtime.lock().await;
        tokio::time::sleep(runtime_lock.tick_interval())
    };
        
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                let tick_interval = {
                    let mut runtime_lock = runtime.lock().await;
                    runtime_lock.tick().await.unwrap();
                    runtime_lock.tick_interval()
                };

                sleep.as_mut().reset(tokio::time::Instant::now() + tick_interval);
            }
        }
    }
}

async fn zone_listener_task(
    zone_tcp_listener: tokio::net::TcpListener,
    tls_acceptor: tokio_native_tls::TlsAcceptor,
    runtime: UniverseRuntimeSync
) {
    let mut next_connection_id: usize = 1;
    let mut zone_stream_tasks = Vec::new();

    loop {
        let (tcp_stream, addr) = match zone_tcp_listener.accept().await {
            Ok(r) => r,
            Err(e) => {
                server::log_error!("Unable to accept connection from zone. :> {e}");
                continue
            }
        };

        let acceptor = tls_acceptor.clone();
        let tls_stream = match acceptor.accept(tcp_stream).await {
            Ok(s) => s,
            Err(e) => {
                server::log_error!("Unable to accept TLS connection from zone ({addr}). :> {e}");
                continue
            }
        };

        let websocket_stream = match tokio_tungstenite::accept_async(tls_stream).await {
            Ok(s) => s,
            Err(e) => {
                server::log_error!("Unable to accept TLS websocket connection from zone ({addr}). :> {e}");
                continue
            }
        };

        let zone_who = server::Who::Zone(next_connection_id, format!("{}:{}", addr.ip(), addr.port()));
        next_connection_id += 1;
        server::log!("Established connection with {zone_who}.");

        let conn = server::Connection::new(zone_who.clone(), server::Stream::Incoming(websocket_stream));
        let runtime_clone = Arc::clone(&runtime);
        let task = tokio::spawn(async move {
            let conn = match negotiate_zone_session(conn).await {
                Err(e) => {
                    server::log_error!("{e}");
                    return Err(());
                },
                Ok(conn) => conn
            };

            server::log!("Negotiated session with {}", conn.who());

            match zone_stream_loop(conn, runtime_clone).await {
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
}

async fn negotiate_zone_session(mut conn: server::Connection) -> server::ConnectionResult {
    elsenet::negotiate_protocol(&mut conn, true, Protocol::UniverseToZone, Protocol::ZoneToUniverse).await?;

    let msg: ZoneToUniverseMessage = conn.receive().await?;
    match msg {
        ZoneToUniverseMessage::Connect => {
            let response = UniverseToZoneMessage::Connected;
            conn.send(response).await?;
        },
        _ => return Err(conn.error_payload("ZoneToUniverseMessage::Connect").await)
    }

    Ok(conn)
}

async fn zone_stream_loop(
    mut conn: server::Connection,
    runtime: UniverseRuntimeSync
) -> Result<server::Who, server::NetworkError> {

    let (timeframe, universe_bytes) = {
        let runtime_lock = runtime.lock().await;
        (runtime_lock.timeframe().clone(), bincode::serde::encode_to_vec(runtime_lock.universe(), bincode::config::standard()).unwrap())
    };

    let msg = UniverseToZoneMessage::UniverseBytes(timeframe, universe_bytes);
    conn.send(msg).await?;

    let (mut sync_subscriber, mut timeframe_subscriber) = {
        let mut runtime_lock = runtime.lock().await;
        (runtime_lock.subscribe_sync(), runtime_lock.subscribe_timeframe())
    };

    loop {
        tokio::select! {
            result = conn.receive::<ZoneToUniverseMessage>() => {
                let message = result?;
                match message {
                    //ZoneToUniverse::A
                    ZoneToUniverseMessage::Disconnect => {
                        conn.halt().await;
                        break;
                    },
                    _ =>  {
                        server::log!("Received a message! {:?}", message);
                    },
                }
            }
            sync = sync_subscriber.recv() => {
                let sync = sync.unwrap();
                let msg = UniverseToZoneMessage::Sync(sync);
                conn.send(msg).await?;
            }
            _result = timeframe_subscriber.changed() => {
                let timeframe: model::TimeFrame = timeframe_subscriber.borrow_and_update().clone();
                let msg = UniverseToZoneMessage::TimeFrame(NewTimeFrameMsg{timeframe});
                conn.send(msg).await?;
            }
        }
    }

    Ok(conn.who().clone())
}
