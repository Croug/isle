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
    T: QueryParam<'static>,
    V: ReadOnlyQueryParam,
{
    marker: PhantomData<(T, V)>,
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam<'a> {
    fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized;
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

impl<'a, T: Component + 'static> QueryParam<'a> for &'a T {
    fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized
    {
        unsafe { world.get_component_mut::<T>(entity).unwrap() }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<'a, T: Component + 'static> QueryParam<'a> for &'a mut T {
    fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized
    {
        let comp = unsafe { world.get_component_mut::<T>(entity).unwrap() };

        &mut *comp
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Mutable)]
    }
}

impl<'a, T: Component + 'static> QueryParam<'a> for Option<&'a T> {
    fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized
    {
        world.get_component::<T>(entity)
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalImmutable)]
    }
}

impl<'a, T: Component + 'static> QueryParam<'a> for Option<&'a mut T> {
    fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized
    {
        unsafe { world.get_component_mut::<T>(entity) }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalMutable)]
    }
}

impl<T, V> SystemParam for Query<T, V>
where
    T: QueryParam<'static>,
    V: ReadOnlyQueryParam,
{
    fn from_world(_world: &mut World) -> Self {
        Self {
            marker: PhantomData,
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
        impl<'a,
            $($($params: QueryParam<'a>),+)?
        > QueryParam<'a> for ($($($params),+)?) {
            fn from_world(entity: &Entity, world: &'a mut World) -> Self where Self: Sized {
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
