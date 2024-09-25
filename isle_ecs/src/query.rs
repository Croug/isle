use std::{marker::PhantomData, any::TypeId};

use isle_engine::entity::Entity;

use crate::{ecs::SystemParam, world::World, component::Component};

pub enum RefType {
    Immutable,
    Mutable,
    OptionalImmutable,
    OptionalMutable,
}

impl RefType {
    pub fn make_optional(self) -> Self {
        match self {
            Self::Immutable => Self::OptionalImmutable,
            Self::Mutable => Self::OptionalMutable,
            _ => self,
        }
    }
}

pub struct Query<T, V = ()> 
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    marker: PhantomData<(T, V)>,
}

impl<T, V> SystemParam for Query<T, V>
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    type Item<'new> = Query<T, V>;
    fn from_world<'w>(_world: &'w mut World) -> Self::Item<'w> {
        Query::<T, V> {
            marker: PhantomData::<(T,V)>,
        }
    }
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam {
    type Readonly<'new>;

    // fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Readonly<'w>;
    fn get_components() -> Vec<(TypeId, RefType)>;
}

pub trait ReadOnlyQueryParam {
    fn get_components() -> Vec<(TypeId, RefType)>;
}

impl<T: Component + 'static> ReadOnlyQueryParam for With<T> {
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<T: Component + 'static> ReadOnlyQueryParam for Without<T> {
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<T: Component + 'static> QueryParam for &T
{
    type Readonly<'new> = &'new T;
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<T: Component + 'static> QueryParam for &mut T {
    type Readonly<'new> = &'new T;
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Mutable)]
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
            fn get_components() -> Vec<(TypeId, RefType)> {
                let mut components = Vec::new();
                $($(
                    components.extend($params::get_components());
                )+)?
                components
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
            fn get_components() -> Vec<(TypeId, RefType)> {
                let mut components = Vec::new();
                $($(
                    components.extend($params::get_components());
                )+)?
                components
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
