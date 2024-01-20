#[macro_export]
macro_rules! s {
    ($s:literal) => { String::from($s) };
    ($s:ident) => { $s.to_string() };
}

pub mod error;
pub mod builder;
pub mod identity;
pub mod descriptor;
pub mod inventory;
pub mod composition;
pub mod entity;
pub mod something;
pub mod character;
pub mod item;
pub mod thing;
pub mod area;
pub mod route;
pub mod access;
pub mod zone;

pub use error::*;
pub use builder::*;
pub use identity::*;
pub use descriptor::*;
pub use inventory::*;
pub use composition::*;
pub use entity::*;
pub use something::*;
pub use character::*;
pub use item::*;
pub use thing::*;
pub use area::*;
pub use route::*;
pub use zone::*;

pub mod testing {
    use crate::s;
    use crate::{self as model, *};

    pub fn create_zone() -> Zone {
        let mut zone_creator = model::Zone::creator();

        zone_creator.identity_builder().guid(0, 0, 1, 1).unwrap();

        zone_creator.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("unit_test_zone")).unwrap();
                descriptor.name(s!("Unit Test Zone")).unwrap();
                descriptor.description(s!("A zone where all models are equally buggy")).unwrap();
                descriptor.notes(s!("Testing only")).unwrap();
                descriptor
        }).unwrap();

        zone_creator.add_area({
            let mut area_creator = model::Area::creator();
            area_creator.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("backyard")).unwrap();
                descriptor.name(s!("Backyard")).unwrap();
                descriptor.description(s!("A well-manicured grass yard with orange trees lined along its border.")).unwrap();
                descriptor
            }).unwrap();
            area_creator
        }).unwrap();

        zone_creator.add_area({
            let mut area_creator = model::Area::creator();
            area_creator.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("cat_house")).unwrap();
                descriptor.name(s!("Cat House")).unwrap();
                descriptor.description(s!("A large playhouse for cats with multiple floors, windows, etc.")).unwrap();
                descriptor
            }).unwrap();
            area_creator
        }).unwrap();

        zone_creator.add_area({
            let mut area_creator = model::Area::creator();
            area_creator.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("dog_house")).unwrap();
                descriptor.name(s!("Dog House")).unwrap();
                descriptor.description(s!("A medium-sized dog house. It's just big enough for a single pet to lay.")).unwrap();
                descriptor
            }).unwrap();
            area_creator
        }).unwrap();

        zone_creator.add_thing({
            let mut character_creator = model::Character::creator();
            let descriptor_creator = character_creator.entity_builder().descriptor_builder();
            descriptor_creator.key(s!("black_cat")).unwrap();
            descriptor_creator.name(s!("Black Cat")).unwrap();
            descriptor_creator.description(s!("A cat with a shiny black coat")).unwrap();
            character_creator.thing_builder()
        }).unwrap();

        let mut zone = zone_creator.create().unwrap();
        let mut zone_editor = Zone::editor();

        zone_editor.add_route({
            let area_a = zone.find_area("backyard").unwrap();
            let area_b = zone.find_area("dog_house").unwrap();
            let mut route_creator = Route::creator();
            route_creator.descriptor({
                let mut descriptor_creator = Descriptor::creator();
                descriptor_creator.key(s!("backyard_and_dog_house")).unwrap();
                descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
                descriptor_creator
            }).unwrap();
            route_creator.point_a({
                let mut endpoint_creator = route::Endpoint::creator();
                endpoint_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_dog_house_point_a")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
                    descriptor_creator
                }).unwrap();
                endpoint_creator.area_identity(area_a.identity().to_creator()).unwrap();
                endpoint_creator.direction(Direction::West).unwrap();
                endpoint_creator.point_builder()
            }).unwrap();
            route_creator.point_b({
                let mut endpoint_creator = route::Endpoint::creator();
                endpoint_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_dog_house_point_b")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
                    descriptor_creator
                }).unwrap();
                endpoint_creator.area_identity(area_b.identity().to_creator()).unwrap();
                endpoint_creator.direction(Direction::East).unwrap();
                endpoint_creator
            }).unwrap();
            route_creator
        }).unwrap();

        let _result = zone_editor.modify(&mut zone).unwrap();
        
        zone
    }

}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::{self as model, *};
    use crate::s;

    #[test]
    fn test_create_zone() {
        let zone = testing::create_zone();
        dbg!(&zone);

        assert_eq!("Cat House", zone.find_area("cat_house").unwrap().name());
        assert_eq!("Black Cat", zone.find_thing("black_cat").unwrap().name());
    }

    #[test]
    fn test_spawn_thing() {
        let mut zone = testing::create_zone();

        let area = zone.find_area("cat_house").unwrap();

        let mut character_creator = model::Character::creator();
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

        let thing_id = zone.spawn_thing(character_creator.thing_builder(), area.id()).unwrap();
        let thing = zone.thing(thing_id).unwrap();

        assert_eq!("A gray cat", thing.description().unwrap());
    }

    #[test]
    fn test_manual_building() {
        let mut zone = testing::create_zone();

        let litterbox_id = zone.find_area("cat_house")
            .unwrap()
            .id();

        let mut gray_cat = model::Character::creator();
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

        let gray_cat_id = zone.spawn_thing(gray_cat.thing_builder(), litterbox_id).unwrap();
        let gray_cat = zone.thing(gray_cat_id).unwrap();

        assert_eq!("Cat", gray_cat.name());

        let result = zone.find_things("Cat");
        let gray_cat = result.first().unwrap();

        assert_eq!("A gray cat", gray_cat.description().unwrap());

        // test simple mutation

        let gray_cat = zone.find_thing_mut("gray_cat").unwrap();

        let mut gray_cat_descriptor_editor = Descriptor::editor();
        gray_cat_descriptor_editor.description(s!("A slightly gray cat")).unwrap();
        gray_cat_descriptor_editor.modify(gray_cat.descriptor_mut()).unwrap();

        let gray_cat_editor = Entity::editor();
        gray_cat_editor.modify(gray_cat.entity_mut()).unwrap();

        let gray_cat = zone.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", gray_cat.description().unwrap());
    }
}
