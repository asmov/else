use crate::{error::*, identity::*, builder::*, descriptor::*};
use serde;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Entity {
    identity: Identity,
    descriptor: Descriptor,
    //inventory: Inventory,
    //composition: Composition
}

pub trait Exists: Identifiable + Descriptive {
    fn entity(&self) -> &Entity;
}

pub trait ExistsMut: Exists + IdentifiableMut + DescriptiveMut {
    fn entity_mut(&mut self) -> &mut Entity;
}

#[derive(Debug)]
pub enum EntityField {
    Identity,
    Descriptor,
    //Inventory,
    //Composition
}

impl Fields for EntityField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            //Self::Inventory => &Self::FIELD_INVENTORY,
            //Self::Composition => &Self::FIELD_COMPOSITION,
        }
    }
}

impl EntityField {
    const CLASSNAME: &'static str = "Entity";
    const FIELDNAME_IDENTITY: &'static str = "identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    //pub const FIELDNAME_INVENTORY: &'static str = "inventory";
    //pub const FIELDNAME_COMPOSITION: &'static str = "composition";

    const FIELD_IDENTITY: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    const FIELD_DESCRIPTOR: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    //pub const FIELD_INVENTORY: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_INVENTORY, FieldValueType::Object);
    //pub const FIELD_COMPOSITION: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_COMPOSITION, FieldValueType::Object);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for EntityBuilder {
    type ModelType = Entity;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None
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
        let identity = Creation::try_assign(&mut self.identity, EntityField::Identity)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, EntityField::Descriptor)?;

        let entity = Entity {
            identity: identity,
            descriptor: descriptor,
        };

        Ok(Creation::new(self, entity))
    }

    fn modify(mut self, original: &mut Entity) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.descriptor.is_some() { 
            original.descriptor = Creation::assign(&mut self.descriptor)?;
            fields_changed.push(EntityField::Descriptor.field())
        }

        Ok(Modification::new(self, fields_changed))
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

impl Identifiable for Entity {
    fn identity(&self) -> &Identity {
        &self.identity
    }
}

impl IdentifiableMut for Entity {
    fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
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

impl DescriptiveMut for Entity {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}