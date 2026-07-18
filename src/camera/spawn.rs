use std::f32::consts::PI;

use bevy::prelude::*;

const TRANSLATION_OFFSET: Vec3 = Vec3::new(0.0, 2.0, 5.0);
const ROTATION_OFFSET_EULER: Vec3 = Vec3::new(PI / 4.0, 0.0, 0.0);

pub fn spawn_camera(
    mut commands: Commands,
) {
    let euler_array = ROTATION_OFFSET_EULER.to_array();

    println!("{:?}", euler_array);

    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: TRANSLATION_OFFSET,
            rotation: Quat::from_euler(EulerRot::XYZ, euler_array[0], euler_array[1], euler_array[2]),
            scale: default(),
        }
    ));

    debug!("Camera queued to spawn.");
}
