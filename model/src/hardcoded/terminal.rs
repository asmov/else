use asmov_else_rust_common::*;
use crate::*;

pub const TERMINAL_AREA_KEY: &'static str ="terminal";

/// Creates the starting room that all players join
pub fn create_terminal() -> World {
    let mut world_creator = World::creator();
    let identity = Identity::new(
        UniverseID::MAX,
        1,
        WorldField::class_id(),
        2);
    world_creator.uid(identity.uid()).unwrap();

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
