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
    fn from_world(_world: &mut World) -> Self {
        Self(42)
    }
}

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(my_system);
    ecs.spin();
}

fn my_system(
    res: MyResource,
    _query: Query<
        (&'static mut MyComponentOne, &'static MyComponentTwo),
        (With<MyComponentThree>, Without<MyComponentFour>)
    >
) {
    println!("Res is {}", res.0);
}
