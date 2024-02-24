use crate::{codebase::*, descriptor::*, error::*, identity::*, modeling::*, route::*, world::*, thing::*};

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
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    occupant_uids: Vec<ListOp<IdentityBuilder, UID>>,
    route_uids: Vec<ListOp<IdentityBuilder, UID>>
}

impl Builder for AreaViewBuilder {
    type ModelType = AreaView;
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

        let uid = Build::create(&mut self.identity, &mut fields_changed, AreaViewField::UID)?.to_uid();
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

        if self.descriptor.is_some() {
            Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, AreaViewField::Descriptor)?;
        }

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

impl BuildableIdentity for AreaViewBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        self.identity = Some(identity);
        Ok(())
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.identity.is_none() {
            self.identity = Some(IdentityBuilder::builder(self.builder_mode));
        }

        self.identity.as_mut().unwrap()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for AreaViewBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(DescriptorBuilder::builder(self.builder_mode));
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl BuildableRouteUIDList for AreaViewBuilder {
    fn add_route_uid(&mut self, uid: UID) -> Result<()> {
        self.route_uids.push(ListOp::Add(IdentityBuilder::from_existing(self, &uid)));
        Ok(())
    }

    fn remove_route_uid(&mut self, uid: UID) -> Result<()> {
        self.route_uids.push(ListOp::Remove(uid));
        Ok(())
    }
}

impl BuildableOccupantList for AreaViewBuilder {
    fn add_occupant_uid(&mut self, uid: UID) -> Result<()> {
        self.occupant_uids.push(ListOp::Add(IdentityBuilder::from_existing(self, &uid)));
        Ok(())
    }

    fn remove_occupant_uid(&mut self, uid: UID) -> Result<()> {
        self.occupant_uids.push(ListOp::Remove(uid));
        Ok(())
    }
}