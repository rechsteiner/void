use hashbrown::HashMap;
use std::any::{Any, TypeId};

/// A type-alias for our vector of components. Each item in the vector is either
/// None or Some(Any). This is done so that we can use the index of the vector
/// as the identifier for a entity. It's not the most performant having to
/// potentially loop over a bunch of None values, so we might want to look into
/// other data structures here like Archetypes.
pub type Components = Vec<Option<Box<dyn Any>>>;

/// The entities struct is just a wrapper around our components store. It hides
/// the underlying data structures, while allowing us to create and get
/// components from both our World and Query types.
pub struct Entities {
    pub components: HashMap<TypeId, Components>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            components: HashMap::new(),
        }
    }

    /// Register a component type by creating an empty vector for that type.
    pub fn register_component<T: 'static>(&mut self) {
        let id = TypeId::of::<T>();
        self.components.insert(id, vec![]);
    }

    /// Create an entity by inserting empty values in the vectors for each
    /// component type.
    pub fn create_entity(&mut self) {
        for (_key, value) in self.components.iter_mut() {
            value.push(None);
        }
    }

    /// Insert a component of a given type. It's important to call
    /// `register_component` and `create_entity` before calling this method.
    pub fn insert_component<T: 'static>(&mut self, component: T) {
        let id = TypeId::of::<T>();
        let components = self.components.get_mut(&id).unwrap();
        let index = components.len() - 1;
        components[index] = Some(Box::new(component));
    }

    /// Get a reference to all the components of a given type.
    pub fn get_components<T: 'static>(&self) -> Vec<&T> {
        // Generate a unique identifier based on the generic type
        let id = TypeId::of::<T>();
        // Look for all component for that type identifier. Downcast each value
        // to the given generic type.
        self.components
            .get(&id)
            .unwrap()
            .into_iter()
            .flatten()
            .map(|c| c.downcast_ref::<T>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register_component() {
        let mut entities = Entities::new();
        entities.register_component::<u32>();
        entities.register_component::<isize>();

        assert!(entities.components.contains_key(&TypeId::of::<u32>()));
        assert!(entities.components.contains_key(&TypeId::of::<isize>()));
    }

    #[test]
    fn test_create_entity() {
        let mut entities = Entities::new();
        entities.register_component::<u32>();
        entities.register_component::<isize>();
        entities.create_entity();

        let u32s = entities.components.get(&TypeId::of::<u32>()).unwrap();
        let isizes = entities.components.get(&TypeId::of::<u32>()).unwrap();

        assert_eq!(u32s.len(), 1);
        assert_eq!(isizes.len(), 1);
        assert!(u32s[0].is_none());
        assert!(isizes[0].is_none());
    }

    #[test]
    fn test_insert_component() {
        let mut entities = Entities::new();
        entities.register_component::<u32>();
        entities.register_component::<isize>();
        entities.create_entity();
        entities.insert_component(1_u32);
        entities.insert_component(2_isize);

        let u32s = entities.get_components::<u32>();
        let isizes = entities.get_components::<isize>();

        assert_eq!(u32s.len(), 1);
        assert_eq!(isizes.len(), 1);
        assert_eq!(*u32s[0], 1);
        assert_eq!(*isizes[0], 2);
    }
}
