use crate::entities::Entities;
use crate::query::Query;

// Our world holds all our components. It's stored in a hash map where each
// component type is the key and the value is all the components of that type.
pub struct World {
	entities: Entities,
}

impl World {
	pub fn new() -> Self {
		World {
			entities: Entities::new(),
		}
	}

	pub fn register_component<T: 'static>(&mut self) {
		self.entities.register_component::<T>();
	}

	pub fn create_entity(&mut self) -> &mut Self {
		self.entities.create_entity();
		self
	}

	pub fn with_component<T: 'static>(&mut self, component: T) -> &mut Self {
		self.entities.insert_component(component);
		self
	}

	pub fn query<'a, T: Query<'a>>(&'a self) -> Vec<T::QueryItem> {
		T::query(&self.entities)
	}

	pub fn query_mut<T: 'static>(&mut self) -> Vec<&mut T> {
		self.entities.get_components_mut::<T>()
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
	fn test_create_entity_with_component() {
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
