mod spawn;
mod movement;

use crate::player::{
    spawn::spawn_player,
    movement::{move_player, rotate_player},
};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (rotate_player, move_player));
    }
}
