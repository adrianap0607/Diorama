use raylib::prelude::Vector3;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vector3, normal: Vector3, distance: f32, material: Material) -> Self {
        let n = if normal.length() > 0.0 { normal.normalized() } else { Vector3::zero() };
        Intersect {
            point,
            normal: n,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zero(),
            normal: Vector3::zero(),
            distance: f32::INFINITY,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}

impl Default for Intersect {
    fn default() -> Self { Self::empty() }
}

pub trait RayIntersect: Sync {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}