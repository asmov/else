#[cfg(test)]
mod tests {
    use bincode::de;
    use elsezone_model::{self as model, testing, *};
    use model::testing::DOG_HOUSE;

    #[test]
    fn test_create_world() {
        let world = testing::create_world();
        dbg!(&world);

        assert_eq!("Cat House", world.find_area(testing::CAT_HOUSE).unwrap().name());
        assert_eq!("Black Cat", world.find_thing("black_cat").unwrap().name());
    }

    #[test]
    fn test_spawn_thing() {
        let mut world = testing::create_world();

        let area = world.find_area(testing::CAT_HOUSE).unwrap();

        let mut character_creator = model::Character::creator();
        character_creator.cortex({
            let mut routine_cortex_creator = model::RoutineCortexBuilder::creator();
            routine_cortex_creator.routine_uid(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
            routine_cortex_creator.routine_awareness(Awareness::Conscious).unwrap();
            routine_cortex_creator.cortex_builder()
        }).unwrap();
        character_creator.entity({
            let mut entity_creator = model::Entity::creator();
            entity_creator.descriptor({
                let mut descriptor_creator = model::Descriptor::creator();
                descriptor_creator.key(s!("gray_cat")).unwrap();
                descriptor_creator.name(s!("Gray Cat")).unwrap();
                descriptor_creator.description(s!("A gray cat")).unwrap();
                descriptor_creator
            }).unwrap();
            entity_creator.location(Location::Area(area.uid())).unwrap();
            entity_creator
        }).unwrap();

        let thing_id = world.spawn_thing(character_creator.thing_builder()).unwrap().0;
        let thing = world.thing(thing_id).unwrap();

        assert_eq!("A gray cat", thing.description().unwrap());
        dbg!("Thing ID: {}", thing.uid().into_identity().id_to_string());
    }

    #[test]
    fn test_manual_building() {
        let mut world = testing::create_world();

        let cathouse_uid = world.find_area(testing::CAT_HOUSE).unwrap().uid();

        let mut gray_cat = model::Character::creator();
        gray_cat.cortex({
            let mut routine_cortex_creator = model::RoutineCortexBuilder::creator();
            routine_cortex_creator.routine_uid(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
            routine_cortex_creator.routine_awareness(Awareness::Conscious).unwrap();
            routine_cortex_creator.cortex_builder()
        }).unwrap();
        gray_cat.entity({
            let mut entity = model::Entity::creator();
            entity.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor
                    .key(s!("gray_cat")).unwrap()
                    .name(s!("Cat")).unwrap()
                    .description(s!("A gray cat")).unwrap();
                descriptor
            }).unwrap();
            entity.location(Location::Area(cathouse_uid)).unwrap();
            entity
        }).unwrap();

        let gray_cat_id = world.spawn_thing(gray_cat.thing_builder()).unwrap().0;
        let gray_cat = world.thing(gray_cat_id).unwrap();

        assert_eq!("Cat", gray_cat.name());

        let result = world.find_things("Cat");
        let gray_cat = result.first().unwrap();

        assert_eq!("A gray cat", gray_cat.description().unwrap());

        // test simple mutation

        let mut gray_cat = world.find_thing_mut("gray_cat").unwrap();

        let mut character_editor = Character::editor();
        character_editor.entity_builder().descriptor_builder()
            .description(s!("A slightly gray cat")).unwrap();
        character_editor.thing_builder().modify(&mut gray_cat).unwrap();

        let gray_cat = world.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", gray_cat.description().unwrap());
    }

    #[test]
    fn test_thing_movement() {
        let mut world = testing::create_world();

        let black_cat_uid = world.find_thing(testing::BLACK_CAT).unwrap().uid();
        let backyard_uid = world.find_area(testing::BACKYARD).unwrap().uid();

        // move 'black_cat' from 'cat_house' to 'backyard'.
        let mut black_cat_editor = world
            .find_thing(testing::BLACK_CAT).unwrap()
            .edit_self();
        black_cat_editor.entity_builder()
            .location(Location::Area(backyard_uid)).unwrap();

        let mut world_editor = World::editor();
        world_editor.edit_thing(black_cat_editor).unwrap();
        dbg!(world_editor.modify(&mut world).unwrap());

        // confirm the change in location for 'black_cat'
        let new_location_uid = world
            .find_thing(testing::BLACK_CAT).unwrap()
            .location()
            .uid();

        assert_eq!(new_location_uid, backyard_uid);

        // confirm that 'backyard' has 'black_cat' as an occupant
        assert!(world
            .find_area(testing::BACKYARD).unwrap()
            .occupant_uids()
            .contains(&black_cat_uid));

        // confirm that 'cat_house' no longer has 'black_cat' as an occupant
        assert!(world
            .find_area(testing::CAT_HOUSE).unwrap()
            .occupant_uids()
            .contains(&black_cat_uid) == false);
    }

