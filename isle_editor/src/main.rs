use std::{
    f32::consts::PI,
    path::PathBuf,
    str::FromStr,
    time::{Instant, UNIX_EPOCH},
};

use geode::{
    camera::CameraCreationSettings, geometry::Geometry, material, plugin::components::{Camera, Material, Mesh, SpotLight}, renderer::{self, Renderer}, texture::Texture
};
use isle::{isle_engine::{flow::stages, params::Tick}, prelude::*};
use isle_ecs::command::WorldCommand;
use isle_engine::{
    input::{
        define_axis_binding, define_binding, Axis, AxisMapping, Button, InputMap, Key, Mapping,
    },
    params::{Event, EventTrigger, Input, InputAxis},
};
use isle_math::{rotation::{quaternion::Quaternion, Angle}, vector::d3::Vec3};

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

    flow.add_resource(false);
    flow.add_resource(Vec3::ZERO);

    flow.push_system(setup);
    flow.push_system(update_light);
    flow.push_system(move_light_target);

    flow.run().unwrap();
}

fn random_orientation() -> Quaternion {
    let random = rand::random::<f32>;
    let x = random() * 2.0 - 1.0;
    let y = random() * 2.0 - 1.0;
    let z = random() * 2.0 - 1.0;
    let w = random() * 2.0 - 1.0;

    Quaternion(x, y, z, w).norm()
}

fn setup(renderer: Option<ResMut<Renderer>>, mut flow: WorldCommand, mut run: ResMut<bool>) {
    if *run {
        return;
    }

    let mut renderer = match renderer {
        Some(renderer) => renderer,
        None => return,
    };

    let camera = Entity(0, 0);
    flow.add_component(camera, Camera::new(&CameraCreationSettings::default()));

    let cube_size = Vec3(100.0, 100.0, 100.0);
    let cube = Geometry::cube(cube_size);
    // cube.load_to_gpu(renderer.device());
    let cube = renderer.add_geometry(cube);

    let mut texture = Texture::new(&PathBuf::from_str("assets/happy_tree.png").unwrap());
    texture.load_to_mem().unwrap();
    texture.load_to_gpu(renderer.device(), renderer.queue());
    let texture = renderer.add_texture(texture);

    let material = geode::material::Material::default_shader(&renderer);
    let material = renderer.add_material(material);
    let material_instance = renderer.instantiate_material(material, "Material", &texture);

    // let entity = Entity(0, 1);
    // flow.add_component(entity, Mesh::new(cube));
    // flow.add_component(entity, Material::new(material, material_instance));
    // flow.add_component(entity, Transform::identity());
    
    let padding = 50.;
    let num_cubes_x = 10usize;
    let num_cubes_y = 10usize;

    let x_span = cube_size.0 + padding;
    let y_span = cube_size.1 + padding;

    let len_x = (x_span * num_cubes_x as f32) - x_span;
    let len_y = (y_span * num_cubes_y as f32) - y_span;

    let start_x = -len_x / 2.0;
    let start_y = -len_y / 2.0;

    let mut n = 0;

    for x in 0..num_cubes_x {
        for y in 0..num_cubes_y {
            let pos = Vec3(
                start_x + (x as f32 * x_span),
                0.,
                start_y + (y as f32 * y_span),
            );

            let entity = Entity(1, n);
            flow.add_component(entity, Mesh::new(cube));
            flow.add_component(entity, Material::new(material, material_instance));
            flow.add_component(entity, Transform::new(pos, random_orientation().into(), Vec3::IDENTITY));
            n += 1;
        }
    }

    let position = Vec3(0.0, 500., -500.0);
    let light = Entity(0, 1);
    flow.add_component(light, SpotLight::new(
        Vec3::IDENTITY,
        300.0,
        Angle::Degrees(15.),
        Angle::Degrees(13.),
    ));
    flow.add_component(light, Transform::new(
        position,
        Quaternion::look_at(&position, &Vec3::ZERO).into(),
        Vec3::IDENTITY,
    ));

    *run = true;
}

fn update_light(look_at: Res<Vec3>, query: Query<&mut Transform, With<SpotLight>>) {
    let Some(light) = query.iter().next() else {
        return;
    };

    let position = light.position();
    light.set_rotation(Quaternion::look_at(&position, &look_at).into());
}

define_binding!(Up, Key::Up | Key::W);
define_binding!(Down, Key::Down | Key::S);
define_binding!(Left, Key::Left | Key::A);
define_binding!(Right, Key::Right | Key::D);

const CAMERA_SPEED: f32 = 500.;

fn move_light_target(tick: Tick, up: Input<Up>, down: Input<Down>, left: Input<Left>, right: Input<Right>, mut look_at: ResMut<Vec3>) {
    look_at.2 += tick.delta() * CAMERA_SPEED * (up.state() as i32 - down.state() as i32) as f32;
    look_at.0 += tick.delta() * CAMERA_SPEED * (right.state() as i32 - left.state() as i32) as f32;
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

    flow.add_system(stages::POST_RUN, my_counting_system);

    // flow.add_system(my_complete_system);
    // flow.add_system(my_resource_system);
    // flow.add_system(my_query_system);
    // flow.add_system(my_other_query_system);

    // flow.barrier();

    // flow.add_system(my_event_signal);
    // flow.add_system(my_event_system);

    flow.push_system(my_input_system);
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
        _ => (),
    }
    input.set_axis(Axis::LeftStickX, STEP * (counter as f32).sin());
}
