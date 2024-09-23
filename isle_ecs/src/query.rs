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

pub struct Query<'w, T, V = ()> 
where
    T: QueryParam,
    V: ReadOnlyQueryParam,
{
    marker: PhantomData<&'w (T, V)>,
}

impl<T, V> SystemParam for Query<'_, T, V>
where
    T: QueryParam + 'static,
    V: ReadOnlyQueryParam + 'static,
{
    type Item<'new> = Query<'new, T, V>;
    fn from_world<'w>(_world: &'w mut World) -> Self::Item<'w> {
        Query::<'w, T, V> {
            marker: PhantomData::<&'w (T,V)>,
        }
    }
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam {
    type Item<'new>;

    fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>;
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

impl<T: Component> QueryParam for &T
{
    type Item<'new> = &'new T;
    fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>
    {
        unsafe { world.get_component_mut::<T>(entity).unwrap() }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<T: Component> QueryParam for &mut T {
    type Item<'new> = &'new mut T;
    fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>
    {
        let comp = unsafe { world.get_component_mut::<T>(entity).unwrap() };

        &mut *comp
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Mutable)]
    }
}

impl<T: Component> QueryParam for Option<&T> {
    type Item<'new> = Option<&'new T>;
    fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>
    {
        world.get_component::<T>(entity)
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalImmutable)]
    }
}

impl<T: Component> QueryParam for Option<&mut T> {
    type Item<'new> = Option<&'new mut T>;
    fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>
    {
        unsafe { world.get_component_mut::<T>(entity) }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalMutable)]
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
        where $($(
            for<'a> $params: QueryParam<Item<'a>=$params>,
        )+)?
        {
            type Item<'new> = ($($($params::Item<'new>),+)?);
            fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
                let world: *mut World = world;
                unsafe { ($($($params::from_world(entity, &mut *world)),+)?) }
            }
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
