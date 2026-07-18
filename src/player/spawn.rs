use crate::player::Player;

use std::f32::consts::PI;

use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("character/knight.glb"))))
        .insert((
            Player,
            Transform::from_scale(Vec3::splat(0.7)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI, 0.0)),
        ));
}
