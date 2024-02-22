use crate::{area::*, builder::*, character::*, entity::*, error::*, identity::*, location::*, route::*, timeframe::*};
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

impl Keyed for World {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
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

impl Built for World {
    type BuilderType = WorldBuilder;
}

impl World {
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn thing(&self, uid: UID) -> Result<&Thing> {
        self.things.iter().find(|thing| thing.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: "Thing", uid})
    }

    pub fn thing_mut(&mut self, uid: UID) -> Result<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: "Thing", uid})
    }

    pub fn area(&self, uid: UID) -> Result<&Area> {
        self.areas.iter().find(|area| area.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: "Area", uid})
    }

    pub fn area_mut(&mut self, uid: UID) -> Result<&mut Area> {
        self.areas.iter_mut().find(|area| area.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound{model: "Area", uid})
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

    pub fn spawn_thing(&mut self, mut thing: ThingBuilder) -> Result<(UID, Modification<WorldBuilder>)> {
        let thing_id = self.generate_id();
        let class_id = thing.class_ident().class_id();
        let world_identity = self.uid().into_identity();

        let identity_builder = thing.entity_builder().identity_builder();
        identity_builder
            .universe_id(world_identity.universe_id())?
            .world_id(world_identity.world_id())?
            .class_id(class_id)?
            .id(thing_id)?;
        let uid = identity_builder.get_uid()?;

        let mut world_editor = World::editor();
        world_editor.add_thing(thing)?;
        let modification = world_editor.modify(self)?;
        Ok((uid, modification))
    }
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
            Self::Identity => &Self::FIELD_UID,
            Self::Frame => &Self::FIELD_FRAME,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Areas => &Self::FIELD_AREAS,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Things => &Self::FIELD_THINGS
        }
    }
}

