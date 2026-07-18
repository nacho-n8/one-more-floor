use crate::player::Player;

use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("character/knight.glb"))))
        .insert((
            Player,
            Transform::default(),
        ));
}
