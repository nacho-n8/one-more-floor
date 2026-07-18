use crate::player::Player;

use std::f32::consts::PI;

use avian::prelude::*;
use bevy::prelude::*;

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
            }
    ));
});
}
