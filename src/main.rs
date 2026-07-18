mod temp;
mod player;

use crate::player::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (temp::spawn_camera, temp::spawn_level))
        .run();
}
