use std::marker::PhantomData;
use super::world::World;
use isle_engine::Scheduler;

pub struct ECS {
    systems: Vec<Box<dyn System>>,
    world: World,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            world: World::new(),
        }
    }
    pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }
}

impl Scheduler for ECS {
    fn spin(&mut self) -> () {
        for system in self.systems.iter_mut() {
            system.run(&mut self.world)
        }
    }
}

pub trait System {
    fn run(&mut self, world: &mut World);
}

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

pub trait SystemParam {
    type Item<'new>;

    fn from_world<'w>(world: &'w mut World) -> Self::Item<'w>;
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
        impl<$($($params),+)?> SystemParam for ($($($params),+)?)
        where $($(
            for<'a> $params: SystemParam<Item<'a>=$params>
        ),+)?
        {
            type Item<'new> = ($($($params::Item<'new>),+)?);

            fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
                $($(
                    let $params = {
                        let world: &mut World = unsafe { &mut *(world as *mut World) };
                        $params::from_world(world)
                    };
                )+)?

                ($($($params),+)?)
            }
        }
    }
}

impl_system_param!();
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
            fn run(&mut self, world: &mut World) {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*),
                    $($params: $params),*
                ) {
                    f($($params),*);
                }

                $(
                    let $params = {
                        let world: &mut World = unsafe { &mut *(world as *mut World) };
                        $params::from_world(world)
                    };
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