    const CAT_TO_DOG_HOUSE: &str = "cat_house_to_dog_house";

    fn create_route(mut world: World) -> World {
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // Point B
        
        // route 'cat_house' to 'dog_house'
        let mut world_editor = World::editor();
        let mut route_cat_to_dog_house = Route::creator();
        route_cat_to_dog_house
            .point_a({
                let mut point_a = Endpoint::creator();
                point_a.end({ 
                    let mut end = End::creator();
                    end
                        .area_identity(IdentityBuilder::from_existing(&point_a, cat_house)).unwrap()
                        .direction(Direction::up()).unwrap()
                        .descriptor({
                            let mut descriptor = Descriptor::creator();
                            descriptor
                                .name(s!("portal_to_cat_house")).unwrap()
                                .description(s!("A portal swirling above")).unwrap();
                            descriptor
                        }).unwrap();
                    end
                }).unwrap();
                point_a.point_builder()
            }).unwrap()
            .point_b({
                let mut point_b = Endpoint::creator();
                point_b.end({ 
                    let mut end = End::creator();
                    end
                        .area_identity(IdentityBuilder::from_existing(&point_b, dog_house)).unwrap()
                        .direction(Direction::up()).unwrap()
                        .descriptor({
                            let mut descriptor = Descriptor::creator();
                            descriptor
                                .name(s!("portal_to_dog_house")).unwrap()
                                .description(s!("A portal swirling above")).unwrap();
                            descriptor
                        }).unwrap();
                    end
                }).unwrap();
                point_b.point_builder()
            }).unwrap()
            .descriptor({
                let mut descriptor = Descriptor::creator();
                descriptor
                    .key(CAT_TO_DOG_HOUSE.to_string()).unwrap()
                    .name(s!("Portal between Cat House and Dog House")).unwrap()
                    .description(s!("A route from the cat house to the dog house")).unwrap();
                descriptor
            }).unwrap();
        world_editor.add_route(route_cat_to_dog_house).unwrap();
        world_editor.modify(&mut world).unwrap();

        world
    }

    #[test]
    pub fn test_create_route() {
        let mut world = testing::create_world();
        world = create_route(world);

        // assertions
        let route_cat_to_dog_house = world.find_route(CAT_TO_DOG_HOUSE).unwrap();
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // Point B

        assert!(cat_house.route_uids().contains(&route_cat_to_dog_house.uid()));
        assert!(dog_house.route_uids().contains(&route_cat_to_dog_house.uid()));

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_a() {
            assert_eq!(endpoint.end().area_uid(), cat_house.uid());
        } else {
            panic!("Route Point A should be an Endpoint");
        }

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_b() {
            assert_eq!(endpoint.end().area_uid(), dog_house.uid());
        } else {
            panic!("Route Point B should be an Endpoint");
        }
    }

    #[test]
    pub fn test_edit_route() {
        let mut world = testing::create_world();
        world = create_route(world);

        let route_cat_to_dog_house = world.find_route(CAT_TO_DOG_HOUSE).unwrap();
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let backyard = world.find_area(testing::BACKYARD).unwrap(); // new Point B
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // old Point B

        // edit route to change Point B from 'dog_house' to 'backyard'
        let mut world_editor = World::editor();
        let mut route_editor = route_cat_to_dog_house.edit_self();
        route_editor.point_b({
            let mut point_creator = Endpoint::creator();
            point_creator
                .end({
                    let mut end_creator = End::creator();
                    end_creator
                        .area_identity(IdentityBuilder::from_existing(&end_creator, backyard)).unwrap()
                        .direction(Direction::down()).unwrap()
                        .descriptor({
                            let mut descriptor_creator = Descriptor::creator();
                            descriptor_creator
                                .name(s!("portal_to_cat_house")).unwrap()
                                .description(s!("A portal swirling below")).unwrap();
                            descriptor_creator
                        }).unwrap();
                    end_creator
                }).unwrap();
            point_creator.point_builder()
        }).unwrap();
        world_editor.edit_route(route_editor).unwrap();
        world_editor.modify(&mut world).unwrap();
    }
}
