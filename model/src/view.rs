// the model from the perspective of a Thing within the world (typically a Character)
// information is limited to what is visible or perceivable to the viewer
// as visibility changes, so will the model, through Sync messages

pub mod world;
pub mod area;
pub mod thing;

pub use world::*;
pub use area::*;
pub use thing::*;