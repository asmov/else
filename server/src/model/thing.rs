use crate::model::{error::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*};

#[derive(Debug)]
pub enum Thing {
    Character (Character),
    Item (Item)
}

impl Identifiable for Thing {
    fn identity(&self) -> &Identity {
        match self {
            Thing::Character(t) => t.identity(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl IdentifiableMut for Thing {
    fn identity_mut(&mut self) -> &mut Identity {
        match self {
            Thing::Character(t) => t.identity_mut(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl Descriptive for Thing {
    fn descriptor(&self) -> &Descriptor {
        match self {
            Thing::Character(t) => t.descriptor(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl DescriptiveMut for Thing {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        match self {
            Thing::Character(t) => t.descriptor_mut(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl Exists for Thing {
    fn entity(&self) -> &Entity {
        match self {
            Thing::Character(t) => t.entity(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl ExistsMut for Thing {
    fn entity_mut(&mut self) -> &mut Entity {
        match self {
            Thing::Character(t) => t.entity_mut(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl Something for Thing {}

pub trait BuildableThing: Builder + BuildableEntity {
    fn create_thing(self) -> Result<Thing>;
    fn modify_thing(self, original: &mut Self::Type) -> Result<ModifyResult>; 
    fn thing_builder(self) -> ThingBuilder;
}

#[derive(Debug)]
pub enum ThingBuilder {
    Character(CharacterBuilder),
}

impl Builder for ThingBuilder {
    type Type = Thing;

    fn creator() -> Self {
        panic!("Cannot call ThingBuilder::creator() directly")
    }

    fn editor() -> Self {
        panic!("Cannot call ThingBuilder::editor() directly")
    }

    fn builder_mode(&self) -> BuilderMode {
        match self {
            ThingBuilder::Character(b) => b.builder_mode(),
        }
    }

    fn create(self) -> Result<Self::Type> {
        match self {
            ThingBuilder::Character(b) => b.create_thing()
        }
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        match self {
            ThingBuilder::Character(b) => {
                if let Thing::Character(t) = original {
                    b.modify_thing(t)
                } else {
                    panic!("Dispatch type mismatch in ThingBuilder::modify for Character")
                }
            }
        }
    }
}

impl BuildableEntity for ThingBuilder {
    fn entity(&mut self, entity: EntityBuilder) -> Result<()> {
        match self {
            ThingBuilder::Character(b) => b.entity(entity)
        }
    }

    fn entity_builder(&mut self) -> &mut EntityBuilder {
        match self {
            ThingBuilder::Character(b) => b.entity_builder()
        }
    }
}

pub trait BuildableThingVector {
    fn add_thing(&mut self, thing: ThingBuilder) -> Result<()>;
}