use crate::{classes::*, error::*, timeframe::*, builder::*, identity::*, descriptor::*, entity::*, character::*, area::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct World {
    uid: UID,
    frame: Frame,
    descriptor: Descriptor,
    areas: Vec<Area>,
    routes: Vec<Route>,
    things: Vec<Thing>,
    next_id: ID,
}

#[derive(Clone, Copy, Debug)]
pub enum WorldField {
    Identity,
    Frame,
    Descriptor,
    Areas,
    Routes,
    Things
}

impl Fields for WorldField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Frame => &Self::FIELD_FRAME,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Areas => &Self::FIELD_AREAS,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Things => &Self::FIELD_THINGS
        }
    }
}

impl Class for WorldField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl WorldField {
    const CLASS_ID: ClassID = ClassIdent::World as ClassID;
    const CLASSNAME: &'static str = "World";
    const FIELDNAME_IDENTITY: &'static str = "identity";
    const FIELDNAME_FRAME: &'static str = "frame";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_AREAS: &'static str = "areas";
    const FIELDNAME_ROUTES: &'static str = "routes";
    const FIELDNAME_THINGS: &'static str = "things";

    const FIELD_IDENTITY: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_IDENTITY, FieldValueType::Model);
    const FIELD_FRAME: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_FRAME, FieldValueType::UnsignedInteger);
    const FIELD_DESCRIPTOR: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model);
    const FIELD_AREAS: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_AREAS, FieldValueType::ObjectArray);
    const FIELD_ROUTES: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_ROUTES, FieldValueType::ObjectArray);
    const FIELD_THINGS: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, Self::FIELDNAME_THINGS, FieldValueType::ObjectArray);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    frame: Option<Frame>,
    descriptor: Option<DescriptorBuilder>,
    areas: Vec<AreaBuilder>,
    routes: Vec<RouteBuilder>,
    things: Vec<ThingBuilder>,
    next_id: ID,
}

impl Builder for WorldBuilder {
    type ModelType = World;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            frame: None,
            descriptor: None,
            areas: Vec::new(),
            routes: Vec::new(),
            things: Vec::new(),
            next_id: 2 // universe is always 1
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
        let identity = Creation::try_assign(&mut self.identity, WorldField::Identity)?;
        let frame = Self::try_assign_value(&mut self.frame, WorldField::Frame)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, WorldField::Descriptor)?;

        let mut next_id = self.generate_id();
        let (universe_id, world_id) = {
            (identity.universe_id(), identity.world_id())
        };

        // set IDs for areas
        for area in &mut self.areas {
            let class_id = area.class_id();
            let identity_builder = area.identity_builder();
            identity_builder.universe_id(universe_id)?;
            identity_builder.world_id(world_id)?;
            identity_builder.class_id(class_id)?;
            identity_builder.id(next_id)?;
            next_id += 1;
        }

        // set IDs for things
        for thing in &mut self.things {
            let class_id = thing.class_id();
            let identity_builder = thing.entity_builder().identity_builder();
            identity_builder.universe_id(universe_id)?;
            identity_builder.world_id(world_id)?;
            identity_builder.class_id(class_id)?;
            identity_builder.id(next_id)?;
            next_id += 1;
        }

        self.next_id = next_id;

        let world = World {
            uid: identity.into_uid(),
            frame,
            descriptor,
            areas: Creation::assign_vec(&mut self.areas)?,
            routes: Creation::assign_vec(&mut self.routes)?,
            things: Creation::assign_vec(&mut self.things)?,
            next_id: self.next_id + 1,
        };

        Ok(Creation::new(self, world))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        if self.descriptor.is_some() {
            original.descriptor = Creation::assign(&mut self.descriptor)?;
        }
        if let Some(frame) = self.frame {
            original.frame = frame;
        }

        let (universe_id, world_id) = {
            let identity = Identity::from_uid(original.uid());
            (identity.universe_id(), identity.world_id())
        };

        // build identities
        // todo: use appropriate class ids for things
        Self::build_local_identities(original, &mut self.areas, universe_id, world_id)?;
        Self::build_local_identities(original, &mut self.routes, universe_id, world_id)?;
        Self::build_local_identities(original, &mut self.things, universe_id, world_id)?;

        Creation::modify_vec(&mut self.areas, &mut original.areas)?;
        Creation::modify_vec(&mut self.routes, &mut original.routes)?;
        Creation::modify_vec(&mut self.things, &mut original.things)?;

        Ok(Modification::new(self, Vec::new()))
    }

    fn sync_modify(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        self.modify(world)
    }

    fn class_id(&self) -> ClassID {
        WorldField::class_id()
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

    fn build_local_identities(
        original: &mut World,
        builders: &mut Vec<impl BuildableIdentity>,
        universe_id: UniverseID, world_id: WorldID
    ) -> Result<()> {
        for builder in builders {
            if builder.has_identity() {
                return Ok(())
            }

            let class_id = builder.class_id();
            let identity_builder = builder.identity_builder();
            identity_builder.universe_id(universe_id)?;
            identity_builder.world_id(world_id)?;
            identity_builder.class_id(class_id)?;
            identity_builder.id(original.generate_id())?;
        }
        Ok(())
    }

    pub fn frame(&mut self, frame: Frame) -> Result<()> {
        self.frame = Some(frame);
        Ok(())
    }

}

impl Built for World {
    type BuilderType = WorldBuilder;
}

impl Identifiable for World {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for World {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl World {
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn thing(&self, uid: UID) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.uid() == uid)
    }

    pub fn thing_mut(&mut self, uid: UID) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.uid() == uid)
    }

    pub fn area(&self, uid: UID) -> Option<&Area> {
        self.areas.iter().find(|area| area.uid() == uid)
    }

    pub fn area_mut(&mut self, uid: UID) -> Option<&mut Area> {
        self.areas.iter_mut().find(|area| area.uid() == uid)
    }

    pub fn find_areas(&self, query: &str) -> Vec<&Area> {
        self.areas.iter()
            .filter(|area| area.name() == query)
            .collect()
    }

    pub fn find_area(&self, key: &str) -> Option<&Area> {
        self.areas.iter().find(|area| area.key().is_some_and(|k| k == key))
    }

    pub fn things(&self) -> &Vec<Thing> {
        &self.things
    }

    pub fn things_mut(&mut self) -> &mut Vec<Thing> {
        &mut self.things
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

    pub fn spawn_thing(&mut self, mut thing: ThingBuilder, area_uid: UID) -> Result<UID> {
        let _area = self.area(area_uid).expect("Area not found");
        let thing_id = self.generate_id();
        let world_identity = self.uid().into_identity();

        let identity_builder = thing.entity_builder().identity_builder();
        identity_builder
            .universe_id(world_identity.universe_id())?
            .world_id(world_identity.world_id())?
            .class_id(ClassID::MAX)? //todo: determine class id from builder
            .id(thing_id)?;
        let uid = identity_builder.get_uid()?;

        let mut world_editor = World::editor();
        world_editor.add_thing(thing)?;
        let _result = world_editor.modify(self)?;

        Ok(uid)
    }
}