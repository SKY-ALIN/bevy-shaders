use bevy::asset::RenderAssetUsages;
use bevy::core_pipeline::bloom::{Bloom, BloomPrefilter};
use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;
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
    mut materials: ResMut<Assets<TextMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    const BUFFER_SIZE: usize = 4; // The buffer size. If the text length might be differen, set the maximum amount of characters
    let mut buffer = ShaderStorageBuffer::with_size(BUFFER_SIZE, RenderAssetUsages::default());
    buffer.set_data(TextData::<BUFFER_SIZE>::new("37°C")); // Set the data for the buffer we will send to the shade. Updating this buffer you can update the text
    let material = TextMaterial::new(buffers.add(buffer))
        .width(250.0) // width for the character calculation. If you imagine that plane is an led monitor, this will be the amount of pixels by width
        .height(90.0) // width for the character calculation. If you imagine that plane is an led monitor, this will be the amount of pixels by height
        .char_width(50.0) // The character width in pixels we set before
        .char_height(70.0) // The character height in pixels we set before
        .gap(10.0) // The gap between character in pixels we set before
        .margin(10.0) // The margin from borders in pixels we set before
        .emission(3.0); // Emission for the light effect. If you use emission, you must enable Bloom shader, as in this example

    commands.spawn((
        Transform::default(),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(3.0, 1.0)))),
        MeshMaterial3d(materials.add(material)), // The material
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true, // For using bloom and emission we have to enable hdr
            ..default()
        },
        Transform::from_xyz(10.0, 1.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
