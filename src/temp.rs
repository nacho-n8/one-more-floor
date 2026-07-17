use std::f32::consts::PI;
use bevy::{
    color::palettes::tailwind::AMBER_500,
    prelude::*
};

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 28.0, 0.0)).looking_at(Vec3::splat(0.0), Vec3::Y),
    ));
}

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(GlobalAmbientLight {
        color: AMBER_500.into(),
        brightness: 200.0,
        ..default()
    });
    
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(-4.0, 0.0, -8.0)));
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, -8.0)));
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(4.0, 0.0, -8.0)));

    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(-4.0, 0.0, 8.0)));
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 8.0)));
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform::from_translation(Vec3::new(4.0, 0.0, 8.0)));

    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(-8.0, 0.0, -4.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(-8.0, 0.0, 0.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(-8.0, 0.0, 4.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );

    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(8.0, 0.0, -4.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(8.0, 0.0, 0.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(8.0, 0.0, 4.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );

    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall_corner.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(-8.0, 0.0, -8.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall_corner.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(8.0, 0.0, -8.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall_corner.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(-8.0, 0.0, 8.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI, 0.0))
        );
    commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/wall_corner.gltf"))))
        .insert(Transform
            ::from_translation(Vec3::new(8.0, 0.0, 8.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0 + PI, 0.0))
        );

    let mut i = 0;
    while i < 5 {
        let mut j = 0;
        while j < 5 {
            commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/floor_wood_large.gltf"))))
                .insert(Transform::from_translation(Vec3::new(-8.0 + j as f32 * 4.0, 0.0, -8.0 + i as f32 * 4.0)));

            j += 1;
        }

        i += 1;
    }

    let mut i = 0;
    while i < 5 {
        let mut j = 0;
        while j < 5 {
            commands.spawn(WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/floor_wood_large.gltf"))))
                .insert(Transform::from_translation(Vec3::new(-8.0 + j as f32 * 4.0, 4.0, -8.0 + i as f32 * 4.0)));

            j += 1;
        }

        i += 1;
    }

}
