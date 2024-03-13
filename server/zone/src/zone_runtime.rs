use std::fs;
use native_tls as tls;
use asmov_else_model as model;
use asmov_else_server_common as server;
use model::{area, identity, route, BuildableDescriptor, BuildableUID, BuildableOccupantList, BuildableRouteUIDList, Builder, Built, CloneBuilding, Descriptive, DomainSynchronizer, Exists, Identifiable, RouteBuilder};

pub struct ZoneRuntime {
    world: Option<model::World>,
    timeframe: Option<model::TimeFrame>,
    timeframe_channel_tx: tokio::sync::watch::Sender<model::TimeFrame>,
    //universe_channel: tokio::sync::mpsc::
}

impl ZoneRuntime {
    pub fn new() -> Self {
        Self {
            world: None,
            timeframe: None,
            timeframe_channel_tx: tokio::sync::watch::channel(model::TimeFrame::new(0,0)).0
        }
    }

    pub fn ready(&self) -> bool {
        self.world.is_some()
    }

    pub fn timeframe(&self) -> Option<&model::TimeFrame> {
        self.timeframe.as_ref()
    }

    pub fn world(&self) -> Option<&model::World> {
        self.world.as_ref()
    }

    pub fn sync_world(&mut self, bytes: Vec<u8>) -> Result<&model::World, ()> {
        let world: model::World = bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
            .map_err(|_| ())?
            .0;
        self.world = Some(world);
        Ok(self.world.as_ref().unwrap())
    }

    pub fn sync(&mut self, sync: model::Sync) -> model::Result<()> {
        sync.sync(self.world.as_mut().unwrap())?;
        Ok(())
    }

    pub fn sync_timeframe(&mut self, timeframe: model::TimeFrame) {
        self.timeframe = Some(timeframe);
        let _ = self.timeframe_channel_tx.send(self.timeframe.as_ref().unwrap().clone());
    }

    pub fn subscribe_timeframe(&mut self) -> tokio::sync::watch::Receiver<model::TimeFrame> {
        self.timeframe_channel_tx.subscribe()
    }

    pub fn subscribe_universe_stream(&self) -> tokio::sync::watch::Receiver<model::UniverseStream> {
        let (tx, rx) = tokio::sync::watch::channel(model::UniverseStream::new());
        tx
    }
}

pub type ZoneRuntimeSync = std::sync::Arc<tokio::sync::Mutex<ZoneRuntime>>;

pub fn load_certs() -> Vec<tls::Certificate> {
    const FILENAMES: [&'static str; 2] = ["cert.der", "root-ca.der"];
    let certs_dir = server::certs_dir();
    let mut certs = Vec::new();

    for filename in FILENAMES {
        let bytes = &fs::read(certs_dir.join(filename)).unwrap();
        let cert = tls::Certificate::from_der(bytes).unwrap();
        certs.push(cert);
    }

    certs
}

pub fn build_tls_connector() -> tokio_tungstenite::Connector {
    let mut native_tls_connector_builder = tls::TlsConnector::builder();

    #[cfg(debug_assertions)]
    native_tls_connector_builder.danger_accept_invalid_hostnames(true);

    for cert in load_certs() {
        native_tls_connector_builder.add_root_certificate(cert);
    }
    
    let native_tls_connector = native_tls_connector_builder.build().unwrap();
    tokio_tungstenite::Connector::NativeTls(native_tls_connector)
}



