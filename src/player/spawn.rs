use crate::player::Player;

use std::f32::consts::PI;

use avian::prelude::*;
use bevy::prelude::*;

const CAMERA_FOLLOW_OFFSET: Vec3 = Vec3::new(0.5, 1.5, 3.0);
const CAMERA_ROTATION_EULER: Vec3 = Vec3::new(-PI / 10.0, 0.0, 0.0);

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        RigidBody::Dynamic,
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule(0.5, 0.5),
        LinearVelocity::ZERO,
    )).with_children(|parent| {
        parent.spawn((
            WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("character/knight.glb"))),
            Transform {
                translation: Vec3::new(0.0, -1.0, 0.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, PI, 0.0),
                scale: Vec3::splat(0.7),
            },
        ));
        let euler_array = CAMERA_ROTATION_EULER.to_array();
        parent.spawn((
            Camera3d::default(),
            Transform {
                translation: CAMERA_FOLLOW_OFFSET,
                rotation: Quat::from_euler(EulerRot::XYZ, euler_array[0], euler_array[1], euler_array[2]),
                scale: Vec3::ONE,
            },
        ));
    });

    debug!("Player queued to spawn.");
}
