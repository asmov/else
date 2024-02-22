use crate::s;
use crate::{self as model, *};
use bincode;
use bytes;

use self::routine::RoutineCortexBuilder;

pub const BACKYARD: &'static str = "backyard";
pub const DOG_HOUSE: &'static str = "dog_house";
pub const CAT_HOUSE: &'static str = "cat_house";
pub const BLACK_CAT: &'static str = "black_cat";

/// Creates a world for unit and integration testing.
/// 
/// Areas:  
/// dog_house <--> backyard <--> cat_house
/// 
/// Occupants:
/// - black_cat: spawns in cat_house
pub fn create_world() -> World {
    let mut world_creator = model::World::creator();

    world_creator.identity_builder()
        .universe_id(UniverseID::MAX).unwrap()
        .world_id(1).unwrap()
        .class_id(WorldField::class_id()).unwrap()
        .id(2).unwrap();

    world_creator.frame(0).unwrap();

    world_creator.descriptor({
            let mut descriptor = model::Descriptor::creator();
            descriptor.key(s!("unit_test_world")).unwrap();
            descriptor.name(s!("Unit Test World")).unwrap();
            descriptor.description(s!("A world where all models are equally buggy")).unwrap();
            descriptor.notes(s!("Testing only")).unwrap();
            descriptor
    }).unwrap();

    world_creator.add_area({
        let mut area_creator = model::Area::creator();
        area_creator.descriptor({
            let mut descriptor = model::Descriptor::creator();
            descriptor.key(s!(BACKYARD)).unwrap();
            descriptor.name(s!("Backyard")).unwrap();
            descriptor.description(s!("A well-manicured grass yard with orange trees lined along its border.")).unwrap();
            descriptor
        }).unwrap();
        area_creator
    }).unwrap();

    world_creator.add_area({
        let mut area_creator = model::Area::creator();
        area_creator.descriptor({
            let mut descriptor = model::Descriptor::creator();
            descriptor.key(s!(CAT_HOUSE)).unwrap();
            descriptor.name(s!("Cat House")).unwrap();
            descriptor.description(s!("A large playhouse for cats with multiple floors, windows, etc.")).unwrap();
            descriptor
        }).unwrap();
        area_creator
    }).unwrap();

    world_creator.add_area({
        let mut area_creator = model::Area::creator();
        area_creator.descriptor({
            let mut descriptor = model::Descriptor::creator();
            descriptor.key(s!(DOG_HOUSE)).unwrap();
            descriptor.name(s!("Dog House")).unwrap();
            descriptor.description(s!("A medium-sized dog house. It's just big enough for a single pet to lay.")).unwrap();
            descriptor
        }).unwrap();
        area_creator
    }).unwrap();

    let (_, mut world) = world_creator.create().unwrap().split();
    let mut world_editor = World::editor();

    world_editor.add_thing({
        let mut character_creator = model::Character::creator();
        character_creator.cortex({
            let mut routine_cortex_creator = RoutineCortexBuilder::creator();
            routine_cortex_creator.routine_uid(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
            routine_cortex_creator.routine_awareness(Awareness::Conscious).unwrap();
            routine_cortex_creator.cortex_builder()
        }).unwrap();
        character_creator.entity_builder().descriptor({
            let mut descriptor_creator = Descriptor::creator();
            descriptor_creator
                .key(s!(BLACK_CAT)).unwrap()
                .name(s!("Black Cat")).unwrap()
                .description(s!("A cheerful cat with a shiny black coat")).unwrap();
            descriptor_creator
        }).unwrap();
        character_creator.entity_builder().location(Location::Area(world.find_area(CAT_HOUSE).unwrap().uid())).unwrap();
        character_creator.thing_builder()
    }).unwrap();


    // route: dog_house <--> backyard
    world_editor.add_route({
        let area_a = world.find_area(BACKYARD).unwrap();
        let area_b = world.find_area(DOG_HOUSE).unwrap();
        let mut route_creator = Route::creator();
        route_creator.descriptor({
            let mut descriptor_creator = Descriptor::creator();
            descriptor_creator.key(s!("backyard_and_dog_house")).unwrap();
            descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
            descriptor_creator
        }).unwrap();
        route_creator.point_a({
            let mut endpoint_creator = route::Endpoint::creator();
            endpoint_creator.end({
                let mut end_creator = route::End::creator();
                end_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_dog_house_point_a")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
                    descriptor_creator
                }).unwrap();
                end_creator.area_identity(IdentityBuilder::from_existing(&end_creator, area_a)).unwrap();
                end_creator.direction(Direction::West).unwrap();
                end_creator
            }).unwrap();
            endpoint_creator.point_builder()
        }).unwrap();
        route_creator.point_b({
            let mut endpoint_creator = route::Endpoint::creator();
            endpoint_creator.end({
                let mut end_creator = route::End::creator();
                end_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_dog_house_point_b")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Dog House")).unwrap();
                    descriptor_creator
                }).unwrap();
                end_creator.area_identity(IdentityBuilder::from_existing(&end_creator, area_b)).unwrap();
                end_creator.direction(Direction::East).unwrap();
                end_creator
            }).unwrap();
            endpoint_creator.point_builder()
        }).unwrap();
        route_creator
    }).unwrap();

    // route: backyard <--> cat_house
    world_editor.add_route({
        let area_a = world.find_area(BACKYARD).unwrap();
        let area_b = world.find_area(CAT_HOUSE).unwrap();
        let mut route_creator = Route::creator();
        route_creator.descriptor({
            let mut descriptor_creator = Descriptor::creator();
            descriptor_creator.key(s!("backyard_and_cat_house")).unwrap();
            descriptor_creator.name(s!("Path between Backyard and Cat House")).unwrap();
            descriptor_creator
        }).unwrap();
        route_creator.point_a({
            let mut endpoint_creator = route::Endpoint::creator();
            endpoint_creator.end({
                let mut end_creator = route::End::creator();
                end_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_cat_house_point_a")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Cat House")).unwrap();
                    descriptor_creator
                }).unwrap();
                end_creator.area_identity(IdentityBuilder::from_existing(&end_creator, area_a)).unwrap();
                end_creator.direction(Direction::East).unwrap();
                end_creator
            }).unwrap();
            endpoint_creator.point_builder()
        }).unwrap();
        route_creator.point_b({
            let mut endpoint_creator = route::Endpoint::creator();
            endpoint_creator.end({
                let mut end_creator = route::End::creator();
                end_creator.descriptor({
                    let mut descriptor_creator = Descriptor::creator();
                    descriptor_creator.key(s!("backyard_and_cat_house_point_b")).unwrap();
                    descriptor_creator.name(s!("Path between Backyard and Cat House")).unwrap();
                    descriptor_creator
                }).unwrap();
                end_creator.area_identity(IdentityBuilder::from_existing(&end_creator, area_b)).unwrap();
                end_creator.direction(Direction::West).unwrap();
                end_creator
            }).unwrap();
            endpoint_creator.point_builder()
        }).unwrap();
        route_creator
    }).unwrap();

    let _result = world_editor.modify(&mut world).unwrap();
    
    world
}

pub fn world_to_binary(world: &model::World) -> Result<Vec<u8>> {
    let bytes = bincode::serde::encode_to_vec(&world, bincode::config::standard()).unwrap();
    Ok(bytes)
}

pub fn world_to_bytes(world: &model::World) -> Result<bytes::Bytes> {
    Ok(bytes::Bytes::from(world_to_binary(world).unwrap()))
}

pub fn world_from_binary(world_bytes: Vec<u8>) -> Result<World> {
    Ok(bincode::serde::decode_from_slice(&world_bytes, bincode::config::standard()).unwrap().0)
}

