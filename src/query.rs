use crate::entities::Entities;

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
