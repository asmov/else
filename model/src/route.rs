pub mod direction;
pub mod endpoint;
pub mod junction;
pub mod point;

use crate::{error::*, builder::*, identity::*, descriptor::*};
use serde;

pub use crate::route::{endpoint::*, junction::*, point::*, direction::*};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Route {
    identity: Identity,
    descriptor: Descriptor,
    point_a: Point,
    point_b: Endpoint
}

impl Identifiable for Route {
    fn identity(&self) -> &Identity {
        &self.identity
    }
}

impl IdentifiableMut for Route {
    fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
    }
}

impl Descriptive for Route {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for Route {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

impl Built for Route {
    type BuilderType = RouteBuilder;
}

impl Route {
    pub fn point_a(&self) -> &Point {
        &self.point_a
    }

    pub fn point_b(&self) -> &Endpoint {
        &self.point_b
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RouteField {
    Identity,
    Descriptor,
    PointA,
    PointB
}

impl RouteField {
    pub const CLASSNAME: &'static str = "Route";
    pub const FIELDNAME_IDENTITY: &'static str = "identity";
    pub const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    pub const FIELDNAME_POINT_A: &'static str = "point_a";
    pub const FIELDNAME_POINT_B: &'static str = "point_b";

    pub const FIELD_IDENTITY: Field = Field::new(Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    pub const FIELD_DESCRIPTOR: Field = Field::new(Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    pub const FIELD_POINT_A: Field = Field::new(Self::FIELDNAME_POINT_A, FieldValueType::Object);
    pub const FIELD_POINT_B: Field = Field::new(Self::FIELDNAME_POINT_B, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::PointA => &Self::FIELD_POINT_A,
            Self::PointB => &Self::FIELD_POINT_B
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    point_a: Option<PointBuilder>,
    point_b: Option<EndpointBuilder>
}

impl Builder for RouteBuilder {
    type Type = Route;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            point_a: None,
            point_b: None
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
        Ok(Route {
            identity: self.identity
                .ok_or_else(||
                    Error::FieldNotSet{class: RouteField::CLASSNAME, field: RouteField::FIELDNAME_IDENTITY})?
                .create()?,
            descriptor: self.descriptor
                .ok_or_else(||
                    Error::FieldNotSet{class: RouteField::CLASSNAME, field: RouteField::FIELDNAME_DESCRIPTOR})?
                .create()?,
            point_a: self.point_a
                .ok_or_else(||
                    Error::FieldNotSet{class: RouteField::CLASSNAME, field: RouteField::FIELDNAME_POINT_A})?
                .create()?,
            point_b: self.point_b
                .ok_or_else(||
                    Error::FieldNotSet{class: RouteField::CLASSNAME, field: RouteField::FIELDNAME_POINT_B})?
                .create()?,
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<Modification<Self>> {
        let mut fields_changed = Vec::new();

        if let Some(identity) = self.identity {
            original.identity = identity.create()?;
            fields_changed.push(RouteField::Identity.field())
        }
        if let Some(descriptor) = self.descriptor {
            original.descriptor = descriptor.create()?;
            fields_changed.push(RouteField::Descriptor.field())
        }
        if let Some(point_a) = self.point_a {
            original.point_a = point_a.create()?;
            fields_changed.push(RouteField::PointA.field())
        }
        if let Some(point_b) = self.point_b {
            original.point_b = point_b.create()?;
            fields_changed.push(RouteField::PointB.field())
        }

        Ok(Modification::new(self, fields_changed))
    }
}

impl BuildableIdentity for RouteBuilder {
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

impl BuildableDescriptor for RouteBuilder {
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

impl RouteBuilder {
    pub fn point_a(&mut self, point_a: PointBuilder) -> Result<()> {
        self.point_a = Some(point_a);
        Ok(())
    }

    pub fn point_a_builder(&mut self) -> &mut PointBuilder {
        todo!()
    }

    pub fn point_b(&mut self, point_b: EndpointBuilder) -> Result<()> {
        self.point_b = Some(point_b);
        Ok(())
    }

    pub fn point_b_builder(&mut self) -> &mut EndpointBuilder {
        todo!()
    }
}

pub trait BuildableRouteVector {
    fn add_route(&mut self, route: RouteBuilder) -> Result<()>; 
}

