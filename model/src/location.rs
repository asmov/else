use serde;
use crate::{error::*, identity::*, modeling::*, codebase::*, descriptor::*};

/// The placement of an Entity within the World; an Area or Route.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub enum Location {
    Area(UID),
    Route(UID)
}

impl Location {
    pub fn uid(&self) -> UID {
        match self {
            Self::Area(uid) => *uid,
            Self::Route(uid) => *uid
        }
    }
}

pub trait Located {
    fn location(&self) -> Location;
}