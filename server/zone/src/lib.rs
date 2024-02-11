use std::fs;
use native_tls as tls;
use elsezone_model as model;
use elsezone_server_common as server;

pub struct ZoneRuntime {
    world: Option<model::World>,
    timeframe: Option<model::TimeFrame>
}

impl ZoneRuntime {
    pub fn new() -> Self {
        Self {
            world: None,
            timeframe: None
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
        let world: model::World = bincode::deserialize(&bytes)
            .map_err(|_| ())?;
        self.world = Some(world);
        Ok(self.world.as_ref().unwrap())
    }

    pub fn sync_timeframe(&mut self, timeframe: model::TimeFrame) {
        self.timeframe = Some(timeframe);
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



