use crate::interpreter::object::Command;
use std::any::{Any, TypeId};
use std::collections::HashMap;

// A trait used to represent a vector of components. This is needed in order to
// convert a vector of dynamic components into a specific generic type when
// querying our world.
trait ComponentVec {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for Vec<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

// Our world holds all our components. It's stored in a hash map where each
// component type is the key and the value is all the components of that type.
pub struct World {
	pub program: String,
	pub commands: Vec<Command>,
	components: HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl World {
	pub fn new() -> Self {
		World {
			program: String::new(),
			commands: vec![],
			components: HashMap::new(),
		}
	}

	pub fn query<T: 'static>(&self) -> Option<&Vec<T>> {
		// Generate a unique identifier based on the generic type
		let id = TypeId::of::<T>();

		// Look for all component for that type identifier. Downcast the dynamic
		// trait to our generic type. If the downcast fails the program will
		// panic.
		return self
			.components
			.get(&id)
			.map(|c| c.as_any().downcast_ref::<Vec<T>>().unwrap());
	}

	pub fn query_mut<T: 'static>(&mut self) -> &mut Vec<T> {
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
pub trait System {
	fn update(&self, world: &mut World);
}
