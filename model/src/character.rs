pub use crate::{s, classes::*, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, cortex::*};
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

impl Fields for CharacterField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Entity => &Self::FIELD_ENTITY,
            Self::Cortex => &Self::FIELD_CORTEX,
        }
    }
}

impl Class for CharacterField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl CharacterField {
    const CLASS_ID: ClassID = ClassIdent::Character as ClassID;
    const CLASSNAME: &'static str = "Character";
    const FIELDNAME_ENTITY: &'static str = "entity";
    const FIELDNAME_CORTEX: &'static str = "cortex";

    const FIELD_ENTITY: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_ENTITY, FieldValueType::Model);
    const FIELD_CORTEX: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_CORTEX, FieldValueType::Model);
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
        let entity = Creation::try_assign(&mut self.entity, CharacterField::Entity)?;
        let cortex = Creation::try_assign(&mut self.cortex, CharacterField::Cortex)?;

        let character = Character {
            entity: entity,
            cortex: cortex
        };

        Ok(Creation::new(ThingBuilder::Character(self), Thing::Character(character)))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<ThingBuilder>> {
        let mut fields_changed = Vec::new();

        // todo
        if self.entity.is_some() {
            let modification = self.entity.take().unwrap().modify(&mut original.entity)?;
            self.entity = Some(modification.take_builder());
            fields_changed.push(CharacterField::Entity.field());
        }

        Ok(Modification::new(ThingBuilder::Character(self), fields_changed))
    }

    fn class_id(&self) -> ClassID {
        CharacterField::class_id()
    }
}

impl Built for Character {
    type BuilderType = CharacterBuilder;
}

impl Identifiable for Character {
    fn uid(&self) -> UID {
        self.entity.uid()
    }
}

impl Descriptive for Character {
    fn descriptor(&self) -> &Descriptor{
        self.entity().descriptor()
    }
}

impl Exists for Character {
    fn entity(&self) -> &Entity {
        &self.entity
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