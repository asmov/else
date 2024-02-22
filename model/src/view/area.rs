use crate::{error::*, builder::*, descriptor::*, identity::*, route::*, view::thing::*, codebase::*};

pub struct AreaView {
    uid: UID,
    descriptor: Descriptor,
    things: Vec<ThingView>, // only those visible to the viewer
    route_ids: Vec<UID> // safe to hand to the client (for now)
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
    fn route_ids(&self) -> &Vec<UID> {
        &self.route_ids
    }
}

impl AreaView {
    pub fn things(&self) -> &Vec<ThingView> {
        &self.things
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
    const FIELD_UID: &'static Field = &Field::new(&Self::CLASS_IDENT, "uid", FieldValueType::UID);
    const FIELD_DESCRIPTOR: &'static Field = &Field::new(&Self::CLASS_IDENT, "descriptor", FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_THINGS: &'static Field = &Field::new(&Self::CLASS_IDENT, "things", FieldValueType::ModelList);
    const FIELD_ROUTES: &'static Field = &Field::new(&Self::CLASS_IDENT, "routes", FieldValueType::ModelList);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

pub struct AreaViewBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    things: Vec<ThingViewBuilder>,
    routes: Vec<RouteBuilder>
}

impl Builder for AreaViewBuilder {
    type ModelType = AreaView;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            things: Vec::new(),
            routes: Vec::new()
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
        let uid = Creation::try_assign(&mut self.identity, AreaViewField::UID)?.to_uid();
        let descriptor = Creation::try_assign(&mut self.descriptor, AreaViewField::Descriptor)?;
        let things = Creation::assign_vec(&mut self.things)?;
        let route_ids = Creation::assign_vec_uid(&mut self.routes)?;

        let area_view = AreaView {
            uid,
            descriptor,
            things,
            route_ids,
        };

        Ok(Creation::new(self, area_view))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.identity.is_some() {
            original.uid = Creation::assign(&mut self.identity)?.to_uid();
            fields_changed.push(AreaViewField::UID.field());
        }
        if self.descriptor.is_some() {
            original.descriptor = Creation::assign(&mut self.descriptor)?;
            fields_changed.push(AreaViewField::Descriptor.field());
        }

        Creation::modify_vec(&mut self.things, &mut original.things)?;
        Creation::modify_vec_uid(&mut self.routes, &mut original.route_ids)?;

        Ok(Modification::new_old(self, fields_changed))
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

impl BuildableRouteVector for AreaViewBuilder {
    fn add_route(&mut self, route: RouteBuilder) -> Result<()> {
        self.routes.push(route);
        Ok(())
    }
}

impl AreaViewBuilder {
    pub fn add_thing(&mut self, thing: ThingViewBuilder) -> Result<()> {
        self.things.push(thing);
        Ok(())
    }
}

