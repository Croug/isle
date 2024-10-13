use isle_math::{matrix::Mat4, vector::Vec3};

pub struct Camera {
    texture_id: usize,
    eye: Vec3,
    target: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    view: Mat4<f32>
}