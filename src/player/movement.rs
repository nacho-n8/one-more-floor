use crate::player::Player;

use avian::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

const SPEED: f32 = 5.0;
const LOOK_SENSITIVITY: f32 = 0.03;

pub fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 4] = [
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::KeyD, Vec2::X),
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyS, Vec2::NEG_Y),
    ];
    
    MOVEMENT_KEYS.iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, dir)| *dir)
        .sum()
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform), With<Player>>,
) {
    let Ok((mut velocity, transform)) = query.single_mut() else {
        error!("More than one suitable entity for 'move_player'.");
        return;
    };

    let direction = read_movement_input(&input);

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * SPEED;
        let new_velocity = transform.rotation.mul_vec3(Vec3::new(delta.x, 0.0, -delta.y));
        velocity.0 = new_velocity;

        trace!("Player's velocity set to {}.", new_velocity);

        if !velocity.is_finite() {
            error!("Player velocity became invalid.");
            return;
        }
    } else {
        velocity.0 = Vec3::ZERO;
    }
}

pub fn rotate_player(
    mut input: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for event in input.read() {
        mouse_delta = event.delta;
    }

    let Ok(mut transform) = query.single_mut() else {
        error!("More than one suitable entity for 'rotate_player'.");
        return;
    };

    let angle_delta = -mouse_delta.x * LOOK_SENSITIVITY;
    if angle_delta.is_finite() {
        transform.rotate_local_y(angle_delta);

        trace!("Player rotated {} rads.", angle_delta);
        
        if !transform.rotation.is_finite() {
            error!("Player rotation became invalid.");
            return;
        }
    }
}
