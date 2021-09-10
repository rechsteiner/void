use crate::entities::Entities;
use crate::query::{Query, QueryMut};

// Our world holds all our entities and components. The actual components are
// stored inside the Entites struct so we can reuse the implementation between
// this struct and our Query implementation.
pub struct World {
	entities: Entities,
}

impl World {
	pub fn new() -> Self {
		World {
			entities: Entities::new(),
		}
	}

	/// Register a component of a given type. Must be called before using the
	/// `create_entity` method, or querying for that type.
	pub fn register_component<T: 'static>(&mut self) {
		self.entities.register_component::<T>();
	}

	/// Create a new entity. Returns an instance of self that can be used to add
	/// components for that entity.
	pub fn create_entity(&mut self) -> &mut Self {
		self.entities.create_entity();
		self
	}

	/// Inserts the given component at the index of the current entity.
	pub fn with_component<T: 'static>(&mut self, component: T) -> &mut Self {
		self.entities.insert_component(component);
		self
	}

	/// Query the world for components based on the generic type. See the
	/// `Query` trait for which generic types are allowed.
	pub fn query<'a, T: Query<'a>>(&'a self) -> Vec<T::QueryItem> {
		T::query(&self.entities)
	}

	/// Query the world for components of a type that returns a mutable
	/// reference to each component. See the `QueryMut` trait for which generic
	/// types are allowed.
	pub fn query_mut<'a, T: QueryMut<'a>>(&'a mut self) -> Vec<T::QueryItem> {
		T::query(&mut self.entities)
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
	fn test_query_tuple_two() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 });
		world
			.create_entity()
			.with_component(Size(40.0))
			.with_component(Location { x: 20.0, y: 20.0 });

		let components = world.query::<(&Location, &Size)>()[0];
		assert_eq!(components.0.x, 20.0);
		assert_eq!(components.1 .0, 40.0);
	}

	#[test]
	fn test_query_tuple_three() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world.register_component::<bool>();

		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 })
			.with_component(false);
		world
			.create_entity()
			.with_component(Size(40.0))
			.with_component(Location { x: 20.0, y: 20.0 })
			.with_component(true);

		let components = world.query::<(&Location, &Size, &bool)>()[0];
		assert_eq!(components.0.x, 20.0);
		assert_eq!(components.1 .0, 40.0);
		assert_eq!(*components.2, true);
	}

	#[test]
	fn test_query_mut() {
		let mut world = World::new();
		world.register_component::<Location>();
		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 });
		let locations = world.query_mut::<&mut Location>();
		assert_eq!(locations.len(), 1);
		assert_eq!(locations[0].x, 10.0);
		assert_eq!(locations[0].y, 10.0);
	}

	#[test]
	fn test_query_mut_tuple_two() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 });
		world
			.create_entity()
			.with_component(Size(40.0))
			.with_component(Location { x: 20.0, y: 20.0 });

		let components = world.query_mut::<(&Location, &Size)>()[0];
		assert_eq!(components.0.x, 20.0);
		assert_eq!(components.1 .0, 40.0);
	}

	#[test]
	fn test_query_mut_tuple_three() {
		let mut world = World::new();
		world.register_component::<Location>();
		world.register_component::<Size>();
		world.register_component::<bool>();

		world
			.create_entity()
			.with_component(Location { x: 10.0, y: 10.0 })
			.with_component(false);
		world
			.create_entity()
			.with_component(Size(40.0))
			.with_component(Location { x: 20.0, y: 20.0 })
			.with_component(true);

		let components = world.query_mut::<(&Location, &Size, &bool)>()[0];
		assert_eq!(components.0.x, 20.0);
		assert_eq!(components.1 .0, 40.0);
		assert_eq!(*components.2, true);
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
