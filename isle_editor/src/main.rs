use isle_ecs::{
    ecs::{
        System,
        IntoSystem,
        ECS,
        SystemParam,
    },
    world::World,
    query::{Query, With, Without}, component::Component
};
use isle_engine::Scheduler;

struct MyResource(pub usize);

struct MyComponentOne;
struct MyComponentTwo;
struct MyComponentThree;
struct MyComponentFour;

impl SystemParam for MyResource {
    type Item<'new> = Self;

    fn from_world(_world: &mut World) -> Self {
        Self(42)
    }
}

impl SystemParam for &MyResource {
    type Item<'new> = &'new MyResource;

    fn from_world(_world: &mut World) -> Self::Item<'_> {
        &MyResource(42)
    }
}

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(my_resource_system);
    ecs.add_system(my_query_system);
    ecs.add_system(my_complete_system);
    ecs.spin();
}

fn my_complete_system(
    res: &MyResource,
    query: Query< (&MyComponentOne, &MyComponentFour) >
) {

}

fn my_resource_system(
    res: & MyResource,
    res2: & MyResource,
) {
    println!("Res is {}\nRes2 is {}", res.0, res2.0);
}

fn my_query_system(
    _query: Query< (&MyComponentOne, &MyComponentFour) >
) {

}
