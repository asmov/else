use crate::{s, model::{error::*, identity::*, builder::*, descriptor::*, inventory::*, composition::*}};


#[derive(Debug)]
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

impl EntityField {
    pub const CLASSNAME: &'static str = "Entity";
    pub const FIELDNAME_IDENTITY: &'static str = "identity";
    pub const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    //pub const FIELDNAME_INVENTORY: &'static str = "inventory";
    //pub const FIELDNAME_COMPOSITION: &'static str = "composition";

    pub const FIELD_IDENTITY: Field = Field::new(Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    pub const FIELD_DESCRIPTOR: Field = Field::new(Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    //pub const FIELD_INVENTORY: Field = Field::new(Self::FIELDNAME_INVENTORY, FieldValueType::Object);
    //pub const FIELD_COMPOSITION: Field = Field::new(Self::FIELDNAME_COMPOSITION, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            //Self::Inventory => &Self::FIELD_INVENTORY,
            //Self::Composition => &Self::FIELD_COMPOSITION,
        }
    }
}

#[derive(Debug)]
pub struct EntityBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for EntityBuilder {
    type Type = Entity;

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

    fn create(self) -> Result<Self::Type> {
        Ok(Entity {
            identity: self.identity
                .ok_or_else(||
                    Error::FieldNotSet{class:EntityField::CLASSNAME, field: EntityField::FIELDNAME_IDENTITY})?
                .create()?,
            descriptor: self.descriptor
                .ok_or_else(||
                    Error::FieldNotSet{class:EntityField::CLASSNAME, field: EntityField::FIELDNAME_DESCRIPTOR})?
                .create()?,
        })
    }

    fn modify(self, original: &mut Entity) -> Result<ModifyResult> {
        let mut fields_changed = Vec::new();

        if let Some(identity) = self.identity {
            original.identity = identity.create()?;
            fields_changed.push(EntityField::Identity.field())
        }
        if let Some(descriptor) = self.descriptor {
            original.descriptor = descriptor.create()?;
            fields_changed.push(EntityField::Descriptor.field())
        }

        Ok(ModifyResult::new(fields_changed))
    }
}

pub trait BuildableEntity: Builder {
    fn entity(&mut self, entity: EntityBuilder) -> Result<()>; 
    fn entity_builder(&mut self) -> &mut EntityBuilder;
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

impl Build for Entity {
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