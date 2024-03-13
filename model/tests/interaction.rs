#[cfg(test)]
mod tests {
    use bincode::de;
    use asmov_else_model::{self as model, testing, *};
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
            let mut cortex_creator = model::Cortex::creator();
            cortex_creator.routine_lobe({
                let mut routine_lobe_creator = model::RoutineLobe::creator();
                routine_lobe_creator
                    .routine_uid(0).unwrap() //todo: model crate should have an enum of IDs from behavior crate
                    .routine_awareness(Awareness::Conscious).unwrap();
                routine_lobe_creator
            }).unwrap();
            cortex_creator
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

        let thing_id = WorldAction::spawn_thing(&mut world, character_creator.thing_builder()).unwrap().0;
        let thing = world.thing(thing_id).unwrap();

        assert_eq!("A gray cat", thing.description().unwrap());
        dbg!("Thing ID: {}", thing.uid().into_identity().to_base58());
    }

    #[test]
    fn test_manual_building() {
        let mut world = testing::create_world();

        let cathouse_uid = world.find_area(testing::CAT_HOUSE).unwrap().uid();

        let mut gray_cat = model::Character::creator();
        gray_cat.cortex({
            let mut cortex_creator = model::Cortex::creator();
            cortex_creator.routine_lobe({
                let mut routine_lobe_creator = model::RoutineLobe::creator();
                routine_lobe_creator
                    .routine_uid(0).unwrap() //todo: model crate should have an enum of IDs from behavior crate
                    .routine_awareness(Awareness::Conscious).unwrap();
                routine_lobe_creator
            }).unwrap();
            cortex_creator
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

        let gray_cat_id = model::WorldAction::spawn_thing(&mut world, gray_cat.thing_builder()).unwrap().0;
        let gray_cat = world.thing(gray_cat_id).unwrap();

        assert_eq!("Cat", gray_cat.name());

        let result: Vec<_> = world.things().iter()
            .filter(|thing| thing.name() == "Cat")
            .collect();
 
        let gray_cat = result.first().unwrap();

        assert_eq!("A gray cat", gray_cat.description().unwrap());

        // test simple mutation
        let mut world_editor = model::World::editor();
        let mut gray_cat_editor = world.find_thing("gray_cat")
            .unwrap()
            .edit_self();
        gray_cat_editor.entity_builder().descriptor_builder()
            .description(s!("A slightly gray cat")).unwrap();
        world_editor.edit_thing(gray_cat_editor).unwrap();
        world_editor.modify(&mut world).unwrap();

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
        world_editor.modify(&mut world).unwrap();

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
}
