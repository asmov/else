use std::fs;
use native_tls as tls;
use elsezone_model as model;
use elsezone_server_common as server;
use model::{area, identity, route, BuildableDescriptor, BuildableIdentity, BuildableOccupantList, BuildableRouteUIDList, Builder, Built, Descriptive, DomainSynchronizer, Exists, Identifiable};

pub struct ClientSession {
    interface_view: model::InterfaceView,
}

impl ClientSession {
    pub fn interface_view(&self) -> &model::InterfaceView {
        &self.interface_view
    }

    pub fn todo_from_universe_server(world: &model::World) -> model::Result<Self> {
        dbg!(world);
        let mut interface_view_creator = model::InterfaceViewBuilder::creator();

        interface_view_creator.interface({
            let mut interface_creator = model::InterfaceBuilder::creator();
            interface_creator.identity({
                let mut identity_creator = model::IdentityBuilder::from_existing(&interface_creator, world);
                identity_creator
                    .class_id(model::CodebaseClassID::Interface as model::ClassID)?
                    .id(1)?;
                identity_creator
            })?;
            interface_creator
        })?;

        interface_view_creator.world_view({
            let mut world_view_creator = model::WorldViewBuilder::creator();
            world_view_creator.identity(model::IdentityBuilder::from_existing(&world_view_creator, world))?;
            world_view_creator.frame(world.frame())?;

            let backyard_area = world.find_area(model::testing::BACKYARD)?; 

            world_view_creator.area_view({
                let mut area_view_creator = model::AreaViewBuilder::creator();
                area_view_creator.identity(model::IdentityBuilder::from_existing(&area_view_creator, backyard_area))?;
                area_view_creator.descriptor(model::DescriptorBuilder::creator().clone(backyard_area.descriptor()))?;
                
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
                world_view_creator.add_route(route.edit_self())?;
            } 

            for thing_uid in backyard_area.occupant_uids() {
                let thing = world.thing(*thing_uid)?;
                let mut thing_view_creator = model::ThingViewBuilder::creator();
                thing_view_creator.identity(model::IdentityBuilder::from_existing(&thing_view_creator, thing))?;
                thing_view_creator.descriptor(model::DescriptorBuilder::creator().clone(thing.entity().descriptor()))?;
                world_view_creator.add_thing_view(thing_view_creator)?;
            } 

            world_view_creator
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



