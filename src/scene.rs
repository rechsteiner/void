use crate::world::World;

pub struct Scene {
    pub world: World,
}

impl Scene {
    pub fn new(world: World) -> Scene {
        Scene { world }
    }
}
