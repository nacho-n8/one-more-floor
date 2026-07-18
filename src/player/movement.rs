use crate::player::Player;

use bevy::prelude::*;

const SPEED: f32 = 3.0;

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
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        error!("More than one suitable entity for 'move_player'.");
        return;
    };

    let direction = read_movement_input(&input);

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * SPEED * time.delta_secs();
        transform.translation += Vec3::new(delta.x, 0.0, -delta.y);
    }
}
