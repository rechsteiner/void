use crate::components::physics_mode::PhysicsMode;
use crate::components::program::Program;
use crate::components::rigid_body::{RigidBody, Transform};
use crate::components::shape::{ColorRGBA, Point, Shape};
use crate::components::viewport::Viewport;
use crate::scene::Scene;
use crate::systems::interpreter::InterpreterSystem;
use crate::systems::renderer::RenderSystem;
use crate::systems::simulation::SimulationSystem;
use crate::world::World;

pub fn generate_scene() -> Scene {
    let mut world = World::new();
    let color_cyan = ColorRGBA {
        r: 0,
        g: 255,
        b: 209,
        a: 1.0,
    };

    let color_orange = ColorRGBA {
        r: 255,
        g: 153,
        b: 34,
        a: 1.0,
    };

    // Register components

    // TODO: Would be nice if we didn't have to remember to register
    // components before using them. Right now it will panic.
    world.register_component::<RigidBody>();
    world.register_component::<Shape>();
    world.register_component::<PhysicsMode>();
    world.register_component::<Program>();
    world.register_component::<Viewport>();

    // Entity 1

    world
        .create_entity()
        .with_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 200.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point { x: -10.0, y: -15.0 },
                Point { x: 10.0, y: -15.0 },
                Point { x: 15.0, y: 10.0 },
                Point { x: -15.0, y: 10.0 },
            ],
            color: color_cyan,
        })
        .with_component(PhysicsMode::Dynamic)
        .with_component(Program::new());

    // Entity 2

    world
        .create_entity()
        .with_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 160.0 },
                rotation: 0.0,
            },
            mass: 0.2,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point { x: -10.0, y: -10.0 },
                Point { x: 10.0, y: -10.0 },
                Point { x: 10.0, y: 10.0 },
                Point { x: -10.0, y: 10.0 },
            ],
            color: color_orange,
        })
        .with_component(PhysicsMode::Dynamic);

    // Entity 3

    world
        .create_entity()
        .with_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 160.0 },
                rotation: 0.0,
            },
            mass: 0.2,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: true,
            vertices: vec![
                Point { x: -10.0, y: -10.0 },
                Point { x: 10.0, y: -10.0 },
                Point { x: 10.0, y: 10.0 },
                Point { x: -10.0, y: 10.0 },
            ],
            color: ColorRGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        })
        .with_component(PhysicsMode::Static);

    // Entity 4

    world
        .create_entity()
        .with_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 400.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point {
                    x: -300.0,
                    y: -10.0,
                },
                Point { x: 300.0, y: -10.0 },
                Point { x: 300.0, y: 0.0 },
                Point { x: -300.0, y: 0.0 },
            ],
            color: ColorRGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        })
        .with_component(PhysicsMode::Static);

    // Entity 5

    world
        .create_entity()
        .with_component(RigidBody {
            transform: Transform {
                position: Point { x: 0.0, y: 390.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point {
                    x: -40.0,
                    y: -120.0,
                },
                Point { x: 40.0, y: -120.0 },
                Point { x: 40.0, y: 0.0 },
                Point { x: -40.0, y: 0.0 },
            ],
            color: ColorRGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        })
        .with_component(PhysicsMode::Static);

    // Viewport resource

    world.create_resource(Viewport {
        position: Point { x: 200.0, y: 200.0 },
        zoom: 1.0,
        target_position: Point { x: 200.0, y: 200.0 },
        target_zoom: 1.0,
    });

    Scene::new(
        world,
        vec![
            Box::new(InterpreterSystem::new()),
            Box::new(SimulationSystem::new()),
            Box::new(RenderSystem::new()),
        ],
    )
}
