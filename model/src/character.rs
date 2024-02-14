pub use crate::{s, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, cortex::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Character {
    entity: Entity,
    cortex: Cortex
}

#[derive(Debug)]
pub enum CharacterField {
    Entity,
    Cortex
}

impl CharacterField {
    pub const CLASSNAME: &'static str = "Character";
    pub const FIELDNAME_ENTITY: &'static str = "entity";
    pub const FIELDNAME_CORTEX: &'static str = "cortex";

    pub const FIELD_ENTITY: Field = Field::new(Self::FIELDNAME_ENTITY, FieldValueType::Object);
    pub const FIELD_CORTEX: Field = Field::new(Self::FIELDNAME_CORTEX, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Entity => &Self::FIELD_ENTITY,
            Self::Cortex => &Self::FIELD_CORTEX,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CharacterBuilder {
    builder_mode: BuilderMode,
    entity: Option<EntityBuilder>,
    cortex: Option<CortexBuilder>
}

impl Builder for CharacterBuilder {
    type ModelType = Character;
    type BuilderType = ThingBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            entity: None,
            cortex: None
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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let entity = Creation::try_assign(&mut self.entity, CharacterField::CLASSNAME, CharacterField::FIELDNAME_ENTITY)?;
        let cortex = Creation::try_assign(&mut self.cortex, CharacterField::CLASSNAME, CharacterField::FIELDNAME_CORTEX)?;

        let character = Character {
            entity: entity,
            cortex: cortex
        };

        Ok(Creation::new(ThingBuilder::Character(self), Thing::Character(character)))
    }

    fn modify(self, _original: &mut Self::ModelType) -> Result<Modification<ThingBuilder>> {
        Ok(Modification::new(ThingBuilder::Character(self), Vec::new()))
    }
}

impl Built for Character {
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

impl Sensitive for Character {
    fn cortex(&self) -> &Cortex {
        &self.cortex
    }
}

impl SensitiveMut for Character {
    fn cortext_mut(&mut self) -> &mut Cortex {
        &mut self.cortex
    }
}

impl BuildableCortex for CharacterBuilder {
    fn cortex(&mut self, cortex: CortexBuilder) -> Result<()> {
        self.cortex = Some(cortex);
        Ok(())
    }

    fn get_cortex_builder_mut(&mut self) -> &mut Option<CortexBuilder> {
        &mut self.cortex
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

    fn get_entity(&self) -> Option<&EntityBuilder> {
        self.entity.as_ref()
    }
}

impl ThingBuilderVariant for CharacterBuilder {
    fn thing_builder(self) -> ThingBuilder {
        ThingBuilder::Character(self)
    }
}

impl Something for Character {}