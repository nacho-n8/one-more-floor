use bevy::prelude::*;

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 2.0, 4.0)).looking_at(Vec3::splat(0.0), Vec3::Y),
    ));
}
