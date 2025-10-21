use raylib::prelude::{Color, Vector3};

// estructura de material
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Vector3,
    pub albedo: [f32; 4],
    pub specular: f32,
    pub refractive_index: f32,
    pub alpha: f32,
}

impl Material {
    pub fn new(diffuse: Vector3, specular: f32, albedo: [f32; 4], refractive_index: f32, alpha: f32) -> Self {
        Material { diffuse, albedo, specular, refractive_index, alpha }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Vector3::zero(),
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 0.0,
            alpha: 1.0,
        }
    }
}

pub fn vector3_to_color(v: Vector3, alpha: f32) -> Color {
    Color::new(
        (v.x * 255.0).min(255.0) as u8,
        (v.y * 255.0).min(255.0) as u8,
        (v.z * 255.0).min(255.0) as u8,
        (alpha * 255.0).min(255.0) as u8,
    )
}