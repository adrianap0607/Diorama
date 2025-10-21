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
            return 1.0;
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
    let mut nearest = Intersect::empty();
    let mut z = f32::INFINITY;

    for obj in objects {
        let i = obj.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < z {
            z = i.distance;
            nearest = i;
        }
    }

    if !nearest.is_intersecting {
        return procedural_sky(*ray_direction);
    }

    let l = (light.position - nearest.point).normalized();
    let ndotl = nearest.normal.dot(l).max(0.0);

    let shadow = cast_shadow(&nearest, light, objects);

    let diffuse_intensity = ndotl * light.intensity * (1.0 - shadow);
    let diffuse = nearest.material.diffuse * diffuse_intensity;

    let hemi_strength = 0.3;
    let up = nearest.normal.y.clamp(0.0, 1.0);
    let sky = Vector3::new(0.60, 0.80, 1.00);
    let ground = Vector3::new(0.30, 0.25, 0.20);
    let ambient_color = ground * (1.0 - up) + sky * up;
    let ambient = mul_vec3(nearest.material.diffuse, ambient_color) * hemi_strength;

    let base = diffuse + ambient;

    if nearest.material.alpha < 1.0 {
        let behind_origin = nearest.point + *ray_direction * 1e-3;
        let behind = cast_ray(&behind_origin, ray_direction, objects, light);
        return over_rgb(base, behind, nearest.material.alpha);
    }

    base
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

    // render paralelo por filas
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

    // copiar buffer al framebuffer (main thread)
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
    let tex_trunk      = Texture::from_file("assets/tronco.jpeg");
    let tex_leaves     = Texture::from_file("assets/cherry_leaves.png");
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

    // referencias para render
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

        // zoom con trackpad
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