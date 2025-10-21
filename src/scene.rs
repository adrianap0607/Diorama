// scene.rs
use raylib::prelude::Vector3;

use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::cube_tex::TexturedCube;
use crate::texture::Texture;

pub enum Object<'a> {
    TexCube(TexturedCube<'a>),
}

impl<'a> RayIntersect for Object<'a> {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Intersect {
        match self {
            Object::TexCube(c) => c.ray_intersect(ro, rd),
        }
    }
}

pub struct Scene<'a> {
    pub objects: Vec<Object<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self { Self { objects: Vec::new() } }

    pub fn add_textured_cube(
        &mut self,
        min: Vector3,
        max: Vector3,
        mat: Material,
        tex: &'a Texture,
    ) {
        self.objects.push(Object::TexCube(TexturedCube::new(min, max, mat, tex)));
    }

    pub fn as_refs(&'a self) -> Vec<&'a dyn RayIntersect> {
        self.objects.iter().map(|o| o as &dyn RayIntersect).collect()
    }
}