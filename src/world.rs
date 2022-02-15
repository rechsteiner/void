use crate::entities::Entities;
use crate::query::{Query, QueryMut};
use hashbrown::HashMap;
use instant::Instant;
use std::any::{Any, TypeId};

// Our world holds all our entities and components. The actual components are
// stored inside the Entites struct so we can reuse the implementation between
// this struct and our Query implementation.
pub struct World {
    resources: HashMap<TypeId, Box<dyn Any>>,
    entities: Entities,
    pub start_timestamp: Instant, // TODO: Remove in favor of a Resource
}

impl World {
    pub fn new() -> Self {
        World {
            resources: HashMap::new(),
            entities: Entities::new(),
            start_timestamp: Instant::now(),
        }
    }

    /// Returns a resource of the given type.
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let id = TypeId::of::<T>();
        if let Some(data) = self.resources.get(&id) {
            data.downcast_ref()
        } else {
            None
        }
    }

    /// Returns a mutable reference to the resource of the given type.
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        if let Some(data) = self.resources.get_mut(&id) {
            data.downcast_mut()
        } else {
            None
        }
    }

    /// Inserts the given resource. This will override any existing resource
    /// with the same type.
    pub fn create_resource<T: 'static>(&mut self, resource: T) {
        let id = TypeId::of::<T>();
        self.resources.insert(id, Box::new(resource));
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

    /// Removes all components for a given entity identifier.
    pub fn remove_entity(&mut self, entity_id: usize) {
        self.entities.remove_entity(entity_id);
    }

    /// Removes the component of the given type for a specific entity.
    pub fn remove_component<T: 'static>(&mut self, entity_id: usize) {
        self.entities.remove_component::<T>(entity_id);
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

    #[test]

    fn test_get_resource() {
        let mut world = World::new();
        let id = TypeId::of::<bool>();
        world.resources.insert(id, Box::new(true));

        let resource = world.get_resource::<bool>().unwrap();
        assert_eq!(*resource, true);
    }

    #[test]
    fn test_get_resource_mut() {
        let mut world = World::new();
        let id = TypeId::of::<usize>();
        world.resources.insert(id, Box::new(1_usize));

        let resource = world.get_resource_mut::<usize>().unwrap();
        *resource += 1;
        assert_eq!(*resource, 2);
    }

    #[test]
    fn test_create_resource() {
        let mut world = World::new();
        world.create_resource(10_usize);

        let resource = world.get_resource::<usize>().unwrap();
        assert_eq!(*resource, 10);
    }
}
