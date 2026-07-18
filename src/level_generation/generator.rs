use std::f32::consts::PI;

use avian::prelude::*;
use bevy::{
    color::palettes::tailwind::AMBER_500,
    prelude::*
};
use rand::random_bool;

const ROOM_WIDTH: usize = 7;
const ROOM_LENGTH: usize = 7;

const FLOOR_DENSITY: f64 = 0.9;
const WALL_DENSITY: f64 = 0.07;

#[derive(Default, Clone, Copy)]
struct Tile {
    floor: bool,
    top_wall: bool,
    bottom_wall: bool,
    left_wall: bool,
    right_wall: bool,
}

fn generate_floorplan() -> [[Tile; ROOM_WIDTH]; ROOM_LENGTH] {
    let mut floorplan = [[Tile::default(); ROOM_WIDTH]; ROOM_LENGTH];

    for (row, columns) in floorplan.iter_mut().enumerate() {
        for (column, tile) in columns.iter_mut().enumerate() {
            if row == 0 && column == 0 { // Corner tile will always spawn
                tile.floor = true;
            } else {
                tile.floor = random_bool(FLOOR_DENSITY);
            }

            if !tile.floor {
                tile.top_wall = true;
                tile.bottom_wall = true;
                tile.left_wall = true;
                tile.right_wall = true;
                continue;
            }

            for i in 0..4 {
                let do_spawn = random_bool(WALL_DENSITY);

                if i == 0 {
                    tile.top_wall = do_spawn;
                } else if i == 1 {
                    tile.right_wall = do_spawn;
                } else if i == 2 {
                    tile.bottom_wall = do_spawn;
                } else if i == 3 {
                    tile.left_wall = do_spawn;
                }
            }

            if row == 0 { tile.bottom_wall = true; }
            if row == ROOM_LENGTH - 1 { tile.top_wall = true; }
            if column == 0 { tile.left_wall = true; }
            if column == ROOM_WIDTH -1 { tile.right_wall = true; }
        }
    }

    floorplan
}

pub fn spawn_room(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let floorplan = generate_floorplan();

    let mut spawn_z = 0.0;
    for columns in floorplan.iter() {
        let mut spawn_x = 0.0;
        for tile in columns.iter() {
            if tile.floor {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x, 0.0, spawn_z)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/floor_wood_large.gltf")))
                ));
            }

            if tile.top_wall {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x, 2.0, spawn_z - 2.0)),
                    RigidBody::Static,
                    Collider::cuboid(4.0, 4.0, 1.0)
                )).with_child((
                    Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                ));
            }
            if tile.bottom_wall {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x, 2.0, spawn_z + 2.0)),
                    RigidBody::Static,
                    Collider::cuboid(4.0, 4.0, 1.0)
                )).with_child((
                    Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                ));
            }
            if tile.left_wall {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x - 2.0, 2.0, spawn_z)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    RigidBody::Static,
                    Collider::cuboid(4.0, 4.0, 1.0)
                )).with_child((
                    Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                ));
            }
            if tile.right_wall {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x + 2.0, 2.0, spawn_z)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    RigidBody::Static,
                    Collider::cuboid(4.0, 4.0, 1.0)
                )).with_child((
                    Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                ));
            }

            spawn_x += 4.0;
        }
        spawn_z -= 4.0;
    }

    commands.insert_resource(GlobalAmbientLight {
        color: AMBER_500.into(),
        brightness: 200.0,
        ..default()
    });
}
