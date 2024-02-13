use crate::{error::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    //fn create_thing(self) -> Result<Creation<ThingBuilder>>;
    //fn modify_thing(self, original: &mut Self::ModelType) -> Result<Modification<ThingBuilder>>; 
    fn thing_builder(self) -> ThingBuilder;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ThingBuilder {
    Character(CharacterBuilder),
}

impl Builder for ThingBuilder {
    type ModelType = Thing;
    type BuilderType = Self;

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

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        match self {
            ThingBuilder::Character(b) => b.create()
        }
    }

    fn modify(self, original: &mut Thing) -> Result<Modification<Self::BuilderType>> {
        match self {
            ThingBuilder::Character(character_builder) => {
                if let Thing::Character(character) = original {
                    character_builder.modify(character)
                } else {
                    unreachable!("Dispatch type mismatch in ThingBuilder::modify for Character")
                }
            }
        }
    }
}

impl BuildableEntity for ThingBuilder {
    fn entity(&mut self, entity_builder: EntityBuilder) -> Result<()> {
        match self {
            ThingBuilder::Character(character_builder) => character_builder.entity(entity_builder)
        }
    }

    fn entity_builder(&mut self) -> &mut EntityBuilder {
        match self {
            ThingBuilder::Character(character_builder) => character_builder.entity_builder()
        }
    }
}

impl BuildableIdentity for ThingBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        match self {
            ThingBuilder::Character(character_builder) => character_builder.entity_builder().identity(identity)
        }
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        match self {
            ThingBuilder::Character(character_builder) => character_builder.entity_builder().identity_builder()
        }
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        todo!()/*match self {
            ThingBuilder::Character(character_builder) => character_builder.entity_builder().get_identity()
        }*/
    }
}

pub trait BuildableThingVector {
    fn add_thing(&mut self, thing: ThingBuilder) -> Result<()>;
}