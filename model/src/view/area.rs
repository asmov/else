use crate::{builder::*, descriptor::*, identity::*, route::*, view::thing::*};

pub struct AreaView {
    identity: Identity,
    descriptor: Descriptor,
    things: Vec<ThingView>, // only those visible to the viewer
    routes: Vec<Route> // safe to hand to the client (for now)
}

impl Built for AreaView {
    type BuilderType = AreaViewBuilder;
}

impl AreaView {
    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }

    pub fn things(&self) -> &Vec<ThingView> {
        &self.things
    }

    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }
}

pub enum AreaViewFields {
    Identity,
    Descriptor,
    Things,
    Routes
}

impl Fields for AreaViewFields {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => Self::FIELD_IDENTITY,
            Self::Descriptor => Self::FIELD_DESCRIPTOR,
            Self::Things => Self::FIELD_THINGS,
            Self::Routes => Self::FIELD_ROUTES,
        }
    }
}

impl AreaViewFields {
    const CLASSNAME: &'static str = "AreaView";
    const FIELD_IDENTITY: &'static Field = &Field::new(Self::CLASSNAME, "Identity", FieldValueType::Object);
    const FIELD_DESCRIPTOR: &'static Field = &Field::new(Self::CLASSNAME, "Descriptor", FieldValueType::Object);
    const FIELD_THINGS: &'static Field = &Field::new(Self::CLASSNAME, "Things", FieldValueType::ObjectArray);
    const FIELD_ROUTES: &'static Field = &Field::new(Self::CLASSNAME, "Routes", FieldValueType::ObjectArray);
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

    fn create(mut self) -> crate::Result<Creation<Self::BuilderType>> {
        let identity = Creation::try_assign(&mut self.identity, AreaViewFields::Identity)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, AreaViewFields::Descriptor)?;
        let things = Creation::assign_vec(&mut self.things)?;
        let routes = Creation::assign_vec(&mut self.routes)?;

        let area_view = AreaView {
            identity,
            descriptor,
            things,
            routes,
        };

        Ok(Creation::new(self, area_view))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> crate::Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.identity.is_some() {
            original.identity = Creation::assign(&mut self.identity)?;
            fields_changed.push(AreaViewFields::Identity.field());
        }
        if self.descriptor.is_some() {
            original.descriptor = Creation::assign(&mut self.descriptor)?;
            fields_changed.push(AreaViewFields::Descriptor.field());
        }

        Creation::modify_vec(&mut self.things, &mut original.things)?;
        Creation::modify_vec(&mut self.routes, &mut original.routes)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl BuildableIdentity for AreaViewBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> crate::Result<()> {
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
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> crate::Result<()> {
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
    fn add_route(&mut self, route: RouteBuilder) -> crate::Result<()> {
        self.routes.push(route);
        Ok(())
    }
}

impl AreaViewBuilder {
    fn add_thing(&mut self, thing: ThingViewBuilder) -> crate::Result<()> {
        self.things.push(thing);
        Ok(())
    }
}

