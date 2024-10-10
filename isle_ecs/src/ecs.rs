use std::{
    any::{type_name, TypeId},
    cell::UnsafeCell,
    collections::HashSet,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{component::Component, entity::Entity, world::World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, Eq)]
pub struct BorrowSignature(pub TypeId, pub RefType);

impl PartialEq for BorrowSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::hash::Hash for BorrowSignature {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct ECS {
    systems: Vec<Box<dyn System>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }
    pub fn add_component<T: Component + 'static>(
        &mut self,
        entity: Entity,
        component: T,
        world: &mut World,
    ) {
        world.store_component(entity, component);
    }
    pub fn add_resource<T: 'static>(&mut self, resource: T, world: &mut World) {
        world.store_resource(resource);
    }
    pub fn get_system_ids(&self) -> Vec<usize> {
        self.systems.iter().enumerate().map(|(i, _)| i).collect()
    }
    pub fn run_system_by_id(&mut self, id: usize, world: &UnsafeCell<World>) {
        self.systems[id].run(world);
    }
}

pub trait System {
    fn run(&mut self, world: &UnsafeCell<World>);
}

pub trait TypeSet {
    fn insert_type<T: 'static>(&mut self, ref_type: RefType);
}

impl TypeSet for HashSet<BorrowSignature> {
    fn insert_type<T: 'static>(&mut self, ref_type: RefType) {
        let type_id = TypeId::of::<T>();
        if !self.insert(BorrowSignature(type_id, ref_type)) {
            panic!(
                "Duplicate type in dependency list\nType {} appears at least twice",
                type_name::<T>()
            );
        }
    }
}

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

pub trait SystemParam {
    type Item<'new>;

    fn from_world<'w>(world: &'w UnsafeCell<World>) -> Self::Item<'w>;
    fn collect_types(types: &mut impl TypeSet);
}

pub struct Res<'a, T: 'static>(&'a T);

impl<'a, T> Deref for Res<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> SystemParam for Res<'a, T> {
    type Item<'new> = Res<'new, T>;

    fn from_world<'w>(world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &*world.get() };
        Res(world.get_resource::<T>().unwrap())
    }

    fn collect_types(types: &mut impl TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
}

pub struct ResMut<'a, T: 'static>(&'a mut T);

impl<'a, T> Deref for ResMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for ResMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: 'static> SystemParam for ResMut<'a, T> {
    type Item<'new> = ResMut<'new, T>;

    fn from_world<'w>(world: &'w UnsafeCell<World>) -> Self::Item<'w> {
        let world = unsafe { &mut *world.get() };
        ResMut(unsafe { world.get_resource_mut::<T>().unwrap() })
    }

    fn collect_types(types: &mut impl TypeSet) {
        types.insert_type::<T>(RefType::Mutable);
    }
}

pub struct StoredSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_system_param {
    (
        $(
            $($params:ident),+
        )?
    ) => {
        #[allow(non_snake_case, unused)]
        impl<$($($params),+)?> SystemParam for ($($($params,)+)?)
        where $($(
            for<'a> $params: SystemParam<Item<'a>=$params>
        ),+)?
        {
            type Item<'new> = ($($($params::Item<'new>),+)?);

            fn from_world<'w>(world: &'w UnsafeCell<World>) -> Self::Item<'w> {
                $($(
                    let $params = $params::from_world(unsafe { &*world });
                )+)?

                ($($($params),+)?)
            }

            fn collect_types(types: &mut impl TypeSet) {
                $($(
                    $params::collect_types(types);
                )+)?
            }
        }
    }
}

impl_system_param!();
impl_system_param!(T1);
impl_system_param!(T1, T2);
impl_system_param!(T1, T2, T3);
impl_system_param!(T1, T2, T3, T4);
impl_system_param!(T1, T2, T3, T4, T5);
impl_system_param!(T1, T2, T3, T4, T5, T6);
impl_system_param!(T1, T2, T3, T4, T5, T6, T7);
impl_system_param!(T1, T2, T3, T4, T5, T6, T7, T8);

macro_rules! impl_system {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case, unused)]
        impl<F, $($params: SystemParam),*> System for StoredSystem<($($params,)*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut( $($params),* ) +
                FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            fn run(&mut self, world: &UnsafeCell<World>) {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*),
                    $($params: $params),*
                ) {
                    f($($params),*);
                }

                $(
                    let $params = $params::from_world(&world);
                )*

                call_inner(&mut self.f, $($params),*);
            }
        }
    }
}

impl_system!();
impl_system!(T1);
impl_system!(T1, T2);
impl_system!(T1, T2, T3);
impl_system!(T1, T2, T3, T4);
impl_system!(T1, T2, T3, T4, T5);
impl_system!(T1, T2, T3, T4, T5, T6);
impl_system!(T1, T2, T3, T4, T5, T6, T7);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8);

macro_rules! impl_into_system {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: SystemParam),*> IntoSystem<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut( $($params),* ) +
                FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            type System = StoredSystem<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                let mut _set = HashSet::<BorrowSignature>::new();
                $($params::collect_types(&mut _set);)*
                StoredSystem {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

impl_into_system!();
impl_into_system!(T1);
impl_into_system!(T1, T2);
impl_into_system!(T1, T2, T3);
impl_into_system!(T1, T2, T3, T4);
impl_into_system!(T1, T2, T3, T4, T5);
impl_into_system!(T1, T2, T3, T4, T5, T6);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8);
