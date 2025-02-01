use bevy::core_pipeline::bloom::{Bloom, BloomPrefilter};
use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_shaders::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShadersPlugin))
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LEDMaterial>>,
) {
    commands.spawn((
        Transform::default(),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(LEDMaterial::default())), // The material
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true, // For using bloom we have to enable hdr
            ..default()
        },
        Transform::from_xyz(5.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom {
            // My custom bloom settings, you can use just Bloom::NATURAL, if you want
            intensity: 0.6,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.9,
            high_pass_frequency: 0.7,
            prefilter: BloomPrefilter {
                threshold: 0.2,
                threshold_softness: 0.1,
            },
            ..Bloom::NATURAL
        },
        FlyCam,
    ));
}
