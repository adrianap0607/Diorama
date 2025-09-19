

use raylib::prelude::{Image, Vector3, Color};

pub struct Texture {
    img: Image,
    w: i32,
    h: i32,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = Image::load_image(path).expect("no se pudo cargar textura");
        let (w, h) = (img.width(), img.height());
        Self { img, w, h }
    }

    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        let uu = (u.clamp(0.0, 1.0) * (self.w as f32 - 1.0)).round() as usize;
        let vv = ((1.0 - v.clamp(0.0, 1.0)) * (self.h as f32 - 1.0)).round() as usize;

        let data = self.img.get_image_data();
        let idx = vv * (self.w as usize) + uu;
        let c: Color = data[idx];

        Vector3::new(
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
        )
    }
}