use crate::*;
use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Operation<B>
where
    B: Builder<BuilderType = B>,
    B::ModelType: std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize
{
    Creation(Creation<B>),
    Modification(Modification<B>),
    //Deletion(Deletion<B>)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Sync {
    Area(Operation<AreaBuilder>),
    Thing(Operation<ThingBuilder>)
}

impl Sync {
    pub fn sync(self, world: &mut World) -> Result<Sync> {
        Ok(match self {
            Sync::Area(Operation::Modification(modification)) => {
                Sync::Area(Operation::Modification(
                    modification
                        .take_builder()
                        .sync_modify(world)?
                ))
            },
            _ => todo!()
        })
    }
}