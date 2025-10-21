mod framebuffer;
mod ray_intersect;
mod camera;
mod light;
mod material;
mod texture;
mod scene;
mod world;
mod cube_tex;

use raylib::prelude::{Color, Vector3, KeyboardKey, MouseButton, TraceLogLevel};
use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::vector3_to_color;
use std::f32::consts::PI;
use scene::Scene;
use world::{build_grass_and_water, add_tree};
use texture::Texture;
use rayon::prelude::*;

const WATER_TINT: (f32, f32, f32) = (0.3, 0.5, 1.0);

fn procedural_sky(dir: Vector3) -> Vector3 {
    let t = 0.5 * (dir.y + 1.0);
    let sky_color    = Vector3::new(0.60, 0.80, 1.00);
    let ground_color = Vector3::new(0.85, 0.90, 0.75);
    ground_color * (1.0 - t) + sky_color * t
}

fn over_rgb(fg: Vector3, bg: Vector3, alpha: f32) -> Vector3 {
    let a = alpha.clamp(0.0, 1.0);
    fg * a + bg * (1.0 - a)
}

fn mul_vec3(a: Vector3, b: Vector3) -> Vector3 {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 { v - n * 2.0 * v.dot(n) }

fn refract(v: Vector3, n: Vector3, ior: f32) -> Option<Vector3> {
    let mut nrm = n;
    let mut cosi = (-v.dot(nrm)).clamp(-1.0, 1.0);
    let (etai, etat) = if cosi > 0.0 { (1.0, ior) } else { (ior, 1.0) };
    if cosi < 0.0 { nrm = -nrm; cosi = -cosi; }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 { None } else { Some(v * eta + nrm * (eta * cosi - k.sqrt())) }
}

fn fresnel_schlick(cosi: f32, ior: f32) -> f32 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosi.abs()).powi(5)
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[&dyn RayIntersect],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();

    let shadow_origin = intersect.point + intersect.normal * 1e-4;

    for obj in objects {
        let hit = obj.ray_intersect(&shadow_origin, &light_dir);
        if hit.is_intersecting && hit.distance < light_distance {
            if hit.material.alpha >= 0.85 {
                return 1.0; 
            } else {
                return 0.4; 
                
            }
        }
    }
    0.0
}

fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[&dyn RayIntersect],
    light: &Light,
) -> Vector3 {
    cast_ray_rr(ray_origin, ray_direction, objects, light, 4)
}

fn cast_ray_rr(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[&dyn RayIntersect],
    light: &Light,
    depth: i32,
) -> Vector3 {
    if depth <= 0 { return procedural_sky(*ray_direction); }

    let mut nearest = Intersect::empty();
    let mut z = f32::INFINITY;
    for obj in objects {
        let i = obj.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < z { z = i.distance; nearest = i; }
    }
    if !nearest.is_intersecting { return procedural_sky(*ray_direction); }

    let l = (light.position - nearest.point).normalized();
    let ndotl = nearest.normal.dot(l).max(0.0);
    let shadow = cast_shadow(&nearest, light, objects);

    let kd = nearest.material.albedo[0];
    let ks = nearest.material.albedo[1];
    let kr = nearest.material.albedo[2];
    let kt = nearest.material.albedo[3];

    let diffuse = nearest.material.diffuse * (ndotl * light.intensity * (1.0 - shadow)) * kd;

    let v = (-*ray_direction).normalized();
    let h = (l + v).normalized();
    let spec = ks * light.intensity * h.dot(nearest.normal).max(0.0).powf(nearest.material.specular);
    let specular = Vector3::new(spec, spec, spec);

    let hemi = 0.3;
    let up = nearest.normal.y.clamp(0.0, 1.0);
    let sky = Vector3::new(0.60, 0.80, 1.00);
    let ground = Vector3::new(0.30, 0.25, 0.20);
    let ambient_color = ground * (1.0 - up) + sky * up;
    let ambient = mul_vec3(nearest.material.diffuse, ambient_color) * hemi * kd;

    let mut color = diffuse + ambient + specular;

    if kr > 0.0 {
        let rdir = reflect(*ray_direction, nearest.normal).normalized();
        let ro = nearest.point + rdir * 1e-3;
        let rc = cast_ray_rr(&ro, &rdir, objects, light, depth - 1);
        color = color * (1.0 - kr) + rc * kr;
    }

    if nearest.material.alpha < 1.0 {
        let behind_origin = nearest.point + *ray_direction * 1e-3;
        let behind = cast_ray_rr(&behind_origin, ray_direction, objects, light, depth - 1);
        return over_rgb(color, behind, nearest.material.alpha);
    }

    color
}

