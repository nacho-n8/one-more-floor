use bevy::{
    color::palettes::tailwind::AMBER_500,
    prelude::*
};
use rand::random_bool;

const ROOM_WIDTH: usize = 5;
const ROOM_LENGTH: usize = 5;

const ROOM_DENSITY: f64 = 0.9;

fn generate_floorplan() -> [[bool; ROOM_WIDTH]; ROOM_LENGTH] {
    let mut floorplan = [[false; ROOM_WIDTH]; ROOM_LENGTH];
    for row in floorplan.iter_mut() {
        for tile in row.iter_mut() {
            *tile = random_bool(ROOM_DENSITY);
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
    for row in floorplan.iter() {
        let mut spawn_x = 0.0;
        for tile in row.iter() {
            if *tile == true {
                commands.spawn((
                    Transform::from_translation(Vec3::new(spawn_x, 0.0, spawn_z)),
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/floor_wood_large.gltf")))
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
