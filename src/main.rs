mod camera;
mod level_generation;
mod player;
mod temp;

use crate::{
    level_generation::LevelGenerationPlugin,
    player::PlayerPlugin
};

use avian::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .add_plugins((LevelGenerationPlugin, PlayerPlugin))
        .add_systems(Update, grab_mouse);

    #[cfg(feature="debug_physics")]
    app.add_plugins(avian::debug_render::PhysicsDebugPlugin);

    app.run();
}

fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) || mouse_input.just_pressed(MouseButton::Right) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