impl Class for WorldField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl WorldField {
    const CLASSNAME: &'static str = "World";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::World as ClassID, Self::CLASSNAME);
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_FRAME: &'static str = "frame";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_AREAS: &'static str = "areas";
    const FIELDNAME_ROUTES: &'static str = "routes";
    const FIELDNAME_THINGS: &'static str = "things";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID);
    const FIELD_FRAME: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_FRAME, FieldValueType::U64);
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_AREAS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_AREAS, FieldValueType::ModelCollection);
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTES, FieldValueType::ModelCollection);
    const FIELD_THINGS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_THINGS, FieldValueType::ModelCollection);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    frame: Option<Frame>,
    descriptor: Option<DescriptorBuilder>,
    areas: Vec<VecOp<AreaBuilder, UID>>,
    routes: Vec<VecOp<RouteBuilder, UID>>,
    things: Vec<VecOp<ThingBuilder, UID>>,
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
        let mut fields_changed = FieldsChanged::new(WorldField::class_ident(), ChangeOp::Create);

        let identity = Build::create(&mut self.identity, &mut fields_changed, WorldField::Identity)?;
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, WorldField::Descriptor)?;
        let frame = Build::create_value(&mut self.frame, &mut fields_changed, WorldField::Frame)?;

        let mut next_id = self.generate_id();
        let (universe_id, world_id) = (identity.universe_id(), identity.world_id());

        // set IDs for areas
        for area_vec_op in &mut self.areas {
            match area_vec_op {
                VecOp::Add(area_builder) => {
                    let class_id = area_builder.class_ident().class_id();
                    let identity_builder = area_builder.identity_builder();
                    identity_builder.universe_id(universe_id)?;
                    identity_builder.world_id(world_id)?;
                    identity_builder.class_id(class_id)?;
                    identity_builder.id(next_id)?;
                    next_id += 1;
                }
                _ => unreachable!("Only Add operations are allowed for areas during creation")
            }
        }

        // set IDs for things
        for thing_vec_op in &mut self.things {
            match thing_vec_op {
                VecOp::Add(thing_builder) => {
                    let class_id = thing_builder.class_ident().class_id();
                    let identity_builder = thing_builder.entity_builder().identity_builder();
                    identity_builder.universe_id(universe_id)?;
                    identity_builder.world_id(world_id)?;
                    identity_builder.class_id(class_id)?;
                    identity_builder.id(next_id)?;
                    next_id += 1;
                }
                _ => unreachable!("Only Add operations are allowed for things during creation")
            }
        }

        self.next_id = next_id;

        let uid = identity.into_uid();
        let areas = Build::assign_vec(&mut self.areas, &mut fields_changed, WorldField::Areas)?;
        let routes = Build::assign_vec(&mut self.routes, &mut fields_changed, WorldField::Routes)?;
        let things = Build::assign_vec(&mut self.things, &mut fields_changed, WorldField::Things)?;
        let next_id = self.next_id + 1;

        let world = World {
            uid,
            frame,
            descriptor,
            areas,
            routes,
            things,
            next_id,
        };

        Ok(Creation::new(self, world))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::new(WorldField::class_ident(), ChangeOp::Modify);

        if self.descriptor.is_some() {
            Build::modify(&mut self.descriptor, &mut original.descriptor, &mut fields_changed, WorldField::Descriptor)?;
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


        let mut areas = self.areas;

        // handle movement of things between locations
        self.things
            .drain(0..)
            .map(|thing_vec_op| {
                match thing_vec_op {
                    VecOp::Remove(_) => {},
                    VecOp::Add(ref thing) | VecOp::Edit(ref thing) => {
                        Self::process_thing_location(thing, &mut areas, original)?
                    }
                }

                Ok(thing_vec_op)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .for_each(|thing_vec_op| self.things.push(thing_vec_op));

        self.areas = areas;

        Build::modify_vec(&mut self.areas, &mut original.areas, &mut fields_changed, WorldField::Areas)?;
        Build::modify_vec(&mut self.routes, &mut original.routes, &mut fields_changed, WorldField::Routes)?;
        Build::modify_vec(&mut self.things, &mut original.things, &mut fields_changed, WorldField::Things)?;

        Ok(Modification::new_old(self, Vec::new()))
    }

    fn sync_modify(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        self.modify(world)
    }

    fn class_ident(&self) -> &'static ClassIdent {
        WorldField::class_ident()
    }
}

impl MaybeIdentifiable for WorldBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
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
        self.areas.push(VecOp::Add(area));
        Ok(())
    }

    fn edit_area(&mut self, area: AreaBuilder) -> Result<()> {
        self.areas.push(VecOp::Edit(area));
        Ok(())
    }

    fn remove_area(&mut self, area_uid: UID) -> Result<()> {
        self.areas.push(VecOp::Remove(area_uid));
        Ok(())
    }
}

impl BuildableRouteVector for WorldBuilder {
    fn add_route(&mut self, route: RouteBuilder) -> Result<()> {
        self.routes.push(VecOp::Add(route));
        Ok(())
    }
}

impl BuildableThingVector for WorldBuilder {
    fn add_thing(&mut self, thing: ThingBuilder) -> Result<()> {
       self.things.push(VecOp::Add(thing)); 
       Ok(())
    }

    fn edit_thing(&mut self, thing: ThingBuilder) -> Result<()> {
        self.things.push(VecOp::Edit(thing));
        Ok(())
    }

    fn remove_thing(&mut self, thing_uid: UID) -> Result<()> {
        self.things.push(VecOp::Remove(thing_uid));
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
        operations: &mut Vec<VecOp<impl BuildableIdentity, UID>>,
        universe_id: UniverseID, world_id: WorldID
    ) -> Result<()> {
        for op in operations {
            match op {
                VecOp::Add(builder) => {
                    if builder.has_identity() {
                        return Ok(())
                    }

                    let class_id = builder.class_ident().class_id();
                    let identity_builder = builder.identity_builder();
                    identity_builder.universe_id(universe_id)?;
                    identity_builder.world_id(world_id)?;
                    identity_builder.class_id(class_id)?;
                    identity_builder.id(original.generate_id())?;
                },
                _ => {},
            }
        }
        Ok(())
    }

