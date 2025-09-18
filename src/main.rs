mod framebuffer;
mod ray_intersect;
mod camera;
mod light;
mod material;
mod cube;
mod plane;

use raylib::prelude::{Color, Vector3, KeyboardKey, MouseButton, TraceLogLevel};
use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use cube::Cube;
use plane::Plane;

use std::f32::consts::PI;

fn procedural_sky(dir: Vector3) -> Vector3 {
    let t = 0.5 * (dir.y + 1.0);
    let sky_color    = Vector3::new(0.60, 0.80, 1.00);
    let ground_color = Vector3::new(0.85, 0.90, 0.75);
    ground_color * (1.0 - t) + sky_color * t
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
    let light_intensity = light.intensity;

    let ndotl = nearest.normal.dot(l).max(0.0);
    let diffuse_intensity = ndotl * light_intensity;
    nearest.material.diffuse * diffuse_intensity
}

fn render(framebuffer: &mut Framebuffer, objects: &[&dyn RayIntersect], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect = width / height;
    let fov = PI / 3.0;
    let scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let sx = (2.0 * x as f32) / width - 1.0;
            let sy = -(2.0 * y as f32) / height + 1.0;

            let cam_x = sx * aspect * scale;
            let cam_y = sy * scale;

            let ray_dir_cam = Vector3::new(cam_x, cam_y, -1.0).normalized();
            let ray_dir_world = camera.basis_change(&ray_dir_cam);

            let color_v3 = cast_ray(&camera.eye, &ray_dir_world, objects, light);
            let color = vector3_to_color(color_v3);

            framebuffer.set_current_color(color);
            framebuffer.set_pixel(x, y);
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

    let mut camera = Camera::new(
        Vector3::new(2.2, 1.6, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = std::f32::consts::PI / 100.0;

    // cubo rojo
    let red_lambert = Material::new(
        Vector3::new(0.9, 0.1, 0.1),
        0.0,
        [1.0, 0.0, 0.0, 0.0],
        0.0,
    );

    // cubo tamaño 2
    let cube = Cube::new(
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new( 1.0,  1.0,  1.0),
        red_lambert,
    );

    // solo difusa
    let objects: [&dyn RayIntersect; 1] = [&cube];

    let light = Light::new(
        Vector3::new(-3.0, 5.0, 3.0),
        Color::WHITE,
        1.2,
    );

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
        if wheel != 0.0 {
            let zoom_speed: f32 = 0.5;
            let delta = camera.forward * wheel * zoom_speed;
            camera.eye += delta;
            camera.center += delta;
            camera.update_basis_vectors();
        }

        framebuffer.clear();
        render(&mut framebuffer, &objects, &camera, &light);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}