use std::any::{Any, TypeId};
use std::collections::HashMap;

struct Velocity {
	x: f32,
	y: f32,
}

struct Location {
	x: f32,
	y: f32,
}

// A trait used to represent a vector of components. This is needed in order to
// convert a vector of dynamic components into a specific generic type when
// querying our world.
trait ComponentVec {
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for Vec<T> {
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

// Our world holds all our components. It's stored in a hash map where each
// component type is the key and the value is all the components of that type.
struct World {
	components: HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl World {
	pub fn new() -> Self {
		World {
			components: HashMap::new(),
		}
	}

	pub fn get_components_by_type<T: 'static>(&mut self) -> &mut Vec<T> {
		// Generate a unique identifier based on the generic type
		let id = TypeId::of::<T>();

		// Look for all component for that type identifier. If it returns none
		// we insert an empty array and use that instead.
		let components_vec = self
			.components
			.entry(id)
			.or_insert(Box::new(Vec::<T>::new()));

		// Downcast the dynamic trait to our generic type. If the downcast fails
		// the program will panic.
		return components_vec
			.as_any_mut()
			.downcast_mut::<Vec<T>>()
			.unwrap();
	}

	// TODO: Replace this with a more advanced Entity builder.
	pub fn insert_component<T: 'static>(&mut self, component: T) {
		let id = TypeId::of::<T>();

		match self.components.get_mut(&id) {
			Some(components) => {
				components
					.as_any_mut()
					.downcast_mut::<Vec<T>>()
					.unwrap()
					.push(component);
			}
			None => {
				self.components.insert(id, Box::new(vec![component]));
			}
		}
	}
}

// The system trait allows each system to read and mutate the world. Any changes
// to the world will be available for the next system.
trait System {
	fn update(&self, world: &mut World);
}

struct RenderSystem {}

impl System for RenderSystem {
	fn update(&self, world: &mut World) {
		for component in world.get_components_by_type::<Location>() {
			println!("rendering component: #{} #{}", component.x, component.y);
		}
	}
}

struct PhysicsSystem {}

impl System for PhysicsSystem {
	fn update(&self, world: &mut World) {
		for component in world.get_components_by_type::<Location>() {
			component.x = 20.0;
			println!(
				"updating physics for component: #{} #{}",
				component.x, component.y
			);
		}
	}
}

fn main() {
	// Initialize our world with some components.
	let mut world = World::new();
	world.insert_component(Location { x: 2.0, y: 10.0 });
	world.insert_component(Velocity { x: 100.0, y: 100.0 });

	for component in world.get_components_by_type::<Location>() {
		println!("location before: #{} #{}", component.x, component.y);
	}

	for component in world.get_components_by_type::<Velocity>() {
		println!("velocity before: #{} #{}", component.x, component.y);
	}

	// Create a vector of all our systems for a given scene. The order here is
	// important as each system can mutate the world.
	let systems: Vec<Box<dyn System>> = vec![Box::new(PhysicsSystem {}), Box::new(RenderSystem {})];

	// Loop through each system in order and update the world.
	for system in systems {
		system.update(&mut world);
	}

	for component in world.get_components_by_type::<Location>() {
		println!("location after: #{} #{}", component.x, component.y);
	}

	for component in world.get_components_by_type::<Velocity>() {
		println!("velocity after: #{} #{}", component.x, component.y);
	}
}
