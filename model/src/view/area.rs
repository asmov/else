use std::borrow::Cow;

use crate::{codebase::*, descriptor::*, error::*, identity::*, modeling::*, route::*, world::*, thing::*, view::world::*};

use super::world;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaView {
    uid: UID,
    descriptor: Descriptor,
    occupant_uids: Vec<UID>, // only those visible to the viewer
    route_uids: Vec<UID> // safe to hand to the client (for now)
}

impl Built for AreaView {
    type BuilderType = AreaViewBuilder;
}

impl Keyed for AreaView {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for AreaView {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for AreaView {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Routing for AreaView {
    fn route_uids(&self) -> &Vec<UID> {
        &self.route_uids
    }
}

impl AreaView {
    pub fn occupant_uids(&self) -> &Vec<UID> {
        &self.occupant_uids
    }

    /// This will return the name of a route with a numeric suffix if there are multiple routes with the same name.
    /// The suffixed name will be in the format of: "{name} #{index}". The index is 1-based.
    /// If the name is relatively unique, it will not be suffixed. 
    pub fn indexed_route_name<'w>(&self, route_uid: UID, world_view: &'w WorldView) -> Result<Cow<'w, str>> {
        if !self.route_uids.contains(&route_uid) {
            return Err(Error::ModelNotFound { model: RouteField::classname(), uid: route_uid });
        }

        let route = world_view.route(route_uid)?;
        let route_name = route.name();

        let similar_routes = world_view.routes().iter()
            .filter(|r| r.name() == route_name);

        if similar_routes.clone().count() > 1 {
            let index = 1 + similar_routes.into_iter().position(|r| r.uid() == route_uid).unwrap();
            let name = format!("{route_name} #{index}");
            Ok(Cow::Owned(name))
        } else {
            Ok(Cow::Borrowed(&route_name))
        }
    }

    /// This will return the name of a thing with a numeric suffix if there are multiple routes with the same name.
    /// The suffixed name will be in the format of: "{name} #{index}". The index is 1-based.
    /// If the name is relatively unique, it will not be suffixed.
    pub fn indexed_thing_name<'w>(&self, thing_uid: UID, world_view: &'w WorldView) -> Result<Cow<'w, str>> {
        if !self.occupant_uids.contains(&thing_uid) {
            return Err(Error::ModelNotFound { model: RouteField::classname(), uid: thing_uid });
        }

        let thing_view = world_view.thing_view(thing_uid)?;
        let thing_name = thing_view.name();

        let similar_things = world_view.thing_views().iter()
            .filter(|r| r.name() == thing_name);

        if similar_things.clone().count() > 1 {
            let index = 1 + similar_things.into_iter().position(|r| r.uid() == thing_uid).unwrap();
            let name = format!("{thing_name} #{index}");
            Ok(Cow::Owned(name))
        } else {
            Ok(Cow::Borrowed(&thing_name))
        }
    }

}

pub enum AreaViewField {
    UID,
    Descriptor,
    Things,
    Routes
}

impl Fields for AreaViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => Self::FIELD_UID,
            Self::Descriptor => Self::FIELD_DESCRIPTOR,
            Self::Things => Self::FIELD_THINGS,
            Self::Routes => Self::FIELD_ROUTES,
        }
    }
}

impl Class for AreaViewField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl AreaViewField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::AreaView as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "AreaView";
    const FIELD_UID: &'static Field = &Field::new(&Self::CLASS_IDENT, "uid", FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_DESCRIPTOR: &'static Field = &Field::new(&Self::CLASS_IDENT, "descriptor", FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_THINGS: &'static Field = &Field::new(&Self::CLASS_IDENT, "things", FieldValueType::ModelList(Thing::class_ident_const()));
    const FIELD_ROUTES: &'static Field = &Field::new(&Self::CLASS_IDENT, "routes", FieldValueType::ModelList(RouteField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaViewBuilder {
    builder_mode: BuilderMode,
    uid: Option<UID>,
    descriptor: Option<DescriptorBuilder>,
    occupant_uids: Vec<ListOp<UID, UID>>,
    route_uids: Vec<ListOp<UID, UID>>
}

impl Builder for AreaViewBuilder {
    type ModelType = AreaView;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            uid: None,
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

        let uid = Build::create_uid(&mut self.uid, &mut fields_changed, AreaViewField::UID)?;
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, AreaViewField::Descriptor)?;
        let occupant_uids = Build::create_uid_vec(&mut self.occupant_uids, &mut fields_changed, AreaViewField::Things)?;
        let route_uids = Build::create_uid_vec(&mut self.route_uids, &mut fields_changed, AreaViewField::Routes)?;

        let area_view = AreaView {
            uid,
            descriptor,
            occupant_uids,
            route_uids,
        };

        Ok(Creation::new(self, area_view))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, AreaViewField::Descriptor)?;
        Build::modify_uid_vec(&mut self.occupant_uids, &mut existing.occupant_uids, &mut fields_changed, AreaViewField::Things)?;
        Build::modify_uid_vec(&mut self.route_uids, &mut existing.route_uids, &mut fields_changed, AreaViewField::Things)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        AreaViewField::class_ident()
    }
}

impl MaybeIdentifiable for AreaViewBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableUID for AreaViewBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.uid = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.uid.as_ref()
    }
}

impl BuildableDescriptor for AreaViewBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(DescriptorBuilder::builder(self.builder_mode));
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl BuildableRouteUIDList for AreaViewBuilder {
    fn add_route_uid(&mut self, route_uid: UID) -> Result<&mut Self> {
        Build::add_uid_to_listops(route_uid, &mut self.route_uids, AreaViewField::Routes)?;
        Ok(self)
    }

    fn remove_route_uid(&mut self, route_uid: UID) -> Result<&mut Self> {
        Build::remove_uid_from_listops(route_uid, &mut self.route_uids, AreaViewField::Routes)?;
        Ok(self)
    }
}

impl BuildableOccupantList for AreaViewBuilder {
    fn add_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self> {
        Build::add_uid_to_listops(thing_uid, &mut self.occupant_uids, AreaViewField::Things)?;
        Ok(self)
    }

    fn remove_occupant_uid(&mut self, thing_uid: UID) -> Result<&mut Self> {
        Build::remove_uid_from_listops(thing_uid, &mut self.occupant_uids, AreaViewField::Things)?;
        Ok(self)
    }
}