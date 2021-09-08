use crate::{systems::System, world::World};

pub struct Scene {
    pub world: World,
    pub systems: Vec<Box<dyn System>>,
}

impl Scene {
    pub fn new(world: World, systems: Vec<Box<dyn System>>) -> Scene {
        Scene { world, systems }
    }
}
