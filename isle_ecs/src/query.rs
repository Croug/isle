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

pub struct Query<'a, T, V = ()> 
where
    T: QueryParam<'a>,
    V: QueryParam<'a>,
{
    marker: PhantomData<&'a (T, V)>,
}

pub struct With<T>(PhantomData<T>);
pub struct Without<T>(PhantomData<T>);

pub trait QueryParam<'a> {
    fn from_world(entity: &Entity, world: &'a World) -> Self;
    fn get_components() -> Vec<(TypeId, RefType)>;
}

impl<'a, T: Component> QueryParam<'a> for With<T> {
    fn from_world(_entity: &Entity, _world: &'a World) -> Self {
        Self(PhantomData)
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<'a, T: Component> QueryParam<'a> for Without<T> {
    fn from_world(_entity: &Entity, _world: &'a World) -> Self {
        Self(PhantomData)
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<'a, T: Component> QueryParam<'a> for &'a T {
    fn from_world(entity: &Entity, world: &'a World) -> Self {
        world.get_component::<T>(entity).unwrap()
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Immutable)]
    }
}

impl<'a, T: Component> QueryParam<'a> for &'a mut T {
    fn from_world(entity: &Entity, world: &'a World) -> Self {
        unsafe { world.get_component_mut::<T>(entity).unwrap() }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::Mutable)]
    }
}

impl<'a, T: Component> QueryParam<'a> for Option<&'a T> {
    fn from_world(entity: &Entity, world: &'a World) -> Self {
        world.get_component::<T>(entity)
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalImmutable)]
    }
}

impl<'a, T: Component> QueryParam<'a> for Option<&'a mut T> {
    fn from_world(entity: &Entity, world: &'a World) -> Self {
        unsafe { world.get_component_mut::<T>(entity) }
    }
    fn get_components() -> Vec<(TypeId, RefType)> {
        vec![(TypeId::of::<T>(), RefType::OptionalMutable)]
    }
}

impl<'a, T, V> SystemParam for Query<'a, T, V>
where
    T: QueryParam<'a>,
    V: QueryParam<'a>,
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
            fn from_world(entity: &Entity, world: &'a World) -> Self {
                ($($($params::from_world(entity, world)),+)?)
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

