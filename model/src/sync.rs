use crate::*;
use serde;

pub trait DomainSynchronizer<D>
where
    Self: Sized,
    D: Sized
{
    fn sync(self, domain: &mut D) -> Result<Self>;
}

pub trait SynchronizedDomainBuilder<D>
where
    Self: Builder,
    D: Sized
{
    //todo: rename this to synchronize
    fn synch(self, domain: &mut D) -> Result<Modification<Self::BuilderType>>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Operation<B>
where
    B: Builder<BuilderType = B>,
    B::ModelType: std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize
{
    Creation(Creation<B>),
    Modification(Modification<B>),
    //todo: Deletion(Deletion<B>)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Sync {
    Init,
    Area(Operation<AreaBuilder>),
    Thing(Operation<ThingBuilder>),
    World(Operation<WorldBuilder>)
}

impl SynchronizedDomainBuilder<InterfaceView> for AreaBuilder {
    fn synch(self, domain: &mut InterfaceView) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }
}

impl DomainSynchronizer<InterfaceView> for Sync {
    fn sync(self, interface_view: &mut InterfaceView) -> Result<Self> {
        Ok(match self {
            Sync::Area(Operation::Modification(modification)) => {
                Sync::Area(Operation::Modification(
                    modification
                        .take_builder()
                        .synch(interface_view)?
                ))
            },
            Sync::Thing(Operation::Modification(modification)) => {
                Sync::Thing(Operation::Modification(
                    modification
                        .take_builder()
                        .synch(interface_view)?
                ))
            },
            Sync::World(Operation::Modification(modification)) => {
                Sync::World(Operation::Modification(
                    modification
                        .take_builder()
                        .synch(interface_view)?
                ))
            },
            _ => todo!("todo: Missing synchronizer implementation for {:?}", self)
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
                        .synch(world)?
                ))
            },
            Sync::Area(Operation::Modification(modification)) => {
                Sync::Area(Operation::Modification(
                    modification
                        .take_builder()
                        .synch(world)?
                ))
            },
            _ => todo!("todo: Missing synchronizer implementation for {:?}", self)
        })
    }
}