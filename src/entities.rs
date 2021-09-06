use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Components = Vec<Option<Box<dyn Any>>>;

pub struct Entities {
    pub components: HashMap<TypeId, Components>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            components: HashMap::new(),
        }
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
