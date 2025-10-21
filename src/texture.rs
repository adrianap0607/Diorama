use raylib::prelude::{Image, Color};
use raylib::prelude::Vector3;

pub struct Texture {
    w: i32,
    h: i32,
    data: Vec<Color>,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        // cargar imagen
        let img = Image::load_image(path).expect("no se pudo cargar textura");
        let (w, h) = (img.width(), img.height());
        let data: Vec<Color> = img.get_image_data().to_vec();
        Self { w, h, data }
    }

    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        // muestra color
        let uu = (u.clamp(0.0, 1.0) * (self.w as f32 - 1.0)).floor() as usize;
        let vv = ((1.0 - v.clamp(0.0, 1.0)) * (self.h as f32 - 1.0)).floor() as usize;
        let idx = vv * (self.w as usize) + uu;
        let c = self.data[idx];
        Vector3::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0)
    }

    pub fn sample_rgba(&self, u: f32, v: f32) -> (Vector3, f32) {
        let uu = (u.clamp(0.0, 1.0) * (self.w as f32 - 1.0)).floor() as usize;
        let vv = ((1.0 - v.clamp(0.0, 1.0)) * (self.h as f32 - 1.0)).floor() as usize;
        let idx = vv * (self.w as usize) + uu;
        let c = self.data[idx];
        let rgb = Vector3::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0);
        let a = c.a as f32 / 255.0;
        (rgb, a)
    }
}