use crate::components::gravity::{GravityAffected, GravitySource};
use crate::components::physics_mode::PhysicsMode;
use crate::components::program::Program;
use crate::components::rigid_body::{RigidBody, Transform};
use crate::components::shape::{ColorRGBA, Point, Polygon, Shape};
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
    world.register_component::<GravitySource>();
    world.register_component::<GravityAffected>();

    // Entity 1: Spaceship
    world
        .create_entity()
        .with_component(RigidBody {
            id: 1,
            transform: Transform {
                position: Point {
                    x: 200.0,
                    y: -600.0,
                },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 350.0, y: 0.0 },
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
        .with_component(GravityAffected {})
        .with_component(Program::new());

    // Entity 2: Orange box
    world
        .create_entity()
        .with_component(RigidBody {
            id: 2,
            transform: Transform {
                position: Point {
                    x: 200.0,
                    y: -650.0,
                },
                rotation: 0.0,
            },
            mass: 0.1,
            linear_velocity: Point { x: 350.0, y: 0.0 },
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
        .with_component(PhysicsMode::Dynamic)
        .with_component(GravityAffected {});

    // Entity 3: Big world
    world
        .create_entity()
        .with_component(RigidBody {
            id: 3,
            transform: Transform {
                position: Point { x: 200.0, y: 900.0 },
                rotation: 0.0,
            },
            mass: 0.2,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: Polygon::planetoid(800.0, 64, 10),
            color: ColorRGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        })
        .with_component(PhysicsMode::Static)
        .with_component(GravitySource {
            strength: 100000.0, // Not super user-friendly with these kinds of large numbers
        });

    // Entity 4: Moon
    world
        .create_entity()
        .with_component(RigidBody {
            id: 4,
            transform: Transform {
                position: Point {
                    x: 4000.0,
                    y: -300.0,
                },
                rotation: 0.0,
            },
            mass: 200.0,
            linear_velocity: Point { x: 0.0, y: 280.0 },
            angular_velocity: 0.0,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: Polygon::planetoid(300.0, 32, 20),
            color: ColorRGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        })
        .with_component(PhysicsMode::Dynamic)
        .with_component(GravitySource {
            strength: 5000.0, // Not super user-friendly with these kinds of large numbers
        })
        .with_component(GravityAffected {});
    // Viewport resource

    world.create_resource(Viewport {
        position: Point { x: 200.0, y: 200.0 },
        zoom: 0.2,
        target_position: Point { x: 200.0, y: 200.0 },
        target_zoom: 0.2,
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
