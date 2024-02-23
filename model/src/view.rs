// the model from the perspective of a Thing within the world (typically a Character)
// information is limited to what is visible or perceivable to the viewer
// as visibility changes, so will the model, through Sync messages

pub mod thing;
pub mod area;
pub mod world;
pub mod interface;

pub use thing::*;
pub use area::*;
pub use world::*;
pub use interface::*;