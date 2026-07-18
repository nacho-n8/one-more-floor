mod camera;
mod level_generation;
mod player;
mod temp;

use crate::{
    camera::CameraPlugin,
    level_generation::LevelGenerationPlugin,
    player::PlayerPlugin
};

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
        .add_plugins((CameraPlugin, LevelGenerationPlugin, PlayerPlugin))
        .run();
}
