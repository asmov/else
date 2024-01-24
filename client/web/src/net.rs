use elsenet::WEBSOCKET_PAYLOAD_ERROR;
use reqwasm::websocket::Message;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use gloo_console::log;
use elsezone_model as model;
use bincode;
use elsezone_network as elsenet;



pub async fn connect() -> Result<(), ()> {
    let mut websocket = WebSocket::open(elsenet::ELSE_LOCALHOST_WORLD_URL).unwrap();
    //let mut websocket = WebSocket::open(elsenet::ELSE_LOCALHOST_ZONE_URL).unwrap();
    //let (mut tx, mut rcv) = websocket.split();

    // Send the protocol header
    let msg = model::ProtocolHeader::current(model::Protocol::ClientToZone);
    match websocket.send(Message::Bytes(bincode::serialize(&msg).unwrap())).await {
        Ok(_) => {},
        Err(_) => { 
            let _ = websocket.close(None, None);
            return Err(());
        }
    }

    // The server should send a protocol header back
    if let Some(msg) = websocket.next().await {
        match msg {
            Ok(Message::Bytes(bytes)) => {
                let protocol_header: model::ProtocolHeader = match bincode::deserialize(&bytes) {
                    Ok(p) => p,
                    Err(_) => {
                        let _ = websocket.close(None, None);
                        return Err(());
                    },
                };

                if !protocol_header.compatible(model::Protocol::ZoneToClient) {
                    log!(format!("Incompatible protocol: {:?}", protocol_header));
                    let _ = websocket.close(None, None);
                    return Err(());
                }
            }
            _ => {
                let _ = websocket.close(None, None);
                return Err(());
            }
        }
    } else {
        let _ = websocket.close(None, None);
        return Err(());
    }

    // Send the connection request
    let msg = model::ClientToZoneMessage::Connect;
    match websocket.send(Message::Bytes(bincode::serialize(&msg).unwrap())).await {
        Ok(_) => {},
        Err(_) => { 
            let _ = websocket.close(Some(WEBSOCKET_PAYLOAD_ERROR), Some("Expected ZoneToClientMessage"));
            return Err(());
        }
    }

    if let Some(msg) = websocket.next().await {
        match msg {
            Ok(Message::Bytes(bytes)) => {
                let msg: model::ZoneToClientMessage = match bincode::deserialize(&bytes) {
                    Ok(msg) => msg,
                    Err(_) => {
                        let _ = websocket.close(Some(WEBSOCKET_PAYLOAD_ERROR), Some("Expected ZoneToClientMessage"));
                        return Err(());
                    }
                };

                match msg {
                    model::ZoneToClientMessage::Connected => {
                        log!("CONNECTION ACCEPTED");
                    },
                    model::ZoneToClientMessage::ConnectRejected => {
                        log!("CONNECTION REJECTED");
                        return Err(())
                    },
                    _ => {
                        let _ = websocket.close(Some(WEBSOCKET_PAYLOAD_ERROR), Some("Expected ZoneToClientMessage"));
                        return Err(());
                    }
                }
            },
            _ => {
                let _ = websocket.close(Some(WEBSOCKET_PAYLOAD_ERROR), Some("Expected ZoneToClientMessage"));
                return Err(())
            }
        }
    } else {
        let _ = websocket.close(None, None);
    }

    Ok(())
}
