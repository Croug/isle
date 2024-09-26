use std::{any::{type_name, TypeId}, cell::UnsafeCell, collections::HashSet, marker::PhantomData};

use isle_engine::entity::Entity;

use crate::{component::Component, ecs::{
    BorrowSignature, RefType, SystemParam, TypeSet
}, world::World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterType {
    With,
    Without,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct  FilterSignature(TypeId, FilterType);

pub trait FilterSet {
    fn insert_type<T: 'static>(&mut self, filter_type: FilterType);
}

impl FilterSet for HashSet<FilterSignature> {
    fn insert_type<T: 'static>(&mut self, filter_type: FilterType) {
        let type_id = TypeId::of::<T>();
        if !self.insert(FilterSignature(type_id, filter_type)) {
            panic!("Duplicate type in dependency list\nType {} appears at least twice", type_name::<T>());
        }
    }
}

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
        let entities = HashSet::<Entity>::new();
        let mut interactable_components = HashSet::<BorrowSignature>::new();
        let mut filter_components = HashSet::<FilterSignature>::new();

        T::get_components(&mut interactable_components);
        V::get_components(&mut filter_components);

        todo!()
    }
    pub fn iter(&self) -> impl Iterator<Item = T::Item<'w>> + '_ {
        let entities = self.fetch_entities();

        entities.into_iter().map(move |entity| {
            T::from_entity(entity, &self.world)
        })
    }
}

impl<'__w, T, V> SystemParam for Query<'__w, T, V>
where
    T: QueryParam + 'static,
    V: ReadOnlyQueryParam + 'static,
{
    type Item<'new> = Query<'new, T, V>;
    fn from_world<'w>(world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        Query::<T, V> {
            world: &world,
            marker: PhantomData::<(T,V)>,
        }
    }
    fn collect_types(types: &mut impl crate::ecs::TypeSet) -> () {
        let mut _component_set = HashSet::<BorrowSignature>::new();
        let mut _filter_set = HashSet::<FilterSignature>::new();
        T::get_components(&mut _component_set);
        V::get_components(&mut _filter_set);
        types.insert_type::<Query<T,V>>(RefType::Immutable);
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
    fn get_components(filter_set: &mut impl FilterSet) -> ();
}

impl<T: Component + 'static> ReadOnlyQueryParam for With<T> {
    fn get_components(filter_set: &mut impl FilterSet) -> () {
        filter_set.insert_type::<T>(FilterType::With);
    }
}

impl<T: Component + 'static> ReadOnlyQueryParam for Without<T> {
    fn get_components(filter_set: &mut impl FilterSet) -> () {
        filter_set.insert_type::<T>(FilterType::Without);
    }
}

impl<T: Component + 'static> QueryParam for &T
{
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
            fn get_components(filter_set: &mut impl FilterSet) {
                $($(
                    $params::get_components(filter_set);
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
