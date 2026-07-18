mod camera;
mod player;
mod temp;

use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;

use avian::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            avian::debug_render::PhysicsDebugPlugin::default(),
        ))
        .add_plugins((CameraPlugin, PlayerPlugin))
        .add_systems(Startup,  temp::spawn_level)
        .run();
}
