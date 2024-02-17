#[cfg(test)]
mod tests {
    use elsezone_model::{self as model, testing, *};

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
            routine_cortex_creator.routine_id(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
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
            entity_creator
        }).unwrap();

        let thing_id = world.spawn_thing(character_creator.thing_builder(), area.uid()).unwrap();
        let thing = world.thing(thing_id).unwrap();

        assert_eq!("A gray cat", thing.description().unwrap());
    }

    #[test]
    fn test_manual_building() {
        let mut world = testing::create_world();

        let litterbox_id = world.find_area(testing::CAT_HOUSE).unwrap()
            .uid();

        let mut gray_cat = model::Character::creator();
        gray_cat.cortex({
            let mut routine_cortex_creator = model::RoutineCortexBuilder::creator();
            routine_cortex_creator.routine_id(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
            routine_cortex_creator.routine_awareness(Awareness::Conscious).unwrap();
            routine_cortex_creator.cortex_builder()
        }).unwrap();
        gray_cat.entity({
            let mut entity = model::Entity::creator();
            entity.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("gray_cat")).unwrap();
                descriptor.name(s!("Cat")).unwrap();
                descriptor.description(s!("A gray cat")).unwrap();
                descriptor
            }).unwrap();
            entity
        }).unwrap();

        let gray_cat_id = world.spawn_thing(gray_cat.thing_builder(), litterbox_id).unwrap();
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
}
