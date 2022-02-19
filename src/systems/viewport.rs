use crate::resources::input::{Input, KeyCode};
use crate::resources::viewport::Viewport;
use crate::systems::System;
use crate::world::World;

pub struct ViewportSystem {}

impl ViewportSystem {
    pub fn new() -> ViewportSystem {
        ViewportSystem {}
    }
}

impl System for ViewportSystem {
    fn update(&mut self, world: &mut World) {
        let input = world.get_resource::<Input>().unwrap();
        let mut x = 0.0;
        let mut y = 0.0;
        let mut zoom = 0.0;

        for key in input.get_pressed() {
            match key {
                KeyCode::W => y = -1.0,
                KeyCode::S => y = 1.0,
                KeyCode::A => x = -1.0,
                KeyCode::D => x = 1.0,
                KeyCode::Z => zoom = -1.0,
                KeyCode::X => zoom = 1.0,
            }
        }

        let viewport = world.get_resource_mut::<Viewport>().unwrap();
        let movement_step = 15.0;
        let zoom_step = 0.04;

        viewport.move_target(x * movement_step, y * movement_step, zoom * zoom_step);
    }
}
