use crate::{error::*, modeling::*, location::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*};
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
            Self::BuilderType: BuildableUID {
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

    pub fn try_character(&self) -> Result<&Character> {
        match self {
            Thing::Character(character) => Ok(character),
            Thing::Item(_) => Err(Error::UnexpectedModelType { expected: CharacterField::classname(), found: "Item" }),
        }
    }
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
        Self::_try_uid(&self)
    }
}

impl BuildableUID for ThingBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.entity_builder().uid(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.get_entity().and_then(|e| e.get_uid())
    }
}
    

pub trait BuildableThingList {
    fn thing_ops(&mut self) -> &mut Vec<ListOp<ThingBuilder, UID>>;
    fn get_thing_ops(&self) -> &Vec<ListOp<ThingBuilder, UID>>;

    fn add_thing(&mut self, thing: ThingBuilder) -> Result<&mut Self> {
       self.thing_ops().push(ListOp::Add(thing)); 
       Ok(self)
    }

    fn edit_thing(&mut self, thing: ThingBuilder) -> Result<&mut Self> {
        self.thing_ops().push(ListOp::Edit(thing));
        Ok(self)
    }

    fn remove_thing(&mut self, thing_uid: UID) -> Result<&mut Self> {
        self.thing_ops().push(ListOp::Remove(thing_uid));
        Ok(self)
    }
}

pub trait BuildableOccupantList {
    fn add_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self>;
    fn remove_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self>;
}