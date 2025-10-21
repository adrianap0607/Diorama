// world.rs
use raylib::prelude::Vector3;

use crate::material::Material;
use crate::texture::Texture;
use crate::scene::Scene;

pub fn build_grass_and_water<'a>(
    scene: &mut Scene<'a>,
    grass_side: &'a Texture,
    grass_top: &'a Texture,
    water: &'a Texture,
    bedrock: &'a Texture,
) {
    let size = 1.0;

    let mat_white = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        0.0,
        [1.0, 0.0, 0.0, 0.0],
        0.0,
        1.0,
    );

    let mat_water = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        0.0,
        [1.0, 0.0, 0.0, 0.0],
        0.0,
        0.75,
    );

    let water_positions: &[(i32, i32)] = &[
        (-1, 0), (0, 0), (1, 0),
        (0, 1),
    ];

    let is_water = |x: i32, z: i32| -> bool {
        water_positions.iter().any(|&(wx, wz)| wx == x && wz == z)
    };

    // crea la base
    for z in -2..=2 {
        for x in -2..=2 {
            if is_water(x, z) { continue; }

            let min_base = Vector3::new(x as f32 * size, -1.0, z as f32 * size);
            let max_base = min_base + Vector3::new(size, 1.0, size);
            scene.add_textured_cube(min_base, max_base, mat_white, grass_side);

            let min_top = Vector3::new(x as f32 * size, 0.0, z as f32 * size);
            let max_top = min_top + Vector3::new(size, 0.06, size);
            scene.add_textured_cube(min_top, max_top, mat_white, grass_top);
        }
    }

    // bloques de piedra
    let mat_bedrock = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        0.0,
        [1.0, 0.0, 0.0, 0.0],
        0.0,
        1.0,
    );

    let raised_positions: &[(i32, i32)] = &[
        (0, -1), (1, -1),
        (1,  0),
    ];

    for &(x, z) in raised_positions {
        let min = Vector3::new(x as f32 * size, 0.06, z as f32 * size);
        let max = min + Vector3::new(size, 0.5, size);
        scene.add_textured_cube(min, max, mat_bedrock, bedrock);
    }

    // agua
    for &(x, z) in water_positions {
        let min_w = Vector3::new(x as f32 * size, -0.2, z as f32 * size);
        let max_w = min_w + Vector3::new(size, 0.2, size);
        scene.add_textured_cube(min_w, max_w, mat_water, water);
    }
}

// árbol normal
pub fn add_tree<'a>(
    scene: &mut Scene<'a>,
    x: f32,
    z: f32,
    trunk_tex: &'a Texture,
    leaf_tex: &'a Texture,
) {
    let size = 1.0;
    let mat_neutral = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        0.0,
        [1.0, 0.0, 0.0, 0.0],
        0.0,
        1.0,
    );

    let min_trunk = Vector3::new(x * size, 0.0, z * size);
    let max_trunk = min_trunk + Vector3::new(size, 2.0, size);
    scene.add_textured_cube(min_trunk, max_trunk, mat_neutral, trunk_tex);

    let h = 0.6;

    let y1 = 2.0;
    let layer1 = [ (0,0), (1,0), (-1,0), (0,1), (0,-1) ];
    for (ox, oz) in layer1 {
        let min = Vector3::new((x + ox as f32) * size, y1, (z + oz as f32) * size);
        let max = min + Vector3::new(size, h, size);
        scene.add_textured_cube(min, max, mat_neutral, leaf_tex);
    }

    let y2 = y1 + h;
    let layer2 = [
        (-1,-1), (0,-1), (1,-1),
        (-1, 0),          (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];
    for (ox, oz) in layer2 {
        let min = Vector3::new((x + ox as f32) * size, y2, (z + oz as f32) * size);
        let max = min + Vector3::new(size, h, size);
        scene.add_textured_cube(min, max, mat_neutral, leaf_tex);
    }

    let y3 = y2 + h;
    let min_top = Vector3::new(x * size, y3, z * size);
    let max_top = min_top + Vector3::new(size, h, size);
    scene.add_textured_cube(min_top, max_top, mat_neutral, leaf_tex);
}

// árbol grande
pub fn add_tree_big<'a>(
    scene: &mut Scene<'a>,
    x: f32,
    z: f32,
    trunk_tex: &'a Texture,
    leaf_tex: &'a Texture,
) {
    let size = 1.0;
    let mat_neutral = Material::new(Vector3::new(1.0, 1.0, 1.0), 0.0, [1.0, 0.0, 0.0, 0.0], 0.0, 1.0);

    let min_trunk = Vector3::new(x * size, 0.0, z * size);
    let max_trunk = min_trunk + Vector3::new(size, 2.0, size);
    scene.add_textured_cube(min_trunk, max_trunk, mat_neutral, trunk_tex);

    let h = 0.6;
    let y1 = 2.0;

    let min_connector = Vector3::new(x * size, y1, z * size);
    let max_connector = min_connector + Vector3::new(size, h, size);
    scene.add_textured_cube(min_connector, max_connector, mat_neutral, leaf_tex);

    let y2 = y1 + h;
    for oz in -1..=1 {
        for ox in -1..=1 {
            let min = Vector3::new((x + ox as f32) * size, y2, (z + oz as f32) * size);
            let max = min + Vector3::new(size, h, size);
            scene.add_textured_cube(min, max, mat_neutral, leaf_tex);
        }
    }

    let y3 = y2 + h;
    let min_top = Vector3::new(x * size, y3, z * size);
    let max_top = min_top + Vector3::new(size, h, size);
    scene.add_textured_cube(min_top, max_top, mat_neutral, leaf_tex);
}

// árbol pequeño
pub fn add_tree_small<'a>(
    scene: &mut Scene<'a>,
    x: f32,
    z: f32,
    trunk_tex: &'a Texture,
    leaf_tex: &'a Texture,
) {
    let size = 1.0;
    let mat_neutral = Material::new(Vector3::new(1.0, 1.0, 1.0), 0.0, [1.0, 0.0, 0.0, 0.0], 0.0, 1.0);

    let min_trunk = Vector3::new(x * size, 0.0, z * size);
    let max_trunk = min_trunk + Vector3::new(size, 1.6, size);
    scene.add_textured_cube(min_trunk, max_trunk, mat_neutral, trunk_tex);

    let h = 0.6;
    let y1 = 1.6;
    let cross = [(1,0), (-1,0), (0,1), (0,-1)];
    for (ox, oz) in cross {
        let min = Vector3::new((x + ox as f32) * size, y1, (z + oz as f32) * size);
        let max = min + Vector3::new(size, h, size);
        scene.add_textured_cube(min, max, mat_neutral, leaf_tex);
    }

    let y2 = y1 + h;
    let min_top = Vector3::new(x * size, y2, z * size);
    let max_top = min_top + Vector3::new(size, h, size);
    scene.add_textured_cube(min_top, max_top, mat_neutral, leaf_tex);
}