fn render(framebuffer: &mut Framebuffer, objects: &[&dyn RayIntersect], camera: &Camera, light: &Light) {
    let w = framebuffer.width as usize;
    let h = framebuffer.height as usize;

    let width  = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect = width / height;
    let fov    = PI / 3.0;
    let scale  = (fov * 0.5).tan();

    // buffer para los píxeles
    let mut pixels = vec![Color::BLACK; w * h];

    pixels
        .par_chunks_mut(w)
        .enumerate()
        .for_each(|(y, row)| {
            let sy = -(2.0 * y as f32) / height + 1.0;
            for x in 0..w {
                let sx = (2.0 * x as f32) / width - 1.0;

                let cam_x = sx * aspect * scale;
                let cam_y = sy * scale;

                let ray_dir_cam   = Vector3::new(cam_x, cam_y, -1.0).normalized();
                let ray_dir_world = camera.basis_change(&ray_dir_cam);

                let color_v3 = cast_ray(&camera.eye, &ray_dir_world, objects, light);
                row[x] = vector3_to_color(color_v3, 1.0);
            }
        });

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            framebuffer.set_current_color(pixels[idx]);
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer - Cube Diffuse")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    // cámara
    let mut camera = Camera::new(
        Vector3::new(2.2, 1.8, 10.0),
        Vector3::new(0.0, 0.2, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = std::f32::consts::PI / 100.0;

    // cargar texturas base
    let tex_grass_side = Texture::from_file("assets/grass_side.png");
    let tex_grass_top  = Texture::from_file("assets/topgrass.jpeg");
    let tex_water      = Texture::from_file("assets/water.jpeg");
    let water_tint = Vector3::new(0.3, 0.5, 1.0); // tono más azul
    let tex_trunk      = Texture::from_file("assets/tronco.jpeg");
    let tex_leaves     = Texture::from_file("assets/cherry_leaves.png");
    let leaves_tint = Vector3::new(1.0, 0.6, 0.8);
    let tex_bedrock    = Texture::from_file("assets/bedrock.png");

    // construir escena base con estanque
    let mut scene = Scene::new();
    build_grass_and_water(
        &mut scene,
        &tex_grass_side,
        &tex_grass_top,
        &tex_water,
        &tex_bedrock,
    );

    // árboles
    add_tree(&mut scene,  2.0,  0.0, &tex_trunk, &tex_leaves);
    add_tree(&mut scene,  0.0, -2.0, &tex_trunk, &tex_leaves);
    scene.objects.iter_mut().for_each(|obj| {
        if let crate::scene::Object::TexCube(c) = obj {
            if c.tex as *const _ == &tex_leaves as *const _ {
                c.material.diffuse = leaves_tint;
            }
        }
    });

    let obj_refs = scene.as_refs();

    let light = Light::new(
        Vector3::new(-3.0, 5.0, 3.0),
        Color::WHITE,
        1.2,
    );

    // render principal
    while !window.window_should_close() {
        if window.is_key_down(KeyboardKey::KEY_LEFT)  { camera.orbit( rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { camera.orbit(-rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_UP)    { camera.orbit(0.0, -rotation_speed); }
        if window.is_key_down(KeyboardKey::KEY_DOWN)  { camera.orbit(0.0,  rotation_speed); }

        let mouse_delta = window.get_mouse_delta();
        let sensitivity: f32 = 0.005;

        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            camera.orbit(-mouse_delta.x * sensitivity, -mouse_delta.y * sensitivity);
        }

        let pan_speed: f32 = 0.01;
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            let pan = (camera.right * -mouse_delta.x + camera.up * mouse_delta.y) * pan_speed;
            camera.eye += pan;
            camera.center += pan;
            camera.update_basis_vectors();
        }

        let wheel = window.get_mouse_wheel_move();
        if wheel.abs() > 0.0 {
            let zoom_sens: f32 = 0.6;
            let min_dist: f32 = 2.0;
            let max_dist: f32 = 30.0;

            let to_center = camera.eye - camera.center;
            let dist = to_center.length().max(1e-6);
            let dir = to_center / dist;

            let new_dist = (dist - wheel * zoom_sens).clamp(min_dist, max_dist);
            camera.eye = camera.center + dir * new_dist;
            camera.update_basis_vectors();
        }

        framebuffer.clear();
        render(&mut framebuffer, &obj_refs, &camera, &light);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}