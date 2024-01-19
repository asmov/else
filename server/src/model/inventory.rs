use crate::model::{identity::*, descriptor::*};

#[derive(Debug)]
pub struct Inventory {
    identity: Identity,
    slots: Vec<InventorySlot>
}

#[derive(Debug)]
pub struct InventorySlot {
    identity: Identity,
    item_identity: Option<Identity>
}

