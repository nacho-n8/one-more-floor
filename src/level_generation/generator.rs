use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::AMBER_500,
    prelude::*
};
use rand::random_bool;

const ROOM_WIDTH: usize = 5;
const ROOM_LENGTH: usize = 5;

const FLOOR_DENSITY: f64 = 0.9;
const WALL_DENSITY: f64 = 0.1;

fn generate_floorplan() -> [[bool; ROOM_WIDTH]; ROOM_LENGTH] {
    let mut floorplan = [[false; ROOM_WIDTH]; ROOM_LENGTH];
    for (row, columns) in floorplan.iter_mut().enumerate() {
        for (column, tile) in columns.iter_mut().enumerate() {
            if row == 0 && column == 0 { // Corner tile will always spawn
                *tile = true;
                continue;
            }

            *tile = random_bool(FLOOR_DENSITY);
        }
    }

    floorplan
}

fn determine_walls(row: usize, column: usize) -> [bool; 4] {
    let mut walls = [false; 4];
    for (wall, do_spawn) in walls.iter_mut().enumerate() {
        // Ensure outer walls
        if wall == 0 && row == 0 {
            *do_spawn = true;
            continue;
        } else if wall == 1 && column == ROOM_WIDTH - 1 {
            *do_spawn = true;
            continue;
        } else if wall == 2 && row == ROOM_LENGTH - 1 {
            *do_spawn = true;
            continue;
        } else if wall == 3 && column == 0 {
            *do_spawn = true;
            continue;
        }

        *do_spawn = random_bool(WALL_DENSITY);
    }

    walls
}

pub fn spawn_room(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let floorplan = generate_floorplan();

    let mut spawn_z = 0.0;
    for (row, columns) in floorplan.iter().enumerate() {
        let mut spawn_x = 0.0;
        for (column, tile) in columns.iter().enumerate() {
            if *tile == true {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x, 0.0, spawn_z)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/floor_wood_large.gltf")))
                ));
                // Create  walls
                for (i, wall) in determine_walls(row, column).iter().enumerate() {
                    if *wall == true {
                        match i {
                            0 => {
                                commands.spawn(( // top wall
                                    Transform::from_translation(Vec3::new(spawn_x, 0.0, spawn_z + 2.0)),
                                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                                ));
                            },
                            1 => {
                                commands.spawn(( // right wall
                                    Transform::from_translation(Vec3::new(spawn_x + 2.0, 0.0, spawn_z)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                                ));
                            },
                            2 => {
                                commands.spawn(( // bottom wall
                                    Transform::from_translation(Vec3::new(spawn_x, 0.0, spawn_z - 2.0)),
                                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                                ));
                            },
                            3 => { 
                                commands.spawn(( // left wall
                                    Transform::from_translation(Vec3::new(spawn_x - 2.0, 0.0, spawn_z)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))),
                                ));
                            },
                            _ => warn!("Not sure why it got to this point...")
                        }
                    }
                }
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
