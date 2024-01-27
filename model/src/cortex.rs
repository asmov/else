pub use crate::{s, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, interface::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Cortex {
    Routine(RoutineCortex),
    Interface(InterfaceCortex)
}

pub type RoutineID = u8;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RoutineCortex {
    routine_id: RoutineID,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct InterfaceCortex {
    interface_id: InterfaceID
}

pub trait CortexInterface {
    /// Inbound stimulus with a possible reaction
    pub fn stimulate() -> Option<()>;
    /// Time stimulus with a possible reaction
    pub fn think(timeframe: Timeframe) -> Option<()>;
}