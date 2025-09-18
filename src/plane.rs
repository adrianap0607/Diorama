

use raylib::prelude::Vector3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub base_material: Material,
}

impl Plane {
    pub fn new(point: Vector3, normal: Vector3, base_material: Material) -> Self {
        Self { point, normal: normal.normalized(), base_material }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Intersect {
        let denom = self.normal.dot(*rd);
        if denom.abs() < 1e-6 {
            return Intersect::empty(); 
        }
        let t = (self.point - *ro).dot(self.normal) / denom;
        if t <= 0.0 { return Intersect::empty(); }

        let hit = *ro + *rd * t;

        let mut mat = self.base_material;
        mat.diffuse = Vector3::new(0.22, 0.68, 0.18); 

        let n = if denom < 0.0 { self.normal } else { -self.normal };

        Intersect::new(hit, n, t, mat)
    }
}