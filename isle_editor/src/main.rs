use isle_ecs::prelude::*;
use isle_engine::{entity::Entity, Scheduler};

struct MyResource(pub usize);

#[derive(Component, Debug)]
struct MyComponentOne;

#[derive(Component, Debug)]
struct MyComponentTwo;

#[derive(Component, Debug)]

struct MyComponentThree;
#[derive(Component, Debug)]
struct MyComponentFour;

fn main() {
    let mut ecs = ECS::new();

    ecs.add_resource(MyResource(42));

    ecs.add_component(Entity(0, 0), MyComponentOne);
    ecs.add_component(Entity(0, 0), MyComponentTwo);
    ecs.add_component(Entity(0, 0), MyComponentThree);
    ecs.add_component(Entity(0, 0), MyComponentFour);

    ecs.add_component(Entity(0, 1), MyComponentOne);
    ecs.add_component(Entity(0, 1), MyComponentThree);
    ecs.add_component(Entity(0, 1), MyComponentFour);

    ecs.add_system(my_resource_system);
    ecs.add_system(my_query_system);
    ecs.add_system(my_complete_system);

    ecs.spin();
}

fn my_complete_system(
    res: ResMut<MyResource>,
    query: Query<(Entity, &MyComponentOne, &MyComponentFour), Without<MyComponentTwo>>,
) {
    println!("Res is {}", res.0);
    for (entity, one, four) in query.iter() {
        println!("<complete_system> Entity: {entity:?}, One: {one:?}, Four: {four:?}");
    }
}

fn my_resource_system(res: Res<MyResource>) {
    println!("Res is {}", res.0);
}

fn my_query_system(
    _query: Query<(
        Entity,
        &MyComponentOne,
        &MyComponentFour,
        Option<&MyComponentTwo>,
    )>,
) {
    for (entity, one, four, two) in _query.iter() {
        println!("<query_system> Entity: {entity:?}, One: {one:?}, Two: {two:?}, Four: {four:?}");
    }
}
