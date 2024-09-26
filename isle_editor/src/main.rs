use isle_ecs::prelude::*;
use isle_engine::{entity::Entity, Scheduler};

struct MyResource(pub usize);

#[derive(Component, Debug)]
struct MyComponentOne(usize);

#[derive(Component, Debug)]
struct MyComponentTwo;

#[derive(Component, Debug)]

struct MyComponentThree;
#[derive(Component, Debug)]
struct MyComponentFour;

fn main() {
    let mut ecs = ECS::new();

    ecs.add_resource(MyResource(42));

    ecs.add_component(Entity(0, 0), MyComponentOne(69));
    ecs.add_component(Entity(0, 0), MyComponentTwo);
    ecs.add_component(Entity(0, 0), MyComponentThree);
    ecs.add_component(Entity(0, 0), MyComponentFour);

    ecs.add_component(Entity(0, 1), MyComponentOne(420));
    ecs.add_component(Entity(0, 1), MyComponentThree);
    ecs.add_component(Entity(0, 1), MyComponentFour);

    ecs.add_system(my_complete_system);
    ecs.add_system(my_resource_system);
    ecs.add_system(my_query_system);
    ecs.add_system(my_other_query_system);

    ecs.spin();
}

fn my_complete_system(
    mut res: ResMut<MyResource>,
    query: Query<(Entity, &mut MyComponentOne, &MyComponentFour), Without<MyComponentTwo>>,
) {
    println!("Res is {}", res.0);
    res.0 += 1;
    for (entity, one, four) in query.iter() {
        println!("<complete_system> Entity: {entity:?}, One: {one:?}, Four: {four:?}");
        one.0 += 32;
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

fn my_other_query_system(
    query: Query<Entity, With<MyComponentTwo>>
) {
    for entity in query.iter() {
        println!("<other_query_system> Entity: {entity:?}");
    }
}
