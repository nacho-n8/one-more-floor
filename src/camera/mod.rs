mod follow;
mod spawn;

use crate::camera::{
    follow::{follow_player, lookat_player},
    spawn::spawn_camera,
};

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (follow_player, lookat_player.after(follow_player)));
    }
}
