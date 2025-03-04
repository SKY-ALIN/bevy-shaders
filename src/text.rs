use crate::color::{bevy_color_into_vec4, MY_BLUE_COLOR, MY_BLUE_GLASS_COLOR};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use phf::phf_map;

/// Only GPU-side rendred text, you don't have to render founts to texture and pass it to the shader.
///
/// For light effect like here it uses emission, so don't forget to enable [bloom shader](https://bevy-cheatbook.github.io/graphics/bloom.html).
/// It uses it's one encoding - many characters of utf8 missed. If it doesn't have comething you need, make a [PR](https://github.com/SKY-ALIN/bevy-shaders/pulls).
///
/// Example:
///
/// ```rust
/// const BUFFER_SIZE: usize = 4; // The buffer size. If the text length might be differen, set the maximum amount of characters
/// let mut buffer = ShaderStorageBuffer::with_size(BUFFER_SIZE, RenderAssetUsages::default());
/// buffer.set_data(TextData::<BUFFER_SIZE>::new("37°C")); // Set the data for the buffer we will send to the shade. Updating this buffer you can update the text
/// let material = TextMaterial::new(buffers.add(buffer))
///     .width(250.0) // width for the character calculation. If you imagine that plane is an led monitor, this will be the amount of pixels by width
///     .height(90.0) // width for the character calculation. If you imagine that plane is an led monitor, this will be the amount of pixels by height
///     .char_width(50.0) // The character width in pixels we set before
///     .char_height(70.0) // The character height in pixels we set before
///     .gap(10.0) // The gap between character in pixels we set before
///     .margin(10.0) // The margin from borders in pixels we set before
///     .emission(3.0); // Emission for the light effect. If you use emission, you must enable Bloom shader, as in this example
/// ```
///
/// ![Shader Preview](media/text.png)
///
/// To specify colors use following functions:
///
/// ```rust
/// let material = material
///     .color(Color::WHITE)
///     .background_color(Color::BLACK)
/// ```
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TextMaterial {
    #[uniform(0)]
    width: f32,
    #[uniform(1)]
    height: f32,
    #[uniform(2)]
    char_width: f32,
    #[uniform(3)]
    char_height: f32,
    #[uniform(4)]
    margin: f32,
    #[uniform(5)]
    rotation: u32,
    #[uniform(6)]
    color: Vec4,
    #[uniform(7)]
    background_color: Vec4,
    #[uniform(8)]
    gap: f32,
    #[storage(9, read_only)]
    chars: Handle<ShaderStorageBuffer>,
}

impl TextMaterial {
    pub fn new(buffer: Handle<ShaderStorageBuffer>) -> Self {
        Self {
            width: 5.0,
            height: 7.0,
            char_width: 5.0,
            char_height: 7.0,
            margin: 0.0,
            rotation: 0,
            color: bevy_color_into_vec4(MY_BLUE_COLOR),
            background_color: bevy_color_into_vec4(MY_BLUE_GLASS_COLOR),
            gap: 2.0,
            chars: buffer,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn char_width(mut self, char_width: f32) -> Self {
        self.char_width = char_width;
        self
    }

    pub fn char_height(mut self, char_height: f32) -> Self {
        self.char_height = char_height;
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn rotation(mut self, rotation: u32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = bevy_color_into_vec4(color);
        self
    }

    pub fn emission(mut self, intensity: f32) -> Self {
        let rgb = self.color.xyz() * intensity;
        self.color.x = rgb.x;
        self.color.y = rgb.y;
        self.color.z = rgb.z;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = bevy_color_into_vec4(color);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl Material for TextMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/text.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Text data container that encapsulates encoding logic.
///
/// We need it to convert `&str` into `[u32; N]` according to the internal encoding.
/// N - must be the same as buffer size.
///
/// Example:
///
/// ```rust
/// let data = TextData::<4>::new("37°C");  // 4 because we have only 4 chars
///
/// // Let's create a buffer that will be passed into shader
/// let mut buffer = ShaderStorageBuffer::with_size(4, RenderAssetUsages::default());
/// // And pass the data
/// buffer.set_data(data);
/// ```
///
/// If the text length might be different, set the maximum amount of characters for the buffer.
/// Changing the buffer you will change the rendered text.
#[derive(Debug, Clone, ShaderType)]
pub struct TextData<const N: usize> {
    pub count: u32,
    pub chars: [u32; N],
}

const ENCODING: phf::Map<char, u32> = phf_map! {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'A' => 10,
    'B' => 11,
    'C' => 12,
    'D' => 13,
    'E' => 14,
    'F' => 15,
    'G' => 16,
    'H' => 17,
    'I' => 18,
    'J' => 19,
    'K' => 20,
    'L' => 21,
    'M' => 22,
    'N' => 23,
    'O' => 24,
    'P' => 25,
    'Q' => 26,
    'R' => 27,
    'S' => 28,
    'T' => 29,
    'U' => 30,
    'V' => 31,
    'W' => 32,
    'X' => 33,
    'Y' => 34,
    'Z' => 35,

    '?' => 36,
    '°' => 37,
};

impl<const N: usize> TextData<N> {
    pub fn new(s: &str) -> Self {
        const DEFAULT: u32 = 36;
        let mut chars = [DEFAULT; N];

        for (i, ch) in s.chars().take(N).enumerate() {
            chars[i] = *ENCODING.get(&ch).unwrap_or(&DEFAULT);
        }

        Self {
            count: s.len() as u32,
            chars,
        }
    }
}
