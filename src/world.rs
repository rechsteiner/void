use crate::query::Query;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Components = HashMap<TypeId, Vec<Option<Box<dyn Any>>>>;

// Our world holds all our components. It's stored in a hash map where each
// component type is the key and the value is all the components of that type.
pub struct World {
	components: Components,
}

impl World {
	pub fn new() -> Self {
		World {
			components: HashMap::new(),
		}
	}

	pub fn register_component<T: 'static>(&mut self) {
		let id = TypeId::of::<T>();
		self.components.insert(id, vec![]);
	}

	pub fn create_entity(&mut self) -> &mut Self {
		for (_key, value) in self.components.iter_mut() {
			value.push(None);
		}
		self
	}

	pub fn with_component<T: 'static>(&mut self, component: T) -> &mut Self {
		let id = TypeId::of::<T>();
		let components = self.components.get_mut(&id).unwrap();
		let index = components.len() - 1;
		components[index] = Some(Box::new(component));
		self
	}

	pub fn query<'a, T: Query<'a>>(&'a self) -> Vec<T::QueryItem> {
		T::query(&self.components)
	}

	pub fn query_mut<T: 'static>(&mut self) -> Vec<&mut T> {
		// Generate a unique identifier based on the generic type
		let id = TypeId::of::<T>();
		// Look for all component for that type identifier. Downcast each value
		// to the given generic type.
		self.components
			.get_mut(&id)
			.unwrap()
			.into_iter()
			.flatten()
			.map(|c| c.downcast_mut::<T>().unwrap())
			.collect()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Debug, PartialEq, Clone)]
	struct Size(f32);
	#[derive(Debug, PartialEq, Clone)]
	struct Location {
		x: f32,
		y: f32,
	}

	#[test]
	fn test_query() {
		let mut world = World::new();
		world.register_component::<Location>();
		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 });
		let locations = world.query::<&Location>();
		assert_eq!(locations.len(), 1);
		assert_eq!(locations[0].x, 10.0);
		assert_eq!(locations[0].y, 10.0);
	}

	#[test]
	fn test_query_mut() {
		let mut world = World::new();
		world.register_component::<Location>();
		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 });
		let locations = world.query_mut::<Location>();
		assert_eq!(locations.len(), 1);
		assert_eq!(locations[0].x, 10.0);
		assert_eq!(locations[0].y, 10.0);
	}

	#[test]
	fn test_register_component() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();

		assert!(world.components.contains_key(&TypeId::of::<Location>()));
		assert!(world.components.contains_key(&TypeId::of::<Size>()));
	}

	#[test]
	fn test_create_entity() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world.create_entity();

		let locations = world.components.get(&TypeId::of::<Location>()).unwrap();
		let sizes = world.components.get(&TypeId::of::<Size>()).unwrap();

		assert_eq!(sizes.len(), 1);
		assert_eq!(locations.len(), 1);
		assert!(sizes[0].is_none());
		assert!(locations[0].is_none());
	}

	#[test]
	fn test_with_component() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world
			.create_entity()
			.with_component(Location { x: 1.0, y: 1.0 })
			.with_component(Size(20.0));

		let locations = world.query::<&Location>();
		let sizes = world.query::<&Size>();

		assert_eq!(sizes.len(), 1);
		assert_eq!(locations.len(), 1);
		assert_eq!(*sizes[0], Size(20.0));
		assert_eq!(*locations[0], Location { x: 1.0, y: 1.0 });
	}
}
