use crate::*;
use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Sync {
    Init,
    World(Operation<WorldBuilder>),
    InterfaceView(Operation<InterfaceViewBuilder>),
}

impl DomainSynchronizer<InterfaceView> for Sync {
    fn sync(self, interface_view: &mut InterfaceView) -> Result<Self> {
        Ok(match self {
            Sync::InterfaceView(Operation::Modification(modification)) => {
                Sync::InterfaceView(Operation::Modification(
                    modification
                        .take_builder()
                        .synchronize(interface_view)?
                ))
            },
            _ => unimplemented!("Handler within DomainSynchronizer<InterfaceView> does not exist for Sync: {:?}", self)
        })
    }
}

impl DomainSynchronizer<World> for Sync {
    fn sync(self, world: &mut World) -> Result<Self> {
        Ok(match self {
            Sync::World(Operation::Modification(modification)) => {
                Sync::World(Operation::Modification(
                    modification
                        .take_builder()
                        .synchronize(world)?
                ))
            },
            _ => unimplemented!("Handler within DomainSynchronizer<World> does not exist for Sync: {:?}", self)
        })
    }
}