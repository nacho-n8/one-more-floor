use bevy::prelude::*;

use crate::player::Player;

const FOLLOW_OFFSET: Vec3 = Vec3::new(0.0, 3.0, 5.0);
const LOOK_OFFSET: Vec3 = Vec3::new(0.0, 2.0, 0.0);

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("More than one suitable entity for 'follow_player'.");
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        error!("More than one suitable entity for 'follow_player'.");
        return;
    };

    camera_transform.translation = player_transform.translation + FOLLOW_OFFSET;
}

pub fn lookat_player(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("More than one suitable entity for 'follow_player'.");
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        error!("More than one suitable entity for 'follow_player'.");
        return;
    };

    camera_transform.look_at(player_transform.translation + LOOK_OFFSET, Vec3::Y);
}
