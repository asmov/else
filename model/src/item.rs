use crate::entity::*;
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Item {
    entity: Entity
}

