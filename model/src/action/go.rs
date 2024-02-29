pub use crate::{identity::*, timeframe::*, action::*};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GoAction {
    pub base: BaseAction,
    pub origin_area_id: UID,
    pub route_uid: UID
}

impl BasicAction for GoAction {
    fn base(&self) -> &BaseAction {
        &self.base
    }
}

