use crate::{builder::*, descriptor::{self, *}, entity::*, error::*, identity::*};
use serde;

/// Represents an area that things are located in, generally. There is no exact position.
/// Each area has a fixed set of `Route` objects that link it to other areas. 
/// There is a dynamic list of `Thing` objects thare are current occupants.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Area {
    identity: Identity,
    descriptor: Descriptor,
    route_id_map: Vec<ID>,
    occupant_thing_ids: Vec<ID>
}

impl Identifiable for Area {
    fn identity(&self) -> &Identity {
        &self.identity
    }
}

impl IdentifiableMut for Area {
    fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
    }
}

impl Descriptive for Area {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for Area {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

#[derive(Debug)]
pub enum AreaField {
    Identity,
    Descriptor,
    Routes,
    Occupants,
}

impl AreaField {
    pub const CLASSNAME: &'static str = "Area";
    pub const FIELDNAME_IDENTITY: &'static str = "identity";
    pub const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    pub const FIELDNAME_ROUTES: &'static str = "routes";
    pub const FIELDNAME_OCCUPANTS: &'static str = "occupants";

    pub const FIELD_IDENTITY: Field = Field::new(Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    pub const FIELD_DESCRIPTOR: Field = Field::new(Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    pub const FIELD_ROUTES: Field = Field::new(Self::FIELDNAME_ROUTES, FieldValueType::ObjectIDArray);
    pub const FIELD_OCCUPANTS: Field = Field::new(Self::FIELDNAME_OCCUPANTS, FieldValueType::ObjectIDArray);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Occupants => &Self::FIELD_OCCUPANTS,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaBuilder {
    builder_mode: BuilderMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    identity: Option<IdentityBuilder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    descriptor: Option<DescriptorBuilder>,
}

impl Builder for AreaBuilder {
    type ModelType = Area;
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
        let identity = Creation::try_assign(&mut self.identity, AreaField::CLASSNAME, AreaField::FIELDNAME_IDENTITY)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, AreaField::CLASSNAME, AreaField::FIELDNAME_DESCRIPTOR)?;

        let area = Area {
            identity,
            descriptor,
            route_id_map: Vec::new(), //todo
            occupant_thing_ids: Vec::new(), //todo
        };

        Ok(Creation::new(self, area))
    }

    fn modify(self, _original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let fields_changed = Vec::new();
        //todo
        Ok(Modification::new(self, fields_changed))
    }
}

impl BuildableIdentity for AreaBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        self.identity = Some(identity);
        Ok(())
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.identity.is_none() {
            self.identity = Some(Identity::builder(self.builder_mode()))
        }

        self.identity.as_mut().unwrap()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for AreaBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::builder(self.builder_mode()))
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl Built for Area {
    type BuilderType = AreaBuilder;
}

pub trait BuildableAreaVector {
    fn add_area(&mut self, area: AreaBuilder) -> Result<()>; 
}

