use isle_ecs::{command::WorldCommand, ecs::{Res, ResMut}, query::Query};
use isle_engine::{prelude::Transform, window::WINDOW};
use isle_math::vector::d2::Vec2;

use crate::{camera::CameraCreationSettings, lighting, renderer::Renderer};

use super::components::{Camera, Material, Mesh, PointLight, SpotLight};

pub fn setup(mut command: WorldCommand) {
    let window = WINDOW.get().unwrap();
    let size = window.inner_size();
    let size = Vec2(size.width as f32, size.height as f32);
    let renderer = Renderer::new(window, CameraCreationSettings {
        viewport: size,
        ..Default::default()
    });

    let renderer = pollster::block_on(renderer).unwrap();

    command.add_resource(renderer);
}

pub fn update_cameras(cameras: Query<(&Camera, &Transform)>, mut renderer: ResMut<Renderer>) {
    cameras
        .iter()
        .filter(|(camera, transform)| camera.dirty || transform.dirty())
        .for_each(|(camera, transform)| {
            let render_camera = renderer.camera_mut(camera.id);

            if camera.dirty {
                render_camera.update_projection(camera.znear, camera.zfar, camera.projection);
            }

            if transform.dirty() {
                render_camera.update_view(&transform.position(), &transform.orientation(), &transform.scale());
            }

        });
}

pub fn update_lights(point_lights: Query<(&PointLight, &Transform)>, spot_lights: Query<(&SpotLight, &Transform)>, mut renderer: ResMut<Renderer>) {
    let lights = renderer.lighting_mut();
    point_lights
        .iter()
        .filter(|(light, transform)| light.dirty || transform.dirty())
        .for_each(|(light, transform)| {
            lights.update_point_light(light.id, lighting::PointLight {
                position: transform.position(),
                color: light.color,
                intensity: light.intensity,
            });
        });

    spot_lights
        .iter()
        .filter(|(light, transform)| light.dirty || transform.dirty())
        .for_each(|(light, transform)| {
            lights.update_spot_light(light.id, lighting::SpotLight {
                position: transform.position(),
                color: light.color,
                intensity: light.intensity,
                direction: transform.orientation().forward(),
                outer: light.outer,
                inner: light.inner,
            })
        });
}

pub fn update_instances(query: Query<(&Mesh, &Material, &Transform)>, mut renderer: ResMut<Renderer>) {
    query
        .iter()
        .filter(|(_, _, transform)| transform.dirty())
        .for_each(|(mesh, material, transform)| {
            let geometry = renderer.geometry_mut(mesh.geometry);
            geometry.update_instance(material.material, mesh.instance, transform.position(), transform.orientation(), transform.scale());
        });
}