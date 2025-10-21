use raylib::prelude::Vector3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::texture::Texture;

pub struct TexturedCube<'a> {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
    pub tex: &'a Texture,
}

impl<'a> TexturedCube<'a> {
    pub fn new(min: Vector3, max: Vector3, material: Material, tex: &'a Texture) -> Self {
        Self { min, max, material, tex }
    }
}

impl<'a> RayIntersect for TexturedCube<'a> {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Intersect {
        fn pair(o: f32, d: f32, mn: f32, mx: f32) -> (f32, f32) {
            if d.abs() < 1e-8 {
                if o < mn || o > mx { return (f32::INFINITY, f32::NEG_INFINITY); }
                return (f32::NEG_INFINITY, f32::INFINITY);
            }
            let t1 = (mn - o) / d; let t2 = (mx - o) / d;
            if t1 <= t2 { (t1, t2) } else { (t2, t1) }
        }
        let (tx1, tx2) = pair(ro.x, rd.x, self.min.x, self.max.x);
        let (ty1, ty2) = pair(ro.y, rd.y, self.min.y, self.max.y);
        let (tz1, tz2) = pair(ro.z, rd.z, self.min.z, self.max.z);
        let t_enter = tx1.max(ty1).max(tz1);
        let t_exit  = tx2.min(ty2).min(tz2);
        if t_enter > t_exit || t_exit < 0.0 { return Intersect::empty(); }

        let t = if t_enter >= 0.0 { t_enter } else { t_exit };
        let p = *ro + *rd * t;

        let eps = 1e-4;
        let (normal, u, v) = if (p.x - self.min.x).abs() < eps {
            let u = (p.z - self.min.z) / (self.max.z - self.min.z);
            let v = (p.y - self.min.y) / (self.max.y - self.min.y);
            (Vector3::new(-1.0, 0.0, 0.0), u, v)
        } else if (p.x - self.max.x).abs() < eps {
            let u = 1.0 - (p.z - self.min.z) / (self.max.z - self.min.z);
            let v = (p.y - self.min.y) / (self.max.y - self.min.y);
            (Vector3::new(1.0, 0.0, 0.0), u, v)
        } else if (p.y - self.min.y).abs() < eps {
            let u = (p.x - self.min.x) / (self.max.x - self.min.x);
            let v = 1.0 - (p.z - self.min.z) / (self.max.z - self.min.z);
            (Vector3::new(0.0, -1.0, 0.0), u, v)
        } else if (p.y - self.max.y).abs() < eps {
            let u = (p.x - self.min.x) / (self.max.x - self.min.x);
            let v = (p.z - self.min.z) / (self.max.z - self.min.z);
            (Vector3::new(0.0, 1.0, 0.0), u, v)
        } else if (p.z - self.min.z).abs() < eps {
            let u = (p.x - self.min.x) / (self.max.x - self.min.x);
            let v = (p.y - self.min.y) / (self.max.y - self.min.y);
            (Vector3::new(0.0, 0.0, -1.0), u, v)
        } else {
            let u = 1.0 - (p.x - self.min.x) / (self.max.x - self.min.x);
            let v = (p.y - self.min.y) / (self.max.y - self.min.y);
            (Vector3::new(0.0, 0.0, 1.0), u, v)
        };

        let (rgb, a_tex) = self.tex.sample_rgba(u, v);
        let mut mat = self.material;
        mat.diffuse = rgb;
        mat.alpha = (a_tex * mat.alpha).max(0.0).min(1.0);

        Intersect::new(p, normal, t, mat)
    }
}