use crate::player::Player;

use avian::prelude::*;
use bevy::prelude::*;

const SPEED: f32 = 5.0;

pub fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 4] = [
        (KeyCode::ArrowLeft, Vec2::NEG_X),
        (KeyCode::ArrowRight, Vec2::X),
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::ArrowDown, Vec2::NEG_Y),
    ];
    
    MOVEMENT_KEYS.iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, dir)| *dir)
        .sum()
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    let Ok(mut velocity) = query.single_mut() else {
        error!("More than one suitable entity for 'move_player'.");
        return;
    };

    let direction = read_movement_input(&input);

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * SPEED;
        velocity.0 = Vec3::new(delta.x, 0.0, -delta.y);
    } else {
        velocity.0 = Vec3::ZERO;
    }
}
