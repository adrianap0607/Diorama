// cube.rs
use raylib::prelude::Vector3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, material: Material) -> Self {
        Self { min, max, material }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_dir: &Vector3) -> Intersect {
        // cubo
        fn pair(o: f32, d: f32, minv: f32, maxv: f32) -> (f32, f32) {
            if d.abs() < 1e-8 {
                let t1 = f32::NEG_INFINITY;
                let t2 = f32::INFINITY;
                if o < minv || o > maxv { return (f32::INFINITY, f32::NEG_INFINITY); }
                return (t1, t2);
            }
            let t1 = (minv - o) / d;
            let t2 = (maxv - o) / d;
            if t1 <= t2 { (t1, t2) } else { (t2, t1) }
        }

        let (tx1, tx2) = pair(ray_origin.x, ray_dir.x, self.min.x, self.max.x);
        let (ty1, ty2) = pair(ray_origin.y, ray_dir.y, self.min.y, self.max.y);
        let (tz1, tz2) = pair(ray_origin.z, ray_dir.z, self.min.z, self.max.z);

        let t_enter = tx1.max(ty1).max(tz1);
        let t_exit  = tx2.min(ty2).min(tz2);

        if t_enter > t_exit || t_exit < 0.0 {
            return Intersect::empty();
        }

        let t = if t_enter >= 0.0 { t_enter } else { t_exit };
        let point = *ray_origin + *ray_dir * t;

        let eps = 1e-4;
        let mut normal = Vector3::zero();
        if (point.x - self.min.x).abs() < eps { normal = Vector3::new(-1.0, 0.0, 0.0); }
        else if (point.x - self.max.x).abs() < eps { normal = Vector3::new( 1.0, 0.0, 0.0); }
        else if (point.y - self.min.y).abs() < eps { normal = Vector3::new(0.0, -1.0, 0.0); }
        else if (point.y - self.max.y).abs() < eps { normal = Vector3::new(0.0,  1.0, 0.0); }
        else if (point.z - self.min.z).abs() < eps { normal = Vector3::new(0.0, 0.0, -1.0); }
        else {                                           normal = Vector3::new(0.0, 0.0,  1.0); }

        Intersect::new(point, normal, t, self.material)
    }
}