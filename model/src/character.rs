pub use crate::{location::*, codebase::*, error::*, identity::*, modeling::*, descriptor::*, entity::*, something::*,
    thing::*, cortex::*, world::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Character {
    entity: Entity,
    cortex: Cortex
}

impl Built for Character {
    type BuilderType = CharacterBuilder;
}

impl Keyed for Character {
    fn key(&self) -> Option<&str> {
        self.descriptor().key()
    }
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

impl Located for Character {
    fn location(&self) -> Location {
        self.entity().location()
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

impl Something for Character {}

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
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl CharacterField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Character as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Character";
    const FIELDNAME_ENTITY: &'static str = "entity";
    const FIELDNAME_CORTEX: &'static str = "cortex";

    const FIELD_ENTITY: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ENTITY, FieldValueType::Model(EntityField::class_ident_const()));
    //todo: this is the wrong class_ident
    const FIELD_CORTEX: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_CORTEX, FieldValueType::Model(RoutineLobeField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
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
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let entity = Build::create(&mut self.entity, &mut fields_changed, CharacterField::Entity)?;
        let cortex = Build::create(&mut self.cortex, &mut fields_changed, CharacterField::Cortex)?;

        let character = Character {
            entity,
            cortex
        };

        Ok(Creation::new(ThingBuilder::Character(self), Thing::Character(character)))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        if self.entity.is_some() {
            Build::modify(&mut self.entity, &mut existing.entity, &mut fields_changed, CharacterField::Entity)?;
        }
        if self.cortex.is_some() {
            Build::modify(&mut self.cortex, &mut existing.cortex, &mut fields_changed, CharacterField::Cortex)?;
        }

        Ok(Modification::new(ThingBuilder::Character(self), fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        CharacterField::class_ident()
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

impl MaybeIdentifiable for CharacterBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableIdentity for CharacterBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<&mut Self> {
        self.entity_builder().identity(identity)?;
        Ok(self)
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        self.entity_builder().identity_builder()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.get_entity()
            .and_then(|entity| entity.get_identity())
    }
}

impl ThingBuilderVariant for CharacterBuilder {
    fn thing_builder(self) -> ThingBuilder {
        ThingBuilder::Character(self)
    }
}
