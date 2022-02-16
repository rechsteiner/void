pub mod instruments_renderer;
pub mod interpreter;
pub mod renderer;
pub mod simulation;
pub mod thrust;

use crate::world::World;

// The system trait allows each system to read and mutate the world. Any changes
// to the world will be available for the next system.
pub trait System {
    fn update(&mut self, world: &mut World);
}
