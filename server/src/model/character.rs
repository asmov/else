pub use crate::{s, model::{error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*}};

#[derive(Debug)]
pub struct Character {
    entity: Entity,
}

#[derive(Debug)]
pub enum CharacterField {
    Entity,
}

impl CharacterField {
    pub const CLASSNAME: &'static str = "Character";
    pub const FIELDNAME_ENTITY: &'static str = "entity";

    pub const FIELD_ENTITY: Field = Field::new(Self::FIELDNAME_ENTITY, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Entity => &Self::FIELD_ENTITY,
        }
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    builder_mode: BuilderMode,
    entity: Option<EntityBuilder>
}

impl Builder for CharacterBuilder {
    type Type = Character;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            entity: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator()
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn create(self) -> Result<Character> {
        Ok(Character {
            entity: self.entity
                .ok_or_else(||
                    Error::FieldNotSet{class: CharacterField::CLASSNAME, field: CharacterField::FIELDNAME_ENTITY})?
                .create()?
        })
    }

    fn modify(self, _original: &mut Self::Type) -> Result<ModifyResult> {
        Ok(ModifyResult::new(Vec::new()))
    }
}

impl Build for Character {
    type BuilderType = CharacterBuilder;
}

impl Identifiable for Character {
    fn identity(&self) -> &Identity {
        self.entity().identity()
    }
}

impl IdentifiableMut for Character {
    fn identity_mut(&mut self) -> &mut Identity {
        self.entity_mut().identity_mut()
    }
}

impl Descriptive for Character {
    fn descriptor(&self) -> &Descriptor{
        self.entity().descriptor()
    }
}

impl DescriptiveMut for Character {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        self.entity_mut().descriptor_mut()
    }
}

impl Exists for Character {
    fn entity(&self) -> &Entity {
        &self.entity
    }
}

impl ExistsMut for Character {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl BuildableEntity for CharacterBuilder {
    fn entity(&mut self, entity: EntityBuilder) -> Result<()> {
        self.entity = Some(entity);
        Ok(())
    }

    fn entity_builder(&mut self) -> &mut EntityBuilder {
        if self.entity.is_none() {
            self.entity = Some(Entity::builder(self.builder_mode()))
        }

        self.entity.as_mut().unwrap()
    }
}

impl BuildableThing for CharacterBuilder {
    fn create_thing(self) -> Result<Thing> {
        Ok(Thing::Character(self.create()?))
    }

    fn modify_thing(self, original: &mut Self::Type) -> Result<ModifyResult> {
        Ok(self.modify(original)?)
    }

    fn thing_builder(self) -> ThingBuilder {
        ThingBuilder::Character(self)
    }
}

impl Something for Character {}