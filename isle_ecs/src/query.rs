use std::{collections::HashSet, marker::PhantomData};

use crate::{component::Component, ecs::{
    BorrowSignature, RefType, SystemParam, TypeSet
}, world::World};

pub struct Query<T, V = ()> 
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    marker: PhantomData<(T, V)>,
}

impl<T, V> SystemParam for Query<T, V>
where
    T: QueryParam + 'static,
    V: ReadOnlyQueryParam + 'static,
{
    type Item<'new> = Query<T, V>;
    fn from_world<'w>(_world: &'w mut World) -> Self::Item<'w> {
        Query::<T, V> {
            marker: PhantomData::<(T,V)>,
        }
    }
    fn collect_types(types: &mut impl crate::ecs::TypeSet) -> () {
        let mut _set = HashSet::<BorrowSignature>::new();
        T::get_components(&mut _set);
        V::get_components(&mut _set);
        types.insert_type::<Query<T,V>>(RefType::Immutable);
    }
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam {
    type Readonly<'new>;

    // fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Readonly<'w>;
    fn get_components(type_set: &mut impl TypeSet) -> ();
}

pub trait ReadOnlyQueryParam {
    fn get_components(type_set: &mut impl TypeSet) -> ();
}

impl<T: Component + 'static> ReadOnlyQueryParam for With<T> {
    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Present);
    }
}

impl<T: Component + 'static> ReadOnlyQueryParam for Without<T> {
    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Absent);
    }
}

impl<T: Component + 'static> QueryParam for &T
{
    type Readonly<'new> = &'new T;
        
    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Immutable);
    }
}

impl<T: Component + 'static> QueryParam for &mut T {
    type Readonly<'new> = &'new T;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::Mutable);
    }
}

impl<T: Component + 'static> QueryParam for Option<&T> {
    type Readonly<'new> = Option<&'new T>;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::OptionalImmutable);
    }
}

impl<T: Component + 'static> QueryParam for Option<&mut T> {
    type Readonly<'new> = Option<&'new T>;

    fn get_components(type_set: &mut impl TypeSet) -> () {
        type_set.insert_type::<T>(RefType::OptionalMutable);
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
        > QueryParam for ($($($params),+)?)
        {
            type Readonly<'new> = ($($($params::Readonly<'new>),+)?);
            // fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            //     let world: *mut World = world;
            //     unsafe { ($($($params::from_world(entity, &mut *world)),+)?) }
            // }
            fn get_components(type_set: &mut impl TypeSet) {
                $($(
                    $params::get_components(type_set);
                )+)?
            }
        }
    }
}

impl_query_param!();
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
        impl<$($($params: ReadOnlyQueryParam),+)?> ReadOnlyQueryParam for ($($($params),+)?) {
            fn get_components(type_set: &mut impl TypeSet) {
                $($(
                    $params::get_components(type_set);
                )+)?
            }
        }
    }
}

impl_read_only_query_param!();
impl_read_only_query_param!(T1, T2);
impl_read_only_query_param!(T1, T2, T3);
impl_read_only_query_param!(T1, T2, T3, T4);
impl_read_only_query_param!(T1, T2, T3, T4, T5);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6, T7);
impl_read_only_query_param!(T1, T2, T3, T4, T5, T6, T7, T8);
