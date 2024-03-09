use serde;
use crate::{area::*, modeling::*, character::*, route::*, timeframe::*, interface::*};

#[derive(Clone, Copy, Debug)]
pub enum WorldField {
    UID,
    Frame,
    Descriptor,
    Areas,
    Routes,
    Things,
    Interfaces
}

impl Fields for WorldField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Frame => &Self::FIELD_FRAME,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Areas => &Self::FIELD_AREAS,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Things => &Self::FIELD_THINGS,
            Self::Interfaces => &Self::FIELD_INTERFACES,
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
    const FIELDNAME_INTERFACES: &'static str = "interfaces";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_FRAME: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_FRAME, FieldValueType::U64);
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_AREAS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_AREAS, FieldValueType::ModelList(AreaField::class_ident_const()));
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTES, FieldValueType::ModelList(RouteField::class_ident_const()));
    const FIELD_THINGS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_THINGS, FieldValueType::ModelList(Thing::class_ident_const()));
    const FIELD_INTERFACES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACES, FieldValueType::ModelList(InterfaceField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldBuilder {
    builder_mode: BuilderMode,
    uid: Option<UID>,
    frame: Option<Frame>,
    descriptor: Option<DescriptorBuilder>,
    areas: Vec<ListOp<AreaBuilder, UID>>,
    routes: Vec<ListOp<RouteBuilder, UID>>,
    things: Vec<ListOp<ThingBuilder, UID>>,
    interfaces: Vec<ListOp<InterfaceBuilder, UID>>,
    next_id: ID,
}

impl Builder for WorldBuilder {
    type BuilderType = Self;
    type ModelType = World;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            uid: None,
            frame: None,
            descriptor: None,
            areas: Vec::new(),
            routes: Vec::new(),
            things: Vec::new(),
            interfaces: Vec::new(),
            next_id: 3, // Universe and World claim 1 and 2 respectively
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

        let uid = Build::create_uid(&mut self.uid, &mut fields_changed, WorldField::UID)?;
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, WorldField::Descriptor)?;
        let frame = Build::create_value(&mut self.frame, &mut fields_changed, WorldField::Frame)?;

        let mut idgen = IdentityGenerator::from_uid(uid, self.next_id);

        // reassign these to self later
        let mut areas = self.areas;
        let mut routes = self.routes;
        let mut things = self.things;

        // build identities
        Self::build_local_identities(&mut areas, &mut idgen)?;
        Self::build_local_identities(&mut routes, &mut idgen)?;
        Self::build_local_identities(&mut things, &mut idgen)?;

        // reassign now that they're ready
        self.areas = areas;
        self.routes = routes;
        self.things = things;

        let areas = Build::create_vec(&mut self.areas, &mut fields_changed, WorldField::Areas)?;
        let routes = Build::create_vec(&mut self.routes, &mut fields_changed, WorldField::Routes)?;
        let things = Build::create_vec(&mut self.things, &mut fields_changed, WorldField::Things)?;
        let interfaces = Build::create_vec(&mut self.interfaces, &mut fields_changed, WorldField::Interfaces)?;

        self.next_id = idgen.next_id();
        let next_id = self.next_id;

        let world = World {
            uid,
            frame,
            descriptor,
            areas,
            routes,
            things,
            interfaces,
            next_id,
        };

        Ok(Creation::new(self, world))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::new(WorldField::class_ident(), ChangeOp::Modify);

        self.next_id = existing.next_id;

        Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, WorldField::Descriptor)?;
        Build::modify_value(&self.frame, &mut existing.frame, &mut fields_changed, WorldField::Frame)?;

        let mut idgen = IdentityGenerator::from_uid(existing.uid(), self.next_id);

        // reassign these to self later
        let mut areas = self.areas;
        let mut routes = self.routes;
        let mut things = self.things;

        // build identities
        Self::build_local_identities(&mut areas, &mut idgen)?;
        Self::build_local_identities(&mut routes, &mut idgen)?;
        Self::build_local_identities(&mut things, &mut idgen)?;

        // handle movement of things between locations
        things.iter_mut()
            .map(|thing_list_op| {
                match thing_list_op {
                    ListOp::Remove(_) => {},
                    ListOp::Add(ref thing) | ListOp::Edit(ref thing) => {
                        Self::process_thing_location(thing, &mut areas, existing)?
                    }
                }

                Ok(thing_list_op)
            })
            .collect::<Result<Vec<_>>>()?;


        Self::link_routes_to_areas(&mut routes, &mut areas, existing)?;

        self.areas = areas;
        self.routes = routes;
        self.things = things;

        Build::modify_vec(&mut self.areas, &mut existing.areas, &mut fields_changed, WorldField::Areas)?;
        Build::modify_vec(&mut self.routes, &mut existing.routes, &mut fields_changed, WorldField::Routes)?;
        Build::modify_vec(&mut self.things, &mut existing.things, &mut fields_changed, WorldField::Things)?;
        Build::modify_vec(&mut self.interfaces, &mut existing.interfaces, &mut fields_changed, WorldField::Interfaces)?;

        self.next_id = idgen.next_id();
        existing.next_id = self.next_id;

        Ok(Modification::new(self, fields_changed))
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

