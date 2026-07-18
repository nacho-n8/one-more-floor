use crate::player::Player;

use bevy::prelude::*;

const FOLLOW_OFFSET: Vec3 = Vec3::new(0.0, 2.0, 5.0);
const LOOK_OFFSET: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let mut camera_transform = match camera_query.single_mut() {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    let player_transform = match player_query.single() {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    
    camera_transform.translation = player_transform.translation + FOLLOW_OFFSET;
}

pub fn lookat_player(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let mut camera_transform = match camera_query.single_mut() {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    let player_transform = match player_query.single() {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    camera_transform.look_at(player_transform.translation + LOOK_OFFSET, Vec3::Y);
}
