use crate::model::{error::*, builder::*, identity::*, descriptor::*, entity::*, thing::*, area::*, route::*};

#[derive(Debug)]
pub struct World {
    identity: Identity,
    descriptor: Descriptor,
    areas: Vec<Area>,
    routes: Vec<Route>,
    things: Vec<Thing>,
    next_id: ID,
}

#[derive(Clone, Copy, Debug)]
pub enum WorldField {
    Identity,
    Descriptor,
    Areas,
    Routes,
    Things
}

impl WorldField {
    pub const CLASSNAME: &'static str = "World";
    pub const FIELDNAME_IDENTITY: &'static str = "identity";
    pub const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    pub const FIELDNAME_AREAS: &'static str = "areas";
    pub const FIELDNAME_ROUTES: &'static str = "routes";
    pub const FIELDNAME_THINGS: &'static str = "things";

    pub const FIELD_IDENTITY: Field = Field::new(Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    pub const FIELD_DESCRIPTOR: Field = Field::new(Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    pub const FIELD_AREAS: Field = Field::new(Self::FIELDNAME_AREAS, FieldValueType::ObjectArray);
    pub const FIELD_ROUTES: Field = Field::new(Self::FIELDNAME_ROUTES, FieldValueType::ObjectArray);
    pub const FIELD_THINGS: Field = Field::new(Self::FIELDNAME_THINGS, FieldValueType::ObjectArray);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Areas => &Self::FIELD_AREAS,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Things => &Self::FIELD_THINGS
        }
    }
}

#[derive(Debug)]
pub struct WorldBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    areas: Vec<AreaBuilder>,
    routes: Vec<RouteBuilder>,
    things: Vec<ThingBuilder>,
    next_id: ID,
}

impl Builder for WorldBuilder {
    type Type = World;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            areas: Vec::new(),
            routes: Vec::new(),
            things: Vec::new(),
            next_id: 1
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

    fn create(mut self) -> Result<Self::Type> {
        let identity = self.identity.as_ref()
            .ok_or_else(|| Error::FieldNotSet {class: WorldField::CLASSNAME, field: WorldField::FIELDNAME_IDENTITY})?
            .clone()
            .create()?;

        let mut next_id = self.generate_id();

        for area in &mut self.areas {
            let identity_builder = area.identity_builder();
            identity_builder.universe_id(identity.universe_id())?;
            identity_builder.world_id(identity.world_id())?;
            identity_builder.region_id(identity.region_id())?;
            identity_builder.id(next_id)?;
            next_id += 1;
        }

        for thing in &mut self.things {
            let identity_builder = thing.entity_builder().identity_builder();
            identity_builder.universe_id(identity.universe_id())?;
            identity_builder.world_id(identity.world_id())?;
            identity_builder.region_id(identity.region_id())?;
            identity_builder.id(next_id)?;
            next_id += 1;
        }

        self.next_id = next_id;

        Ok(World {
            identity,
            descriptor: self.descriptor
                .ok_or_else(||
                    Error::FieldNotSet {class: WorldField::CLASSNAME, field: WorldField::FIELDNAME_IDENTITY})?
                .create()?,
            areas: self.areas.into_iter()
                .map(|area| area.create())
                .collect::<Result<Vec<_>,_>>()?,
            routes: self.routes.into_iter()
                .map(|route| route.create())
                .collect::<Result<Vec<_>,_>>()?,
            things: self.things.into_iter()
                .map(|thing| thing.create())
                .collect::<Result<Vec<_>,_>>()?,
            next_id: self.next_id + 1,
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        for mut area in self.areas {
            if !area.has_identity() {
                let identity_builder = area.identity_builder();
                identity_builder.universe_id(original.identity().universe_id())?;
                identity_builder.world_id(original.identity().world_id())?;
                identity_builder.region_id(original.identity().region_id())?;
                identity_builder.id(original.generate_id())?;
            }

            original.areas.push(area.create()?);
        }

        for mut route in self.routes {
            if !route.has_identity() {
                let identity_builder = route.identity_builder();
                identity_builder.universe_id(original.identity().universe_id())?;
                identity_builder.world_id(original.identity().world_id())?;
                identity_builder.region_id(original.identity().region_id())?;
                identity_builder.id(original.generate_id())?;
            }

            original.routes.push(route.create()?);
        }

        for mut thing in self.things {
            if !thing.entity_builder().has_identity() {
                let identity_builder = thing.entity_builder().identity_builder();
                identity_builder.universe_id(original.identity().universe_id())?;
                identity_builder.world_id(original.identity().world_id())?;
                identity_builder.region_id(original.identity().region_id())?;
                identity_builder.id(original.generate_id())?;
            }

            original.things.push(thing.create()?);
        }


        if let Some(descriptor) = self.descriptor {
            original.descriptor = descriptor.create()?;
        }

        Ok(ModifyResult::new(Vec::new()))
    }
}

impl BuildableIdentity for WorldBuilder {
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

impl BuildableDescriptor for WorldBuilder {
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

impl BuildableAreaVector for WorldBuilder {
    fn add_area(&mut self, area: AreaBuilder) -> Result<()> {
        self.areas.push(area);
        Ok(())
    }
}

impl BuildableRouteVector for WorldBuilder {
    fn add_route(&mut self, route: RouteBuilder) -> Result<()> {
        self.routes.push(route);
        Ok(())
    }
}

impl BuildableThingVector for WorldBuilder {
    fn add_thing(&mut self, thing: ThingBuilder) -> Result<()> {
       self.things.push(thing); 
       Ok(())
    }
}

impl WorldBuilder {
    fn generate_id(&mut self) -> ID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Build for World {
    type BuilderType = WorldBuilder;
}

impl Identifiable for World {
    fn identity(&self) -> &Identity {
        &self.identity
    }
}

impl IdentifiableMut for World {
    fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
    }
}

impl Descriptive for World {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for World {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

impl World {
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn thing(&self, id: u64) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.id() == id)
    }

    pub fn thing_mut(&mut self, id: u64) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.id() == id)
    }

    pub fn area(&self, id: u64) -> Option<&Area> {
        self.areas.iter().find(|area| area.id() == id)
    }

    pub fn find_areas(&self, query: &str) -> Vec<&Area> {
        self.areas.iter()
            .filter(|area| area.name() == query)
            .collect()
    }

    pub fn find_area(&self, key: &str) -> Option<&Area> {
        self.areas.iter().find(|area| area.key().is_some_and(|k| k == key))
    }

    pub fn find_things(&self, query: &str) -> Vec<&Thing> {
        self.things.iter()
            .filter(|thing| thing.name() == query)
            .collect()
    }

    pub fn find_thing(&self, key: &str) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.key().is_some_and(|k| k == key))
    }

    pub fn find_thing_mut(&mut self, key: &str) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.key().is_some_and(|k| k == key))
    }

    pub fn spawn_thing(&mut self, mut thing: ThingBuilder, area_id: ID) -> Result<ID> {
        let area = self.area(area_id).expect("Area not found");
        let thing_id = self.generate_id();

        thing.entity_builder().identity_builder().guid(
            thing_id,
            self.identity.region_id(),
            self.identity.world_id(),
            self.identity.universe_id())?;

        let mut world_editor = World::editor();
        world_editor.add_thing(thing)?;
        let _result = world_editor.modify(self)?;

        Ok(thing_id)
    }
}