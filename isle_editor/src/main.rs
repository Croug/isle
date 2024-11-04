use std::time::{Instant, UNIX_EPOCH};

use isle::prelude::*;
use isle_engine::{event::EventArgs, params::{Event, EventTrigger}};

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
    let mut flow = Flow::new().with_default_plugins().build();

    flow.add_resource(MyResource(0));

    let entity_a = flow.make_entity();
    let entity_b = flow.make_entity();

    flow.add_component(entity_a, MyComponentOne(69));
    flow.add_component(entity_a, MyComponentTwo);
    flow.add_component(entity_a, MyComponentThree);
    flow.add_component(entity_a, MyComponentFour);

    flow.add_component(entity_b, MyComponentOne(420));
    flow.add_component(entity_b, MyComponentThree);
    flow.add_component(entity_b, MyComponentFour);

    flow.add_postfix_system(my_counting_system);

    flow.add_system(my_complete_system);
    flow.add_system(my_resource_system);
    flow.add_system(my_query_system);
    flow.add_system(my_other_query_system);

    flow.barrier();

    flow.add_system(my_event_signal);
    flow.add_system(my_event_system);

    flow.run();
}

fn my_counting_system(mut res: ResMut<MyResource>) {
    println!("Res is {}", res.0);
    res.0 += 1;
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
    query: Query<(
        Entity,
        &MyComponentOne,
        &MyComponentFour,
        Option<&MyComponentTwo>,
    )>,
) {
    for (entity, one, four, two) in query.iter() {
        println!("<query_system> Entity: {entity:?}, One: {one:?}, Two: {two:?}, Four: {four:?}");
    }
}

fn my_other_query_system(query: Query<Entity, With<MyComponentTwo>>) {
    for entity in query.iter() {
        println!("<other_query_system> Entity: {entity:?}");
    }
}

#[derive(Debug, Clone)]
struct MyEvent(usize);

impl EventArgs for MyEvent {}

fn my_event_system(mut events: Event<MyEvent>) {
    for event in events.iter() {
        println!("<event_system> Event: {event:?}");
    }
}

fn my_event_signal(mut event: EventTrigger<MyEvent>) {
    let now = UNIX_EPOCH.elapsed().unwrap().as_secs();
    println!("Sending event: {now}");
    event.send(MyEvent(now as usize));
}
