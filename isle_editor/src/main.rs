use std::{f32::consts::PI, time::{Instant, UNIX_EPOCH}};

use geode::{camera::CameraCreationSettings, geometry::Geometry, plugin::components::Camera, renderer::Renderer};
use isle::prelude::*;
use isle_engine::{input::{define_axis_binding, define_binding, Axis, AxisMapping, Button, InputMap, Key, Mapping}, params::{Event, EventTrigger, Input, InputAxis}};
use isle_math::vector::d3::Vec3;

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
    let mut flow = Flow::new()
        .with_default_plugins()
        .build();

    let camera = flow.make_entity();
    flow.add_component(camera, Camera::new(&CameraCreationSettings::default()));
    let renderer = flow.get_resource_mut::<Renderer>().unwrap();

    let mut cube = Geometry::cube(Vec3(100., 100., 100.));
    cube.load_to_gpu(renderer.device());
    let cube = renderer.add_geometry(cube);
}

fn main_old() {
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

    // flow.add_system(my_complete_system);
    // flow.add_system(my_resource_system);
    // flow.add_system(my_query_system);
    // flow.add_system(my_other_query_system);

    // flow.barrier();

    // flow.add_system(my_event_signal);
    // flow.add_system(my_event_system);

    flow.add_system(my_input_system);
    // flow.add_system(my_fake_input);

    flow.run();
}

fn my_counting_system(mut res: ResMut<MyResource>) {
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

#[derive(Debug, Clone, Copy)]
struct MyEvent(usize);

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

// struct MyMapping;

// impl Mapping for MyMapping {
//     fn keys<'a>() -> &'a [Key] {
//         &[Key::A, Key::B, Key::C]
//     }

//     fn buttons<'a>() -> &'a [Button] {
//         &[Button::North, Button::South]
//     }
// }

define_binding!(MyMapping, Key::Up | Key::W);

// struct MyAxisMapping;

// impl AxisMapping for MyAxisMapping {
//     fn axes<'a>() -> &'a [Axis] {
//         &[Axis::LeftStickX, Axis::RightStickX]
//     }
//     fn positive_keys<'a>() -> &'a [Key] {
//         &[Key::D, Key::Right]
//     }
//     fn positive_buttons<'a>() -> &'a [Button] {
//         &[Button::PadRight]
//     }
//     fn negative_keys<'a>() -> &'a [Key] {
//         &[Key::A, Key::Left]
//     }
//     fn negative_buttons<'a>() -> &'a [Button] {
//         &[Button::PadLeft]
//     }
// }

type Forward = MyMapping;
type Backward = ();

define_axis_binding!(
    MyAxisMapping,
    Axis::LeftStickX | Axis::RightStickX,
    Forward,
    Backward
);

fn my_input_system(input: Input<MyMapping>, input_axis: InputAxis<MyAxisMapping>) {
    if input.just_changed() {
        println!("Edge detected!");
    }
    if input.state() {
        println!("Input detected!");
    }
    
    println!("Axis value: {}", input_axis.value());
}

const STEP: f32 = PI / 32.;

fn my_fake_input(mut input: ResMut<InputMap>, counter: Res<MyResource>) {
    let MyResource(counter) = *counter;
    match counter {
        5 => input.set_key(Key::A, true),
        6 => input.set_key(Key::B, true),
        10 => input.set_key(Key::A, false),
        12 => input.set_key(Key::B, false),
        15 => input.set_button(Button::North, true),
        20 => input.set_button(Button::North, false),
        25 => panic!("Exiting"),
        _ => ()
    }
    input.set_axis(Axis::LeftStickX, STEP * (counter as f32).sin());
}
