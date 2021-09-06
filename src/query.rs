use crate::entities::Entities;

pub trait Query<'a> {
    type QueryItem;
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem>;
}

impl<'a, T: 'static> Query<'a> for &T {
    type QueryItem = &'a T;
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
        entities.get_components::<T>()
    }
}
impl<'a, T: 'static, U: 'static> Query<'a> for (&T, &U) {
    type QueryItem = (&'a T, &'a U);
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
        let first = entities.get_components::<T>().into_iter();
        let second = entities.get_components::<U>().into_iter();
        first.zip(second).collect()
    }
}

impl<'a, T: 'static, U: 'static, V: 'static> Query<'a> for (&T, &U, &V) {
    type QueryItem = (&'a T, &'a U, &'a V);
    fn query(entities: &'a Entities) -> Vec<Self::QueryItem> {
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
