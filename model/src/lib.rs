pub use elsezone_rust_common::{self as elserust, s};

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
pub mod world;
pub mod message;
pub mod interface;
pub mod timeframe;
pub mod cortex;
pub mod sync;
pub mod view;

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
pub use area::*;
pub use route::*;
pub use world::*;
pub use message::*;
pub use interface::*;
pub use timeframe::*;
pub use cortex::*;
pub use sync::*;
pub use view::*;

pub mod hardcoded {
    pub mod terminal {
        use elsezone_rust_common::*;
        use crate::*;

        pub const TERMINAL_AREA_ID: u64 = 1;

        /// Creates the starting room that all players join
        pub fn create_terminal() -> World {
            let mut world_creator = World::creator();
            world_creator.identity_builder().guid(0, 0, 1, 1).unwrap();
            world_creator.frame(0).unwrap();
            world_creator.descriptor({
                    let mut descriptor = Descriptor::creator();
                    descriptor.key(s!("terminal")).unwrap();
                    descriptor.name(s!("Terminal")).unwrap();
                    descriptor
            }).unwrap();
            world_creator.add_area({
                let mut area_creator = Area::creator();
                area_creator.descriptor({
                    let mut descriptor = Descriptor::creator();
                    descriptor.key(s!("terminal")).unwrap();
                    descriptor.name(s!("Terminal")).unwrap();
                    descriptor.description(s!(
                        "Welcome to Terminal.\n\
                        Connect your interface to begin your journey.\n\
                        \n\
                        A myriad of bright colors race around you and then dissolve into your surroundings as quickly \
                        as they appeared.\n\
                        \n\
                        You find yourself in what appears to be an enormous translucent sphere. Beyond that, you see \
                        only the void of space, littered with clusters of brightly lit stars in all directions. The \
                        iridescent wall of the great sphere shimmers with color in tune with the motion and sounds \
                        around you. Numerous others, like yourself, hustle and bustle about the area. You hear the \
                        soft buzz of the commotion surrounding you; discussions, laughter, the whirring of people \
                        casually materializing in and out of existence.\n\
                        \n\
                        A holographic computer screen materializes in front of you. The dotted blue outline of a hand \
                        appears in the center of the screen with instructions below:\n\
                        Type .connect to begin ..."
                    )).unwrap();
                    descriptor
                }).unwrap();
                area_creator
            }).unwrap();

            let (_, world) = world_creator.create().unwrap().split();
            world
        }
    }
}

pub mod testing {
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

        world_creator.identity_builder().guid(0, 0, 1, 1).unwrap();
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

        world_creator.add_thing({
            let mut character_creator = model::Character::creator();
            character_creator.cortex({
                let mut routine_cortex_creator = RoutineCortexBuilder::creator();
                routine_cortex_creator.routine_id(0).unwrap(); //todo: model crate should have an enum of IDs from behavior crate
                routine_cortex_creator.routine_awareness(Awareness::Conscious).unwrap();
                routine_cortex_creator.cortex_builder()
            }).unwrap();
            let descriptor_creator = character_creator.entity_builder().descriptor_builder();
            descriptor_creator.key(s!(BLACK_CAT)).unwrap();
            descriptor_creator.name(s!("Black Cat")).unwrap();
            descriptor_creator.description(s!("A cheerful cat with a shiny black coat")).unwrap();
            character_creator.thing_builder()
        }).unwrap();

        let (_, mut world) = world_creator.create().unwrap().split();
        let mut world_editor = World::editor();

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
                    end_creator.area_identity(area_a.identity().to_creator()).unwrap();
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
                    end_creator.area_identity(area_b.identity().to_creator()).unwrap();
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
                    end_creator.area_identity(area_a.identity().to_creator()).unwrap();
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
                    end_creator.area_identity(area_b.identity().to_creator()).unwrap();
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
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use crate::{self as model, *};
    use crate::s;

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

        let thing_id = world.spawn_thing(character_creator.thing_builder(), area.id()).unwrap();
        let thing = world.thing(thing_id).unwrap();

        assert_eq!("A gray cat", thing.description().unwrap());
    }

    #[test]
    fn test_manual_building() {
        let mut world = testing::create_world();

        let litterbox_id = world.find_area(testing::CAT_HOUSE)
            .unwrap()
            .id();

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

        let gray_cat = world.find_thing_mut("gray_cat").unwrap();

        let mut gray_cat_descriptor_editor = Descriptor::editor();
        gray_cat_descriptor_editor.description(s!("A slightly gray cat")).unwrap();
        gray_cat_descriptor_editor.modify(gray_cat.descriptor_mut()).unwrap();

        let gray_cat_editor = Entity::editor();
        gray_cat_editor.modify(gray_cat.entity_mut()).unwrap();

        let gray_cat = world.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", gray_cat.description().unwrap());
    }
}
