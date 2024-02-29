use crate::{codebase::*, descriptor::*, error::*, identity::*, modeling::*, route::{self, *}, sync::*, thing::*, world::*};
use serde;

/// Represents an area that things are located in, generally. There is no exact position.
/// Each area has a fixed set of `Route` objects that link it to other areas. 
/// There is a dynamic list of `Thing` objects thare are current occupants.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Area {
    uid: UID,
    descriptor: Descriptor,
    route_uids: Vec<UID>,
    occupant_uids: Vec<UID>
}

impl Keyed for Area {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Area {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for Area {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Area {
    /// Returns all Thing UIDs currently located here.
    pub fn occupant_uids(&self) -> &Vec<UID> {
        &self.occupant_uids
    }

    pub fn route_uids(&self) -> &Vec<UID> {
        &self.route_uids
    }
}

#[derive(Debug)]
pub enum AreaField {
    Identity,
    Descriptor,
    Routes,
    Occupants,
}

impl Fields for AreaField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Occupants => &Self::FIELD_OCCUPANTS,
        }
    }
}

impl Class for AreaField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl AreaField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Area as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Area";
    const FIELDNAME_UID: &'static str = "identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_ROUTES: &'static str = "routes";
    const FIELDNAME_OCCUPANTS: &'static str = "occupants";

    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTES, FieldValueType::UIDList(RouteField::class_ident_const()));
    const FIELD_OCCUPANTS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_OCCUPANTS, FieldValueType::UIDList(Thing::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    occupant_uids: Vec<ListOp<UID, UID>>,
    route_uids: Vec<ListOp<UID, UID>>
}

impl Builder for AreaBuilder {
    type ModelType = Area;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            occupant_uids: Vec::new(),
            route_uids: Vec::new()
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

        let uid = Build::create(&mut self.identity, &mut fields_changed, AreaField::Identity)?.uid();
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, AreaField::Descriptor)?;
        let occupant_uids = Build::create_uid_vec(&mut self.occupant_uids, &mut fields_changed, AreaField::Occupants)?;
        let route_uids = Build::create_uid_vec(&mut self.route_uids, &mut fields_changed, AreaField::Routes)?;

        let area = Area {
            uid,
            descriptor,
            occupant_uids,
            route_uids
        };

        Ok(Creation::new(self, area))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        if self.descriptor.is_some() {
            Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, AreaField::Descriptor)?;
        }
        if !self.occupant_uids.is_empty() {
            Build::modify_uid_vec(&mut self.occupant_uids, &mut existing.occupant_uids, &mut fields_changed, AreaField::Occupants)?;
        }
        if !self.route_uids.is_empty() {
            Build::modify_uid_vec(&mut self.route_uids, &mut existing.route_uids, &mut fields_changed, AreaField::Routes)?;
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        AreaField::class_ident()
    }
}

impl MaybeIdentifiable for AreaBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableIdentity for AreaBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<&mut Self> {
        self.identity = Some(identity);
        Ok(self)
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
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
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
    fn add_area(&mut self, area: AreaBuilder) -> Result<&mut Self>; 
    fn edit_area(&mut self, area: AreaBuilder) -> Result<&mut Self>; 
    fn remove_area(&mut self, area_uid: UID) -> Result<&mut Self>; 
}

impl AreaBuilder {
    pub fn add_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self> {
        Build::add_uid_to_listops(thing_uid, &mut self.occupant_uids, AreaField::Occupants)?;
        Ok(self)
    }

    pub fn remove_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self> {
        Build::remove_uid_from_listops(thing_uid, &mut self.occupant_uids, AreaField::Occupants)?;
        Ok(self)
    }
 }

 impl BuildableRouteUIDList for AreaBuilder {
    fn add_route_uid(&mut self, route_uid: UID) -> Result<&mut Self> {
        Build::add_uid_to_listops(route_uid, &mut self.route_uids, AreaField::Occupants)?;
        Ok(self)
    }

    fn remove_route_uid(&mut self, route_uid: UID) -> Result<&mut Self> {
        Build::remove_uid_from_listops(route_uid, &mut self.route_uids, AreaField::Occupants)?;
        Ok(self)
    }
}

