use crate::{error::*, location::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*, world::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Thing {
    Character (Character),
    Item (Item),
}

impl Keyed for Thing {
    fn key(&self) -> Option<&str> {
        self.descriptor().key()
    }
}

impl Identifiable for Thing {
    fn uid(&self) -> UID {
        match self {
            Thing::Character(t) => t.uid(),
            Thing::Item(_) => todo!(),
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

impl Located for Thing {
    fn location(&self) -> Location {
        match self {
            Thing::Character(t) => t.location(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl Exists for Thing {
    fn entity(&self) -> &Entity {
        match self {
            Thing::Character(character) => character.entity(),
            Thing::Item(_item) => todo!(),
        }
    }
}

impl Built for Thing {
    type BuilderType = ThingBuilder;

    fn edit_self(&self) -> Self::BuilderType
        where
            Self: Identifiable,
            Self::BuilderType: BuildableIdentity {
        match self {
            Thing::Character(character) => character.edit_self().thing_builder(),
            Thing::Item(_item) => todo!(),
        } 
    }
}

impl Something for Thing {}

pub trait ThingBuilderVariant: Builder + BuildableEntity {
    fn thing_builder(self) -> ThingBuilder;
}

impl Thing {
    const CLASSNAME: &'static str = "Thing";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Thing as ClassID, Self::CLASSNAME);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ThingBuilder {
    Character(CharacterBuilder),
}

pub enum ThingBuilderRef<'thing> {
    Character(&'thing CharacterBuilder),
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

    fn modify(self, existing: &mut Thing) -> Result<Modification<Self::BuilderType>> {
        match self {
            ThingBuilder::Character(character_builder) => {
                if let Thing::Character(character) = existing {
                    character_builder.modify(character)
                } else {
                    unreachable!("Dispatch type mismatch in ThingBuilder::modify for Character")
                }
            }
        }
    }

    fn class_ident(&self) -> &'static ClassIdent {
        match self {
            Self::Character(modeler) => modeler.class_ident(),
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
    
    fn get_entity(&self) -> Option<&EntityBuilder> {
       match self {
            ThingBuilder::Character(character_builder) => character_builder.get_entity()
        } 
    }
}

impl MaybeIdentifiable for ThingBuilder {
    fn try_uid(&self) -> Result<UID> {
        self.get_identity()
            .ok_or_else(|| Error::IdentityNotGenerated)?
            .get_uid()
            
    }
}

impl BuildableIdentity for ThingBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        self.entity_builder().identity(identity)
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        self.entity_builder().identity_builder()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.get_entity()
            .and_then(|entity_builder| entity_builder.get_identity())
    }
}

pub trait BuildableThingList {
    fn add_thing(&mut self, thing: ThingBuilder) -> Result<()>;
    fn edit_thing(&mut self, thing: ThingBuilder) -> Result<()>;
    fn remove_thing(&mut self, thing_uid: UID) -> Result<()>;
}

pub trait BuildableOccupantList {
    fn add_occupant_uid(&mut self, thing_uid: UID) -> Result<()>;
    fn remove_occupant_uid(&mut self, thing_uid: UID) -> Result<()>;
}