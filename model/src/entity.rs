use crate::{codebase::*, error::*, identity::*, modeling::*, descriptor::*, location::*, world::*};
use serde;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Entity {
    uid: UID,
    descriptor: Descriptor,
    location: Location
    //inventory: Inventory,
    //composition: Composition
}

impl Keyed for Entity {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Entity {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Built for Entity {
    type BuilderType = EntityBuilder;
}

impl Descriptive for Entity {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Located for Entity {
    fn location(&self) -> Location {
        self.location
    }
}

pub trait Exists: Identifiable + Descriptive + Located {
    fn entity(&self) -> &Entity;
}

#[derive(Debug)]
pub enum EntityField {
    Identity,
    Descriptor,
    Location
    //Inventory,
    //Composition
}

impl Fields for EntityField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Location => &Self::FIELD_LOCATION,
            //Self::Inventory => &Self::FIELD_INVENTORY,
            //Self::Composition => &Self::FIELD_COMPOSITION,
        }
    }
}

impl Class for EntityField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl EntityField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Entity as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Entity";
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_LOCATION: &'static str = "location";
    //pub const FIELDNAME_INVENTORY: &'static str = "inventory";
    //pub const FIELDNAME_COMPOSITION: &'static str = "composition";

    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_LOCATION: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_LOCATION, FieldValueType::UID(Location::class_ident_const()));
    //pub const FIELD_INVENTORY: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INVENTORY, FieldValueType::Object);
    //pub const FIELD_COMPOSITION: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_COMPOSITION, FieldValueType::Object);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityBuilder {
    builder_mode: BuilderMode,
    identity: Option<UID>,
    location: Option<Location>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for EntityBuilder {
    type ModelType = Entity;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            location: None
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

        let uid = Build::create_value(&mut self.identity, &mut fields_changed, EntityField::Identity)?;
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, EntityField::Descriptor)?;
        let location = Build::create_value(&self.location, &mut fields_changed, EntityField::Location)?;

        let entity = Entity {
            uid,
            descriptor,
            location
        };

        Ok(Creation::new(self, entity))
    }

    fn modify(mut self, existing: &mut Entity) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_ex(&mut self, existing)?;

        Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, EntityField::Descriptor)?;
        Build::modify_value(&self.location, &mut existing.location, &mut fields_changed, EntityField::Location)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        EntityField::class_ident()
    }
}

pub trait BuildableEntity: Builder {
    fn entity(&mut self, entity: EntityBuilder) -> Result<()>; 
    fn entity_builder(&mut self) -> &mut EntityBuilder;
    fn get_entity(&self) -> Option<&EntityBuilder>;
}

impl MaybeIdentifiable for EntityBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableUID for EntityBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.identity = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for EntityBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::builder(self.builder_mode()));
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl EntityBuilder {
    pub fn location(&mut self, location: Location) -> Result<&mut Self> {
        self.location = Some(location);
        Ok(self)
    }

    pub fn get_location(&self) -> Option<Location> {
        self.location
    }
}