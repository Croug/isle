pub mod matrix {
    pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

    pub type Mat4<T> = Matrix<T, 4, 4>;
    pub type Mat3<T> = Matrix<T, 3, 3>;
    pub type Mat2<T> = Matrix<T, 2, 2>;
}
pub mod vector {
    pub struct Vec3(pub f32, pub f32, pub f32);
    pub struct Vec2(pub f32, pub f32);
}