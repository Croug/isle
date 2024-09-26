use std::cell::UnsafeCell;

use isle_ecs::{prelude::*, world::World};
use isle_engine::Scheduler;

struct MyResource(pub usize);

#[derive(Debug)]
struct MyComponentOne;

#[derive(Debug)]
struct MyComponentTwo;

#[derive(Debug)]

struct MyComponentThree;
#[derive(Debug)]
struct MyComponentFour;

impl SystemParam for &MyResource {
    type Item<'new> = &'new MyResource;

    fn from_world(_world: &UnsafeCell<World>) -> Self::Item<'_> {
        &MyResource(42)
    }

    fn collect_types(types: &mut impl isle_ecs::ecs::TypeSet) {
        types.insert_type::<MyResource>(isle_ecs::ecs::RefType::Immutable);
    }
}

fn main() {
    let mut ecs = ECS::new();

    ecs.add_system(my_resource_system);
    ecs.add_system(my_query_system);
    ecs.add_system(my_complete_system);

    ecs.spin();
}

fn my_complete_system(res: &MyResource, query: Query<(&MyComponentOne, &MyComponentFour)>) {
    println!("Res is {}", res.0);
    for (one, four) in query.iter() {
        println!("One is {:?}", one);
        println!("Four is {:?}", four);
    }
}

fn my_resource_system(res: &MyResource) {
    println!("Res is {}", res.0);
}

fn my_query_system(_query: Query<(&MyComponentOne, &MyComponentFour)>) {}