    pub fn frame(&mut self, frame: Frame) -> Result<()> {
        self.frame = Some(frame);
        Ok(())
    }

    fn find_area_builder_by_uid(areas: &mut Vec<VecOp<AreaBuilder, UID>>, area_uid: UID) -> Option<&mut VecOp<AreaBuilder, UID>> {
        areas.iter_mut()
            .find(|vec_op| match vec_op {
                VecOp::Add(area_builder) | VecOp::Edit(area_builder) => {
                    area_builder.try_uid()
                        .is_ok_and(|uid| uid == area_uid)
                },
                VecOp::Remove(area_builder_uid) => {
                    area_builder_uid == &area_uid
                }
            })
    }

    pub fn area_build_existing(areas: &mut Vec<VecOp<AreaBuilder, UID>>, existing_area_uid: UID) -> Result<&mut VecOp<AreaBuilder, UID>> {
        // otherwise create it
        let mut area_editor = Area::editor();
        area_editor.identity(IdentityBuilder::editor_from_uid(existing_area_uid))?;
        areas.push(VecOp::Edit(area_editor));

        // find it again
        let current_builder = areas.iter_mut()
            .find(|vec_op| match vec_op {
                VecOp::Edit(area_builder) => {
                    area_builder.try_uid()
                        .is_ok_and(|uid| uid == existing_area_uid)
                },
                _ => false
            })
            .expect("Failed to find newly created AreaBuilder"); 

        Ok(current_builder)
    }

    fn process_thing_location(thing_builder: &ThingBuilder, areas: &mut Vec<VecOp<AreaBuilder, UID>>, existing_world: &mut World) -> Result<()> {
        let entity_builder = match thing_builder.get_entity() {
            Some(entity_builder) => entity_builder,
            None => return Ok(())
        };
        let location = match entity_builder.get_location() {
            Some(location) => location,
            None => return Ok(())
        };

        let thing_uid = thing_builder.try_uid()?;

        if thing_builder.builder_mode() == BuilderMode::Editor {
            Self::remove_thing_from_location(areas, thing_uid, existing_world, location)?;
        }

        match location {
            Location::Area(area_uid) => {
                let existing_area_vec_op = if let Some(existing) = Self::find_area_builder_by_uid(areas, area_uid) {
                    existing
                } else {
                    Self::area_build_existing(areas, area_uid)?
                };

                match existing_area_vec_op {
                    VecOp::Add(area_builder) | VecOp::Edit(area_builder) => {
                        area_builder.add_occupant(thing_uid)?;
                    },
                    VecOp::Remove(_) => {
                        return Err(Error::ModelNotFound{model: "Area", uid: area_uid })
                    } 
                }
            },
            Location::Route(route_uid) => {
                todo!()
            }
        }


        Ok(())
    }

    fn remove_thing_from_location(areas: &mut Vec<VecOp<AreaBuilder, u128>>, thing_uid: UID, existing_world: &mut World, new_location: Location) -> Result<()> {
        let existing_thing = existing_world.thing(thing_uid)?;
        match existing_thing.location() {
            Location::Area(area_uid) => {
                let existing_area_vec_op = if let Some(existing) = Self::find_area_builder_by_uid(areas, area_uid) {
                    existing
                } else {
                    Self::area_build_existing(areas, area_uid)?
                };

                match existing_area_vec_op {
                    VecOp::Add(area_builder) | VecOp::Edit(area_builder) => {
                        area_builder.remove_occupant(thing_uid)?;
                    },
                    VecOp::Remove(_) => {/* nothing to do */} 
                }
            },
            Location::Route(route_uid) => {
                todo!()
            }
        }

        Ok(())
    }


}

