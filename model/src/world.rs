pub mod builder;
pub mod action;
use serde;
use crate::{area::*, modeling::*, character::*, route::*, timeframe::*, interface::*};
pub use builder::*;
pub use action::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct World {
    uid: UID,
    frame: Frame,
    descriptor: Descriptor,
    areas: Vec<Area>,
    routes: Vec<Route>,
    things: Vec<Thing>,
    interfaces: Vec<Interface>,
    next_id: ID,
}

impl Keyed for World {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for World {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for World {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for World {
    type BuilderType = WorldBuilder;
}

impl World {
    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn areas(&self) -> &Vec<Area> {
        &self.areas
    }

    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }

    pub fn things(&self) -> &Vec<Thing> {
        &self.things
    }

    pub fn interfaces(&self) -> &Vec<Interface> {
        &self.interfaces
    }

    pub fn area(&self, uid: UID) -> Result<&Area> {
        self.areas.iter().find(|area| area.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: "Area", uid})
    }

    pub fn route(&self, uid: UID) -> Result<&Route> {
        self.routes.iter().find(|route| route.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: RouteField::classname(), uid})
    }

    pub fn thing(&self, uid: UID) -> Result<&Thing> {
        self.things.iter().find(|thing| thing.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: Thing::class_ident_const().classname(), uid})
    }

    pub fn interface(&self, uid: UID) -> Result<&Interface> {
        self.interfaces.iter().find(|interface| interface.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: InterfaceField::classname(), uid})
    }

    pub fn find_area(&self, key: &str) -> Result<&Area> {
        self.areas.iter().find(|area| area.key().is_some_and(|k| k == key))
            .ok_or_else(|| Error::ModelKeyNotFound { model: AreaField::classname(), key: key.to_string() })
    }

    pub fn find_route(&self, key: &str) -> Result<&Route> {
        self.routes.iter().find(|route| route.key().is_some_and(|k| k == key))
            .ok_or_else(|| Error::ModelKeyNotFound { model: RouteField::classname(), key: key.to_string() })
    }

    pub fn find_thing(&self, key: &str) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.key().is_some_and(|k| k == key))
    }

    pub fn find_interface(&self, key: &str) -> Option<&Interface> {
        self.interfaces.iter().find(|interface| interface.key().is_some_and(|k| k == key))
    }

}
