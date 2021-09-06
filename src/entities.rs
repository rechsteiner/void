use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Components = Vec<Option<Box<dyn Any>>>;

pub struct Entities {
    components: HashMap<TypeId, Components>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        let id = TypeId::of::<T>();
        self.components.insert(id, vec![]);
    }

    pub fn create_entity(&mut self) {
        for (_key, value) in self.components.iter_mut() {
            value.push(None);
        }
    }

    pub fn insert_component<T: 'static>(&mut self, component: T) {
        let id = TypeId::of::<T>();
        let components = self.components.get_mut(&id).unwrap();
        let index = components.len() - 1;
        components[index] = Some(Box::new(component));
    }

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
    pub fn get_components_mut<T: 'static>(&mut self) -> Vec<&mut T> {
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
