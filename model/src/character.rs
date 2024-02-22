pub use crate::{location::*, codebase::*, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, cortex::*};
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
    const FIELD_CORTEX: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_CORTEX, FieldValueType::Model(RoutineCortexField::class_ident_const()));

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
        let entity = Creation::try_assign(&mut self.entity, CharacterField::Entity)?;
        let cortex = Creation::try_assign(&mut self.cortex, CharacterField::Cortex)?;

        let character = Character {
            entity: entity,
            cortex: cortex
        };

        Ok(Creation::new(ThingBuilder::Character(self), Thing::Character(character)))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<ThingBuilder>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        // todo
        if self.entity.is_some() {
            Build::modify(&mut self.entity, &mut original.entity, &mut fields_changed, CharacterField::Entity)?;
            /*let modification = self.entity.take().unwrap().modify(&mut original.entity)?;
            self.entity = Some(modification.take_builder());
            fields_changed.push(CharacterField::Entity.field());*/
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
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        self.entity_builder().identity(identity)
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
