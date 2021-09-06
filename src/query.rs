use crate::world::Components;
use std::any::TypeId;

pub trait Query<'a> {
    type QueryItem;
    fn query(components: &'a Components) -> Vec<Self::QueryItem>;
}

fn downcast<'a, T: 'static>(components: &'a Components) -> Vec<&T> {
    components
        .get(&TypeId::of::<T>())
        .unwrap()
        .into_iter()
        .flatten()
        .map(|c| c.downcast_ref::<T>().unwrap())
        .collect()
}

impl<'a, T: 'static> Query<'a> for &T {
    type QueryItem = &'a T;
    fn query(components: &'a Components) -> Vec<Self::QueryItem> {
        downcast::<T>(components)
    }
}

impl<'a, T: 'static, U: 'static> Query<'a> for (&T, &U) {
    type QueryItem = (&'a T, &'a U);
    fn query(components: &'a Components) -> Vec<Self::QueryItem> {
        let first = downcast::<T>(components);
        let second = downcast::<U>(components);
        first
            .iter()
            .zip(second.iter())
            .map(|(first, second)| (*first, *second))
            .collect()
    }
}

impl<'a, T: 'static, U: 'static, V: 'static> Query<'a> for (&T, &U, &V) {
    type QueryItem = (&'a T, &'a U, &'a V);
    fn query(components: &'a Components) -> Vec<Self::QueryItem> {
        let first = downcast::<T>(components);
        let second = downcast::<U>(components);
        let third = downcast::<V>(components);
        first
            .iter()
            .zip(second.iter().zip(third.iter()))
            .map(|(first, (second, third))| (*first, *second, *third))
            .collect()
    }
}