impl BuildableUID for WorldBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.uid = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.uid.as_ref()
    }
}

impl BuildableDescriptor for WorldBuilder {
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

impl BuildableAreaVector for WorldBuilder {
    fn add_area(&mut self, area: AreaBuilder) -> Result<&mut Self> {
        self.areas.push(ListOp::Add(area));
        Ok(self)
    }

    fn edit_area(&mut self, area: AreaBuilder) -> Result<&mut Self> {
        self.areas.push(ListOp::Edit(area));
        Ok(self)
    }

    fn remove_area(&mut self, area_uid: UID) -> Result<&mut Self> {
        self.areas.push(ListOp::Remove(area_uid));
        Ok(self)
    }
}

impl BuildableRouteVector for WorldBuilder {
    fn add_route(&mut self, route: RouteBuilder) -> Result<&mut Self> {
        self.routes.push(ListOp::Add(route));
        Ok(self)
    }
    
    fn edit_route(&mut self, route: RouteBuilder) -> Result<&mut Self> {
        self.routes.push(ListOp::Edit(route));
        Ok(self)
    }
    
    fn remove_route(&mut self, route_uid: UID) -> Result<&mut Self> {
        self.routes.push(ListOp::Remove(route_uid));
        Ok(self)
    }
}

impl BuildableThingList for WorldBuilder {
    fn get_thing_listops_mut(&mut self) -> &mut Vec<ListOp<ThingBuilder, UID>> {
        self.things.as_mut()
    }
    
    fn get_thing_listops(&self) -> &Vec<ListOp<ThingBuilder, UID>> {
        self.things.as_ref()
    }
}

impl BuildableInterfaceList for WorldBuilder {
    fn add_interface(&mut self, interface: InterfaceBuilder) -> Result<&mut Self> {
        self.interfaces.push(ListOp::Add(interface));
        Ok(self)
    }

    fn edit_interface(&mut self, interface: InterfaceBuilder) -> Result<&mut Self> {
        self.interfaces.push(ListOp::Edit(interface));
        Ok(self)
    }

    fn remove_interface(&mut self, interface_uid: UID) -> Result<&mut Self> {
        self.interfaces.push(ListOp::Remove(interface_uid));
        Ok(self)
    }
}

impl SynchronizedDomainBuilder<World> for WorldBuilder {
    fn synchronize(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        self.modify(world)
    }
}

impl WorldBuilder {
    /*pub fn downlink(&mut self, interface_uid: UID, character_uid: UID) -> Result<Self> {
        let mut interface = Interface::editor();
        interface.identity(UID::editor_from_uid(interface_uid))?;
        let mut character = Character::editor();
        character.identity(UID::editor_from_uid(character_uid))?;
        todo
    }*/

    //todo: generate_uid(..) { IdentityGenerator::generate_uid(&mut self, universe_id: UID, world_id: UID, class_id: ClassID) -> UID }
    fn generate_id(&mut self) -> ID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    fn link_routes_to_areas(
        route_ops: &mut Vec<ListOp<RouteBuilder, UID>>,
        area_ops: &mut Vec<ListOp<AreaBuilder, UID>>,
        world: &World 
    ) -> Result<()> {
        for route_op in route_ops {
            match route_op {
                ListOp::Add(route_builder) => {
                    for area_uid in route_builder.creation_area_uids()? {
                        let area_builder = match Self::find_area_builder_by_uid(area_ops, area_uid) {
                            Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                            Some(ListOp::Remove(uid)) => return Err(Error::IllegalRemoveOp{
                                model: AreaField::classname(), uid: *uid, context: "WorldBuilder::link_routes_to_areas"
                            }),
                            None => Self::area_builder_from_existing(area_ops, area_uid)? 
                        };

                        area_builder.add_route_uid(route_builder.try_uid()?)?;
                    }
                },
                ListOp::Edit(route_builder) => {
                    // diff between the area uids of the existing route and the edited route
                    let route_uid = route_builder.try_uid()?;
                    let existing_route = world.route(route_builder.try_uid()?)?;
                    let existing_area_uids = existing_route.area_uids();
                    let edited_area_uids = route_builder.modification_area_uids(&existing_route)?;

                    let removed_area_uids = existing_area_uids.iter()
                        .filter(|uid| !edited_area_uids.contains(uid))
                        .collect::<Vec<_>>();

                    for area_uid in removed_area_uids {
                        let area_builder = match Self::find_area_builder_by_uid(area_ops, *area_uid) {
                            Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                            Some(ListOp::Remove(uid)) => return Err(Error::ListOpRace{
                                op: "Edit", model: AreaField::classname(), uid: *uid, whiled: "removed"
                            }),
                            None => Self::area_builder_from_existing(area_ops, *area_uid)? 
                        };

                        area_builder.remove_route_uid(route_uid)?;
                    }

                    let added_area_uids = edited_area_uids.iter()
                        .filter(|uid| !existing_area_uids.contains(uid))
                        .collect::<Vec<_>>();

                    for area_uid in added_area_uids {
                        let area_builder = match Self::find_area_builder_by_uid(area_ops, *area_uid) {
                            Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                            Some(ListOp::Remove(uid)) => return Err(Error::ListOpRace{
                                op: "Edit", model: AreaField::classname(), uid: *uid, whiled: "removed"
                            }),
                            None => Self::area_builder_from_existing(area_ops, *area_uid)? 
                        };

                        area_builder.add_route_uid(route_uid)?;
                    }
                },
                ListOp::Remove(route_uid) => {
                    let existing_route = world.route(*route_uid)?;

                    for area_uid in existing_route.area_uids() {
                        let area_builder = match Self::find_area_builder_by_uid(area_ops, area_uid) {
                            Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                            Some(ListOp::Remove(uid)) => return Err(Error::ListOpRace{
                                op: "Edit", model: AreaField::classname(), uid: *uid, whiled: "removed"
                            }),
                            None => Self::area_builder_from_existing(area_ops, area_uid)? 
                        };

                        area_builder.remove_route_uid(*route_uid)?;
                    }
                },
            }
        }

        Ok(())
    }

