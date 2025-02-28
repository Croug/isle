use std::{any::TypeId, cell::UnsafeCell, collections::HashSet, marker::PhantomData};

use crate::{
    component::Component,
    ecs::{BorrowSignature, RefType, SystemParam, TypeSet},
    entity::Entity,
    world::World,
};

pub struct Query<'w, T, V = ()>
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    world: &'w UnsafeCell<World>,
    marker: PhantomData<(T, V)>,
}

impl<'w, T, V> Query<'w, T, V>
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    pub fn fetch_entities(&self) -> HashSet<Entity> {
        let mut interactable_components = HashSet::<BorrowSignature>::new();
        let mut with_components = HashSet::<BorrowSignature>::new();
        let mut without_components = HashSet::<BorrowSignature>::new();

        T::get_components(&mut interactable_components);
        V::get_components(&mut with_components, &mut without_components);

        let components: HashSet<TypeId> = interactable_components
            .union(&with_components)
            .filter(|BorrowSignature(_, ref_type)| {
                ref_type != &RefType::OptionalImmutable && ref_type != &RefType::OptionalMutable
            })
            .copied()
            .map(|BorrowSignature(type_id, _)| type_id)
            .collect();

        let without_components: HashSet<TypeId> = without_components
            .iter()
            .copied()
            .map(|BorrowSignature(type_id, _)| type_id)
            .collect();

        interactable_components
            .iter()
            .map(|BorrowSignature(type_id, _)| {
                let world = unsafe { &mut *self.world.get() };
                world.get_entities_with_component(type_id)
            })
            .flatten()
            .filter(|entity| {
                let world = unsafe { &mut *self.world.get() };
                let entity_components = world.get_entity_components(entity);

                entity_components.is_superset(&components)
                    && entity_components.is_disjoint(&without_components)
            })
            .collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = T::Item<'w>> + '_ {
        let entities = self.fetch_entities();

        entities
            .into_iter()
            .map(move |entity| T::from_entity(entity, &self.world))
    }
}

impl<'__w, T, V> SystemParam for Query<'__w, T, V>
where
    T: QueryParam + 'static,
    V: ReadOnlyQueryParam + 'static,
{
    type State = ();
    type Item<'new> = Query<'new, T, V>;
    fn init_state(_: &UnsafeCell<World>) -> Self::State {}
    fn from_world<'w>(
        world: &'w UnsafeCell<World>,
        _: &mut Self::State,
        _: &str,
    ) -> Self::Item<'w> {
        Query::<T, V> {
            world: &world,
            marker: PhantomData::<(T, V)>,
        }
    }
    fn collect_types(types: &mut impl crate::ecs::TypeSet) -> () {
        let mut _component_set = HashSet::<BorrowSignature>::new();
        let mut _filter_set = HashSet::<BorrowSignature>::new();

        T::get_components(&mut _component_set);
        V::get_components(&mut _filter_set, &mut _component_set);
        types.insert_type::<Query<T, V>>(RefType::Immutable);
    }
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam {
    type Item<'new>;

    // fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Readonly<'w>;
    fn get_components(type_set: &mut impl TypeSet) -> ();
    fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w>;
}

pub trait ReadOnlyQueryParam {
    fn get_components(with_set: &mut impl TypeSet, without_set: &mut impl TypeSet) -> ();
}

impl<T: Component + 'static> ReadOnlyQueryParam for With<T> {
    fn get_components(with_set: &mut impl TypeSet, _: &mut impl TypeSet) -> () {
        with_set.insert_type::<T>(RefType::Immutable);
    }
}

impl<T: Component + 'static> ReadOnlyQueryParam for Without<T> {
    fn get_components(_: &mut impl TypeSet, without_set: &mut impl TypeSet) -> () {
        without_set.insert_type::<T>(RefType::Immutable);
    }
}

impl QueryParam for Entity {
    type Item<'new> = Entity;

    fn get_components(_: &mut impl TypeSet) -> () {
        ()
    }
    fn from_entity<'w>(entity: Entity, _: &'w UnsafeCell<World>) -> Self::Item<'w> {
        entity
    }
}

impl<T: Component + 'static> QueryParam for &T {
    type Item<'new> = &'new T;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Immutable);
    }
    fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &mut *world.get() };
        let component = unsafe { world.get_component_mut(&entity) };
        component.unwrap()
    }
}

impl<T: Component + 'static> QueryParam for &mut T {
    type Item<'new> = &'new mut T;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Mutable);
    }
    fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &mut *world.get() };
        let component = unsafe { world.get_component_mut(&entity) };
        component.unwrap()
    }
}

impl<T: Component + 'static> QueryParam for Option<&T> {
    type Item<'new> = Option<&'new T>;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::OptionalImmutable);
    }
    fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &mut *world.get() };
        let component = unsafe { world.get_component_mut(&entity) };

        if let Some(component) = component {
            Some(component)
        } else {
            None
        }
    }
}

impl<T: Component + 'static> QueryParam for Option<&mut T> {
    type Item<'new> = Option<&'new mut T>;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::OptionalMutable);
    }
    fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &mut *world.get() };
        let component = unsafe { world.get_component_mut(&entity) };

        if let Some(component) = component {
            Some(component)
        } else {
            None
        }
    }
}

macro_rules! impl_query_param {
    (
        $($(
            $params:ident
        ),+)?
    ) => {
        #[allow(non_snake_case, unused)]
        impl<
            $($($params: QueryParam),+)?
        > QueryParam for ($($($params,)+)?)
        {
            type Item<'new> = ($($($params::Item<'new>),+)?);
            fn get_components(type_set: &mut impl TypeSet) {
                $($(
                    $params::get_components(type_set);
                )+)?
            }
            fn from_entity<'w>(entity: Entity, world: &'w UnsafeCell<World>) -> Self::Item<'w> {
                ($($($params::from_entity(entity, world)),+)?)
            }
        }
    }
}

impl_query_param!();
impl_query_param!(T1);
impl_query_param!(T1, T2);
impl_query_param!(T1, T2, T3);
impl_query_param!(T1, T2, T3, T4);
impl_query_param!(T1, T2, T3, T4, T5);
impl_query_param!(T1, T2, T3, T4, T5, T6);
impl_query_param!(T1, T2, T3, T4, T5, T6, T7);
impl_query_param!(T1, T2, T3, T4, T5, T6, T7, T8);

macro_rules! impl_read_only_query_param {
    (
        $($(
            $params:ident
        ),+)?
    ) => {
        #[allow(non_snake_case, unused)]
        impl<$($($params: ReadOnlyQueryParam),+)?> ReadOnlyQueryParam for ($($($params,)+)?) {
            fn get_components(with_set: &mut impl TypeSet, without_set: &mut impl TypeSet) {
                $($(
                    $params::get_components(with_set, without_set);
                )+)?
            }
        }
    }
}

impl_read_only_query_param!();
impl_read_only_query_param!(T1);
impl_read_only_query_param!(T1, T2);
impl_read_only_query_param!(T1, T2, T3);
impl_read_only_query_param!(T1, T2, T3, T4);
impl_read_only_query_param!(T1, T2, T3, T4, T5);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6, T7);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6, T7, T8);
