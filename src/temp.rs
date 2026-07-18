use bevy::{
    color::palettes::tailwind::AMBER_500,
    prelude::*
};

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(GlobalAmbientLight {
        color: AMBER_500.into(),
        brightness: 200.0,
        ..default()
    });

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
}
