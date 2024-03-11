use std::fs;
use native_tls as tls;
use asmov_else_model as model;
use asmov_else_server_common as server;
use model::{area, identity, route, BuildableDescriptor, BuildableUID, BuildableOccupantList, BuildableRouteUIDList, Builder, Built, CloneBuilding, Descriptive, DomainSynchronizer, Exists, Identifiable, RouteBuilder};

pub struct ClientSession {
    interface_view: model::InterfaceView,
}

impl ClientSession {
    pub fn interface_view(&self) -> &model::InterfaceView {
        &self.interface_view
    }

    pub fn todo_from_universe_server(world: &model::World) -> model::Result<Self> {
        let mut interface_view_creator = model::InterfaceViewBuilder::creator();

        interface_view_creator.interface(model::testing::interface_from_universe())?;

        interface_view_creator.world_view({
            let mut world_view_creator = model::WorldViewBuilder::creator();
            world_view_creator.uid(world.uid())?;
            world_view_creator.frame(world.frame())?;

            let backyard_area = world.find_area(model::testing::BACKYARD)?; 

            world_view_creator.area_view({
                let mut area_view_creator = model::AreaViewBuilder::creator();
                area_view_creator.uid(backyard_area.uid())?;
                area_view_creator.descriptor(model::DescriptorBuilder::clone_model(model::BuilderMode::Creator, backyard_area.descriptor()))?;
                
                for occupant_uid in backyard_area.occupant_uids() {
                    area_view_creator.add_occupant_uid(*occupant_uid)?;
                }

                for route_uid in backyard_area.route_uids() {
                    area_view_creator.add_route_uid(*route_uid)?;
                }

                area_view_creator
            })?;

            for route_uid in backyard_area.route_uids() {
                let route = world.route(*route_uid)?;
                let route_builder = RouteBuilder::clone_model(model::BuilderMode::Creator, &route);
                world_view_creator.add_route(route_builder)?;
            } 

            for thing_uid in backyard_area.occupant_uids() {
                let thing = world.thing(*thing_uid)?;
                let mut thing_view_creator = model::ThingViewBuilder::creator();
                thing_view_creator.uid(thing.uid())?;
                thing_view_creator.descriptor(model::DescriptorBuilder::clone_model(model::BuilderMode::Creator, thing.entity().descriptor()))?;
                world_view_creator.add_thing_view(thing_view_creator)?;
            } 

            world_view_creator
        })?;

        interface_view_creator.downlink_uid({
            world
                .find_thing(model::testing::HOUSEKEEPER).unwrap()
                .uid()
        })?;

        let (_, interface_view) = interface_view_creator.create()?.split();

        Ok(Self {
            interface_view
        })
    }
}

pub struct ZoneRuntime {
    world: Option<model::World>,
    timeframe: Option<model::TimeFrame>,
    timeframe_channel_tx: tokio::sync::watch::Sender<model::TimeFrame>,
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

    pub fn sync_universe(&mut self, bytes: Vec<u8>) -> Result<&model::World, ()> {
        todo!("sync universe")
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



