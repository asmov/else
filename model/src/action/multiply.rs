use crate::action::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MultiplyAction {
    pub base: BaseAction
}

impl BasicAction for MultiplyAction {
    fn base(&self) -> &BaseAction {
        &self.base
    }
}
