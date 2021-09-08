use crate::entities::Entities;
use std::any::{Any, TypeId};

/// The Query trait allow us to provide an API in the World struct where you can
/// query based on the generic types. For instead, you can query for a single
/// component type using `query::<T>`, but if you need all entities matching two
/// component types you can specify it as a tuple like this `query::<(&T, &U)>`.
///
/// This is acheived by implementing the Query trait for the types we want to
/// allow. If you see below, we have implemented for &T, (&T, &U) etc. The code
/// gets a bit hairy, but the bonus is that our World API becomes very clean.
pub trait Query<'a> {
    type QueryItem;
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem>;
}

/// Supports querying for a single component type.
impl<'a, T: 'static> Query<'a> for &T {
    type QueryItem = &'a T;
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
        // Get all the components for the given generic type.
        entities.get_components::<T>()
    }
}

/// Supports querying two component types at the same time.
impl<'a, T: 'static, U: 'static> Query<'a> for (&T, &U) {
    type QueryItem = (&'a T, &'a U);
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
        // Get all the components for the two generic types in the tuple. Then
        // zip them together into a single tuple.
        let first = entities.get_components::<T>().into_iter();
        let second = entities.get_components::<U>().into_iter();
        first.zip(second).collect()
    }
}

/// Supports querying three component types at the same time.
impl<'a, T: 'static, U: 'static, V: 'static> Query<'a> for (&T, &U, &V) {
    type QueryItem = (&'a T, &'a U, &'a V);
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
        // Get all the components for the three generic types in the tuple. Then
        // zip them together into a single tuple.
        let first = entities.get_components::<T>().into_iter();
        let second = entities.get_components::<U>().into_iter();
        let third = entities.get_components::<V>().into_iter();
        first
            .zip(second)
            .zip(third)
            .map(|((first, second), third)| (first, second, third))
            .collect()
    }
}

// Need a separate QueryMut trait here instead of reusing Query as we need a
// mutable reference to Entities. Making Entities mutable in Query means the
// borrow checker will complain when querying multiple times.
pub trait QueryMut<'a> {
    type QueryItem;
    fn query(entities: &'a mut Entities) -> Vec<Self::QueryItem>;
}

/// Supports querying for a single component type (both &mut T or &T).
impl<'a, T: Downcastable<'a>> QueryMut<'a> for T {
    type QueryItem = T::Item;
    fn query(entities: &'a mut Entities) -> Vec<Self::QueryItem> {
        let mut vec = entities.components.get_mut(&T::id()).unwrap();
        T::downcast(vec)
    }
}

/// Supports querying two component types at the same time.
impl<'a, T: Downcastable<'a>, U: Downcastable<'a>> QueryMut<'a> for (T, U) {
    type QueryItem = (T::Item, U::Item);
    fn query(entities: &'a mut Entities) -> Vec<Self::QueryItem> {
        if let [Ok(first), Ok(second)] = entities.components.get_each_mut([&T::id(), &U::id()]) {
            let first = T::downcast(first).into_iter();
            let second = U::downcast(second).into_iter();
            first.zip(second).collect()
        } else {
            panic!("could not find any components of the given generic type. Make sure you have called register_component for all types.")
        }
    }
}

/// Supports querying three component types at the same time.
impl<'a, T: Downcastable<'a>, U: Downcastable<'a>, V: Downcastable<'a>> QueryMut<'a> for (T, U, V) {
    type QueryItem = (T::Item, U::Item, V::Item);
    fn query(entities: &'a mut Entities) -> Vec<Self::QueryItem> {
        if let [Ok(first), Ok(second), Ok(third)] =
            entities
                .components
                .get_each_mut([&T::id(), &U::id(), &V::id()])
        {
            let first = T::downcast(first).into_iter();
            let second = U::downcast(second).into_iter();
            let third = V::downcast(third).into_iter();
            first
                .zip(second)
                .zip(third)
                .map(|((first, second), third)| (first, second, third))
                .collect()
        } else {
            panic!("could not find any components of the given generic type. Make sure you have called register_component for all types.")
        }
    }
}

/// Used to downcast a vector for dynamic Any into a vec of either simple
/// references or mutable references. Having this in a separate trait means we
/// can reduce the number of QueryMut implementation, as we only need an
/// implementation for the number of item in the tuple and not on for each
/// combination of ref and mut (e.g. (&mut, &, &), (&, &mut, &) etc.). This is
/// because the generic type will automatically choose the correct downcast
/// method based on the generic type that is passed in.
pub trait Downcastable<'a> {
    type Item;
    fn id() -> TypeId;
    fn downcast(vec: &'a mut Vec<Option<Box<dyn Any>>>) -> Vec<Self::Item>;
}

/// Implements downcasting to a normal reference.
impl<'a, T: 'static> Downcastable<'a> for &T {
    type Item = &'a T;
    fn id() -> TypeId {
        TypeId::of::<T>()
    }
    fn downcast(vec: &'a mut Vec<Option<Box<dyn Any>>>) -> Vec<Self::Item> {
        vec.into_iter()
            .flatten()
            .map(|c| c.downcast_ref::<T>().unwrap())
            .collect()
    }
}

/// Implements downcasting to a mutable reference.
impl<'a, T: 'static> Downcastable<'a> for &mut T {
    type Item = &'a mut T;
    fn id() -> TypeId {
        TypeId::of::<T>()
    }
    fn downcast(vec: &'a mut Vec<Option<Box<dyn Any>>>) -> Vec<Self::Item> {
        vec.into_iter()
            .flatten()
            .map(|c| c.downcast_mut::<T>().unwrap())
            .collect()
    }
}
