mod temp;

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
        .add_systems(Startup, (temp::spawn_camera, temp::spawn_level))
        .run();
}