    fn build_local_identities(
        operations: &mut Vec<ListOp<impl BuildableUID, UID>>,
        idgen: &mut IdentityGenerator
    ) -> Result<()> {
        for op in operations {
            match op {
                ListOp::Add(builder) => {
                    if builder.has_uid() {
                        continue;
                    }

                    let uid = idgen.next_uid(builder.class_ident().class_id());
                    builder.uid(uid).unwrap();
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

    fn find_area_builder_by_uid(areas: &mut Vec<ListOp<AreaBuilder, UID>>, area_uid: UID) -> Option<&mut ListOp<AreaBuilder, UID>> {
        areas.iter_mut()
            .find(|list_op| match list_op {
                ListOp::Add(area_builder) | ListOp::Edit(area_builder) => {
                    area_builder.try_uid()
                        .is_ok_and(|uid| uid == area_uid)
                },
                ListOp::Remove(area_builder_uid) => {
                    area_builder_uid == &area_uid
                }
            })
    }

    pub fn area_builder_from_existing(areas: &mut Vec<ListOp<AreaBuilder, UID>>, existing_area_uid: UID) -> Result<&mut AreaBuilder> {
        // otherwise create it
        let mut area_editor = Area::editor();
        area_editor.uid(existing_area_uid)?;
        areas.push(ListOp::Edit(area_editor));

        // find it again
        let current_builder = areas.iter_mut()
            .find(|list_op| match list_op {
                ListOp::Edit(area_builder) => {
                    area_builder.try_uid()
                        .is_ok_and(|uid| uid == existing_area_uid)
                },
                _ => false
            })
            .expect("Failed to find newly created AreaBuilder"); 

        match current_builder {
            ListOp::Edit(builder) => Ok(builder),
            _ => unreachable!()
        }
    }

    fn process_thing_location(thing_builder: &ThingBuilder, areas: &mut Vec<ListOp<AreaBuilder, UID>>, existing_world: &mut World) -> Result<()> {
        let entity_builder = match thing_builder.get_entity_builder() {
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
                let area_builder = match Self::find_area_builder_by_uid(areas, area_uid) {
                    Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                    Some(ListOp::Remove(_)) => return Err(Error::ModelNotFound{model: "Area", uid: area_uid }),
                    None => Self::area_builder_from_existing(areas, area_uid)?,
                };
                
                area_builder.add_occupant_uid(thing_uid)?;
            },
            Location::Route(route_uid) => {
                todo!()
            }
        }


        Ok(())
    }

    fn remove_thing_from_location(areas: &mut Vec<ListOp<AreaBuilder, u128>>, thing_uid: UID, existing_world: &mut World, new_location: Location) -> Result<()> {
        let existing_thing = existing_world.thing(thing_uid)?;
        match existing_thing.location() {
            Location::Area(area_uid) => {
                let area_builder = match Self::find_area_builder_by_uid(areas, area_uid) {
                    Some(ListOp::Add(area_builder)) | Some(ListOp::Edit(area_builder)) => area_builder,
                    Some(ListOp::Remove(_)) => return Err(Error::ModelNotFound{model: "Area", uid: area_uid }),
                    None => Self::area_builder_from_existing(areas, area_uid)?,
                };
                
                area_builder.remove_occupant_uid(thing_uid)?;

            },
            Location::Route(route_uid) => {
                todo!()
            }
        }

        Ok(())
    }


}

