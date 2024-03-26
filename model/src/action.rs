use serde;
pub mod go;
pub mod multiply;
pub use go::*;
pub use multiply::*;

/// Actions are simple immutable structs that represent an attempt by a Thing to mutate the World.
/// Only one action can be taken per Thing per Frame.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Action {
    // testing only
    Multiply(MultiplyAction),
    /// Move from one area to another.
    Go(GoAction)
}

/// As only one action is allowed per Thing per Frame, this acts as a unique identifier for actions.
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct BaseAction {
    pub frame: Frame,
    pub thing_uid: UID
}

pub trait BasicAction {
    fn base(&self) -> &BaseAction;

    fn thing_uid(&self) -> UID {
        self.base().thing_uid
    }

    fn frame(&self) -> Frame {
        self.base().frame
    }
}
