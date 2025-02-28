use isle_ecs::{command::WorldCommand, ecs::ResMut, query::Query};
use isle_engine::{prelude::Transform, window::WINDOW};
use isle_math::vector::d2::Vec2;

use crate::{camera::CameraCreationSettings, lighting, renderer::Renderer};

use super::components::{Camera, Material, Mesh, PointLight, SpotLight};

pub fn setup(mut command: WorldCommand) {
    let window = WINDOW.get().unwrap();
    let size = window.inner_size();
    let size = Vec2(size.width as f32, size.height as f32);
    let renderer = Renderer::new(
        window,
        CameraCreationSettings {
            viewport: size,
            ..Default::default()
        },
    );

    let renderer = pollster::block_on(renderer).unwrap();

    command.add_resource(renderer);
}

pub fn update_cameras(cameras: Query<(&mut Camera, &Transform)>, mut renderer: ResMut<Renderer>) {
    cameras
        .iter()
        .filter(|(camera, transform)| camera.dirty || transform.dirty())
        .for_each(|(camera, transform)| {
            let render_camera = renderer.camera_mut(camera.id);

            if camera.dirty {
                render_camera.update_projection(camera.znear, camera.zfar, camera.projection);
                camera.dirty = false;
            }

            if transform.dirty() {
                render_camera.update_view(
                    &transform.position(),
                    &transform.orientation(),
                    &transform.scale(),
                );
            }
        });
}

pub fn update_lights(
    point_lights: Query<(&mut PointLight, &Transform)>,
    spot_lights: Query<(&mut SpotLight, &Transform)>,
    mut renderer: ResMut<Renderer>,
) {
    let lights = renderer.lighting_mut();
    point_lights
        .iter()
        .filter(|(light, transform)| light.dirty || transform.dirty())
        .for_each(|(light, transform)| {
            match light {
                PointLight {
                    id: Some(id),
                    color,
                    intensity,
                    ..
                } => {
                    lights.update_point_light(
                        *id,
                        lighting::PointLight {
                            position: transform.position(),
                            color: *color,
                            intensity: *intensity,
                        },
                    );
                },
                PointLight {
                    id: None,
                    color,
                    intensity,
                    ..
                } => {
                    let id = lights.add_point_light(lighting::PointLight {
                        position: transform.position(),
                        color: *color,
                        intensity: *intensity,
                    });
                    light.id = Some(id);
                }
            }

            light.dirty = false;
        });

    spot_lights
        .iter()
        .filter(|(light, transform)| light.dirty || transform.dirty())
        .for_each(|(light, transform)| {
            // lights.update_spot_light(
            //     light.id,
            //     lighting::SpotLight {
            //         position: transform.position(),
            //         color: light.color,
            //         intensity: light.intensity,
            //         direction: transform.orientation().forward(),
            //         outer: light.outer,
            //         inner: light.inner,
            //     },
            // );
            match light {
                SpotLight {
                    id: Some(id),
                    color,
                    intensity,
                    outer,
                    inner,
                    ..
                } => {
                    lights.update_spot_light(
                        *id,
                        lighting::SpotLight {
                            position: transform.position(),
                            color: *color,
                            intensity: *intensity,
                            direction: transform.orientation().forward(),
                            outer: *outer,
                            inner: *inner,
                        },
                    );
                },
                SpotLight {
                    id: None,
                    color,
                    intensity,
                    outer,
                    inner,
                    ..
                } => {
                    let id = lights.add_spot_light(lighting::SpotLight {
                        position: transform.position(),
                        color: *color,
                        intensity: *intensity,
                        direction: transform.orientation().forward(),
                        outer: *outer,
                        inner: *inner,
                    });
                    light.id = Some(id);
                }
            }

            light.dirty = false;
        });
}

pub fn update_instances(
    query: Query<(&mut Mesh, &Material, &Transform)>,
    mut renderer: ResMut<Renderer>,
) {
    query
        .iter()
        .filter(|(mesh, _, transform)| (transform.dirty() || mesh.dirty) && mesh.instance.is_some())
        .for_each(|(mesh, material, transform)| {
            let geometry = renderer.geometry_mut(mesh.geometry);
            geometry.update_instance(
                material.material,
                mesh.instance.unwrap(),
                transform.position(),
                transform.orientation(),
                transform.scale(),
            );
            mesh.dirty = false;
        });
}

pub fn create_geometries(
    instances: Query<(&mut Mesh, &Material, &Transform)>,
    mut renderer: ResMut<Renderer>,
) {
    instances
        .iter()
        .filter(|(mesh, _, _)| mesh.instance.is_none())
        .for_each(|(mesh, material, transform)| {
            mesh.instance = Some(renderer.instantiate_geometry(
                mesh.geometry,
                material.material,
                material.instance,
                transform.position(),
                transform.orientation(),
                transform.scale(),
            ));
        });
}
