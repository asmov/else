use crate::{error::*, builder::*, identity::*, descriptor::*, route::*};
use serde;

/// Connects a single Area to a Route.  
/// All fields are from the point-of-view of the Area, describing the Route that this connects to.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Endpoint {
    /// The identity Area that this endpoint provides an exit/entrance for.
    area_identity: Identity,
    /// The description of the Route that this endpoint connects to, from the point-of-view of the Area.
    descriptor: Descriptor,
    /// The direction that this endpoint is found at, from the point-of-view of the Area.
    direction: Direction
}

#[derive(Clone, Copy, Debug)]
pub enum EndpointField {
    AreaIdentity,
    Descriptor,
    Direction
}

impl EndpointField {
    pub const CLASSNAME: &'static str = "Endpoint";
    pub const FIELDNAME_AREA_IDENTITY: &'static str = "area_identity";
    pub const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    pub const FIELDNAME_DIRECTION: &'static str = "direction";

    pub const FIELD_AREA_IDENTITY: Field = Field::new(Self::FIELDNAME_AREA_IDENTITY, FieldValueType::Object);
    pub const FIELD_DESCRIPTOR: Field = Field::new(Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    pub const FIELD_DIRECTION: Field = Field::new(Self::FIELDNAME_DIRECTION, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::AreaIdentity => &Self::FIELD_AREA_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Direction => &Self::FIELD_DIRECTION
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EndpointBuilder {
    builder_mode: BuilderMode,
    area_identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    direction: Option<Direction>
}

impl Builder for EndpointBuilder {
    type ModelType = Endpoint;
    type BuilderType = PointBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            area_identity: None,
            descriptor: None,
            direction: None
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

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        todo!()/*Ok(Endpoint {
            area_identity: self.area_identity
                .ok_or_else(||
                    Error::FieldNotSet{class: EndpointField::CLASSNAME, field: EndpointField::FIELDNAME_AREA_IDENTITY})?
                .create()?,
            descriptor: self.descriptor
                .ok_or_else(||
                    Error::FieldNotSet{class: EndpointField::CLASSNAME, field: EndpointField::FIELDNAME_DESCRIPTOR})?
                .create()?,
            direction: self.direction
                .ok_or_else(||
                    Error::FieldNotSet{class: EndpointField::CLASSNAME, field: EndpointField::FIELDNAME_DIRECTION})?
        })*/
    }

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()/*let mut fields_changed = Vec::new();

        if let Some(area_identity) = self.area_identity {
            original.area_identity = area_identity.create()?;
            fields_changed.push(EndpointField::AreaIdentity.field())
        }
        if let Some(descriptor) = self.descriptor {
            original.descriptor = descriptor.create()?;
            fields_changed.push(EndpointField::Descriptor.field())
        }
        if let Some(direction) = self.direction {
            original.direction = direction;
            fields_changed.push(EndpointField::Direction.field())
        }

        Ok(Modification::new(PointBuilder::Endpoint(self), fields_changed))*/
    }
}

impl Built for Endpoint {
    type BuilderType = EndpointBuilder;
}

impl Descriptive for Endpoint {
    /// The description of the Route that this endpoint connects to, from the point-of-view of the Area.
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for Endpoint {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

impl Endpoint {
    /// The identity of the Area that this endpoint provides an exit/entrance for.
    pub fn area_identity(&self) -> &Identity {
        &self.area_identity
    } 

    /// The general direction that this endpoint is found within the Area.
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

impl BuildableDescriptor for EndpointBuilder {
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

impl EndpointBuilder {
    pub fn area_identity(&mut self, id: IdentityBuilder) -> Result<()> {
        self.area_identity = Some(id);
        Ok(())
    }

    pub fn area_identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.area_identity.is_none() {
            self.area_identity = Some(Identity::builder(self.builder_mode()));
        }

        self.area_identity.as_mut().unwrap()
    }

    pub fn get_area_identity(&self) -> Option<&IdentityBuilder> {
        self.area_identity.as_ref()
    }

    pub fn direction(&mut self, direction: Direction) -> Result<()> {
        self.direction = Some(direction);
        Ok(())
    }
}

impl point::BuildablePoint for EndpointBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Endpoint(self)
    }
}