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
    fn from_world(world: &mut World) -> Self;
}

pub struct StoredSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_system {
    (
        $(
            $($params:ident),+
         )?
    ) => {
        #[allow(non_snake_case, unused)]
        impl<
            F: FnMut(
                $( $($params),+ )?
            )
            $(, $($params: 'static),+ )?
        > System for StoredSystem<($( $($params,)+ )?), F>
        where
            $($(
                $params: SystemParam
            ),+)?
        {
            fn run(&mut self, world: &mut World) {
                $($(
                    let $params = $params::from_world(world);
                )+)?

                (self.f)(
                    $($($params),+)?
                );
            }
        }
    }
}

impl_system!();
impl_system!(T1);
impl_system!(T1, T2);
impl_system!(T1, T2, T3);
impl_system!(T1, T2, T3, T4);

macro_rules! impl_into_system {
    (
        $($($params:ident),+)?
    ) => {
        impl<F: FnMut($($($params),+)?) $(, $($params: 'static),+ )?> IntoSystem<( $($($params,)+)?)> for F
        where
            $($($params: SystemParam),+)?
        {
            type System = StoredSystem<( $($($params,)+)? ), Self>;

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

