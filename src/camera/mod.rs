#![deprecated(since="0.2.1", note="The camera is now a child of the player.")]

#![allow(dead_code)]
#![allow(deprecated)]

mod follow;
pub mod spawn;

use crate::camera::spawn::spawn_camera;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
