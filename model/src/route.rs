pub mod direction;
pub mod end;
pub mod endpoint;
pub mod junction;
pub mod point;

use crate::{codebase::*, error::*, modeling::*, identity::*, descriptor::*, world::*};
use serde;

pub use crate::route::{end::*, endpoint::*, junction::*, point::*, direction::*};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Route {
    uid: UID,
    descriptor: Descriptor,
    point_a: Point,
    point_b: Point 
}

impl Keyed for Route {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Route {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for Route {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for Route {
    type BuilderType = RouteBuilder;
}

impl Route {
    pub fn point_a(&self) -> &Point {
        &self.point_a
    }

    pub fn point_b(&self) -> &Point {
        &self.point_b
    }
}

pub trait Routing {
    fn route_uids(&self) -> &Vec<UID>;
}

#[derive(Clone, Copy, Debug)]
pub enum RouteField {
    UID,
    Descriptor,
    PointA,
    PointB
}

impl Fields for RouteField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::PointA => &Self::FIELD_POINT_A,
            Self::PointB => &Self::FIELD_POINT_B
        }
    }
}

impl Class for RouteField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl RouteField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Route as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Route";
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_POINT_A: &'static str = "point_a";
    const FIELDNAME_POINT_B: &'static str = "point_b";

    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::Model(IdentityField::class_ident_const()));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_POINT_A: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_POINT_A, FieldValueType::Model(EndpointField::class_ident_const()));
    const FIELD_POINT_B: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_POINT_B, FieldValueType::Model(EndpointField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    point_a: Option<PointBuilder>,
    point_b: Option<PointBuilder>
}

impl Builder for RouteBuilder {
    type DomainType = World;
    type ModelType = Route;
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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let uid = Build::create(&mut self.identity, &mut fields_changed, RouteField::UID)?.uid();
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, RouteField::Descriptor)?;
        let point_a = Build::create(&mut self.point_a, &mut fields_changed, RouteField::PointA)?;
        let point_b = Build::create(&mut self.point_b, &mut fields_changed, RouteField::PointB)?;

        let route = Route {
            uid,
            descriptor,
            point_a,
            point_b,
        };

        Ok(Creation::new(self, route))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        if self.descriptor.is_some() {
            Build::create(&mut self.descriptor, &mut fields_changed, RouteField::Descriptor)?;
        }
        if self.point_a.is_some() {
            Build::create(&mut self.point_a, &mut fields_changed, RouteField::PointA)?;
        }
        if self.point_b.is_some() {
            Build::create(&mut self.point_b, &mut fields_changed, RouteField::PointB)?;
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        RouteField::class_ident()
    }
}

impl MaybeIdentifiable for RouteBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
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

    pub fn point_b(&mut self, point_b: PointBuilder) -> Result<()> {
        assert!(matches!(point_b, PointBuilder::Endpoint(_)));
        self.point_b = Some(point_b);
        Ok(())
    }

    pub fn point_b_builder(&mut self) -> &mut PointBuilder {
        todo!()
    }
}

pub trait BuildableRouteVector {
    fn add_route(&mut self, route: RouteBuilder) -> Result<()>; 
}

pub trait BuildableRouteUIDList {
    fn add_route_uid(&mut self, uid: UID) -> Result<()>; 
    fn remove_route_uid(&mut self, uid: UID) -> Result<()>; 
}

