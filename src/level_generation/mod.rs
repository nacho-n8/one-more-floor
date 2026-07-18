mod generator;

use crate::level_generation::generator::spawn_room;

use bevy::prelude::*;

pub struct LevelGenerationPlugin;

impl Plugin for LevelGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_room);
    }
}
