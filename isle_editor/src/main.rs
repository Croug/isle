use isle_ecs::{
    ecs::{
        System,
        IntoSystem,
        ECS,
        SystemParam,
    },
    world::World,
};
use isle_engine::Scheduler;

struct MyResource(pub usize);

impl SystemParam for MyResource {
    fn from_world(world: &mut World) -> Self {
        Self(42)
    }
}

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(my_system);
    ecs.spin();
}

fn my_system(res: MyResource) {
    println!("Res is {}", res.0);
}
