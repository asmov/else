use crate::{classes::*, error::*, identity::*, builder::*, descriptor::*, location::*};
use serde;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Entity {
    uid: UID,
    descriptor: Descriptor,
    location: Location
    //inventory: Inventory,
    //composition: Composition
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
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Location => &Self::FIELD_LOCATION,
            //Self::Inventory => &Self::FIELD_INVENTORY,
            //Self::Composition => &Self::FIELD_COMPOSITION,
        }
    }
}

impl Class for EntityField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl EntityField {
    const CLASS_ID: ClassID = ClassIdent::Entity as ClassID;
    const CLASSNAME: &'static str = "Entity";
    const FIELDNAME_IDENTITY: &'static str = "identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_LOCATION: &'static str = "location";
    //pub const FIELDNAME_INVENTORY: &'static str = "inventory";
    //pub const FIELDNAME_COMPOSITION: &'static str = "composition";

    const FIELD_IDENTITY: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_IDENTITY, FieldValueType::Model);
    const FIELD_DESCRIPTOR: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model);
    const FIELD_LOCATION: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_LOCATION, FieldValueType::UID);
    //pub const FIELD_INVENTORY: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_INVENTORY, FieldValueType::Object);
    //pub const FIELD_COMPOSITION: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_COMPOSITION, FieldValueType::Object);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
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
        let uid = Creation::try_assign(&mut self.identity, EntityField::Identity)?.to_uid();
        let descriptor = Creation::try_assign(&mut self.descriptor, EntityField::Descriptor)?;
        let location = Self::try_assign_value(&mut self.location, EntityField::Location)?;

        let entity = Entity {
            uid,
            descriptor,
            location
        };

        Ok(Creation::new(self, entity))
    }

    fn modify(mut self, original: &mut Entity) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.descriptor.is_some() { 
            Modification::assign(&mut self.descriptor, &mut original.descriptor)?;
            fields_changed.push(EntityField::Descriptor.field())
        }
        if self.location.is_some() {
            original.location = self.location.unwrap();
            fields_changed.push(EntityField::Location.field());
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn class_id(&self) -> ClassID {
        EntityField::class_id()
    }
}

pub trait BuildableEntity: Builder {
    fn entity(&mut self, entity: EntityBuilder) -> Result<()>; 
    fn entity_builder(&mut self) -> &mut EntityBuilder;
    fn get_entity(&self) -> Option<&EntityBuilder>;
}

impl BuildableIdentity for EntityBuilder {
    fn identity(&mut self, id: IdentityBuilder) -> Result<()> {
        self.identity = Some(id);
        Ok(())
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.identity.is_none() {
            self.identity = Some(Identity::builder(self.builder_mode()));
        }

        self.identity.as_mut().unwrap()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for EntityBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
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