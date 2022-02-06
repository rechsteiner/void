use rapier2d::na::Vector2;
use std::f32::consts::PI;

use crate::components::gravity::GravitySource;
use crate::components::point::Point;
use crate::components::program::Program;
use crate::components::rigid_body::{PhysicsMode, RigidBody, Transform};
use crate::components::shape::{ColorRGBA, Polygon, Shape};
use crate::components::text::Text;
use crate::components::thrusters::{Thruster, Thrusters};
use crate::components::viewport::Viewport;
use crate::scene::Scene;
use crate::systems::interpreter::InterpreterSystem;
use crate::systems::renderer::RenderSystem;
use crate::systems::simulation::SimulationSystem;
use crate::systems::thrust::ThrusterSystem;
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
    world.register_component::<Thrusters>();
    world.register_component::<Text>();

    world.create_entity().with_component(Text {
        content: "Hello World".to_string(),
        font: "20px monospace".to_string(),
        position: Point { x: 300.0, y: 300.0 },
        color: ColorRGBA {
            r: 255,
            g: 255,
            b: 255,
            a: 1.0,
        },
    });

    // Entity 1: Spaceship
    // Note that it's upside down, and then rotated 90deg (1 PI).
    // That is because the game Z+ axis points downward.
    world
        .create_entity()
        .with_component(RigidBody {
            id: 1,
            transform: Transform {
                position: Vector2::new(200.0, 50.0),
                rotation: PI,
            },
            mass: 1.0,
            linear_velocity: Vector2::new(0.0, 0.0),
            angular_velocity: 0.0,
            physics_mode: PhysicsMode::Dynamic,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point { x: -15.0, y: -15.0 },
                Point { x: 15.0, y: -15.0 },
                Point { x: 10.0, y: 15.0 },
                Point { x: -10.0, y: 15.0 },
            ],
            color: color_cyan,
        })
        .with_component(Program::new())
        .with_component(Thrusters::new(
            1000.0,
            1000.0,
            vec![Thruster {
                max_thrust_force: 3000.0,
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                fuel_consumption_per_force: 0.001,
            }],
        ));

    // Entity 2: Orange box
    world
        .create_entity()
        .with_component(RigidBody {
            id: 2,
            transform: Transform {
                position: Vector2::new(200.0, 20.0),
                rotation: 0.0,
            },
            mass: 0.1,
            linear_velocity: Vector2::new(0.0, 0.0),
            angular_velocity: 0.0,
            physics_mode: PhysicsMode::Dynamic,
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
        });

    // Entity 3: Orbital mystical object
    world
        .create_entity()
        .with_component(RigidBody {
            id: 3,
            transform: Transform {
                position: Vector2::new(200.0, -700.0),
                rotation: 0.0,
            },
            mass: 2.0,
            linear_velocity: Vector2::new(260.0, 0.0),
            angular_velocity: 0.1,
            physics_mode: PhysicsMode::Dynamic,
        })
        .with_component(Shape {
            is_sensor: false,
            vertices: vec![
                Point { x: -30.0, y: -40.0 },
                Point { x: 30.0, y: -40.0 },
                Point { x: 30.0, y: 40.0 },
                Point { x: -30.0, y: 40.0 },
            ],
            color: ColorRGBA {
                r: 90,
                g: 90,
                b: 90,
                a: 1.0,
            },
        });

    // Entity 4: Big world
    world
        .create_entity()
        .with_component(RigidBody {
            id: 4,
            transform: Transform {
                position: Vector2::new(200.0, 900.0),
                rotation: 0.0,
            },
            mass: 10000.0,
            linear_velocity: Vector2::new(0.0, 0.0),
            angular_velocity: 0.0,
            physics_mode: PhysicsMode::Static,
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
        .with_component(GravitySource {
            strength: 100000000.0, // Not super user-friendly with these kinds of large numbers
        });

    // Entity 5: Moon
    world
        .create_entity()
        .with_component(RigidBody {
            id: 5,
            transform: Transform {
                position: Vector2::new(7000.0, 0.0),
                rotation: 0.0,
            },
            mass: 200.0,
            linear_velocity: Vector2::new(0.0, 100.0),
            angular_velocity: 0.5,
            physics_mode: PhysicsMode::Dynamic,
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
        .with_component(GravitySource {
            strength: 10000.0, // Not super user-friendly with these kinds of large numbers
        });

    // Viewport resource
    world.create_resource(Viewport {
        position: Vector2::new(200.0, 200.0),
        zoom: 0.2,
        target_position: Vector2::new(200.0, 200.0),
        target_zoom: 0.2,
    });

    Scene::new(
        world,
        vec![
            Box::new(InterpreterSystem::new()),
            Box::new(ThrusterSystem::new()),
            Box::new(SimulationSystem::new()),
            Box::new(RenderSystem::new()),
        ],
    )
}
