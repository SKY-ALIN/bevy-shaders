use bevy::math::vec4;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use phf::phf_map;
use rand::prelude::*;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlinkingLEDMaterial>::default())
            .add_plugins(MaterialPlugin::<TextMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[repr(C, align(16))]
pub struct BlinkingLEDMaterial {
    #[uniform(0)]
    pub sequence: [Vec4; 4],
}

impl BlinkingLEDMaterial {
    pub fn new() -> Self {
        let mut rnd = rand::rng();
        let (min_light_interval, max_light_interval) = (50, 200);
        let (min_glass_interval, max_glass_interval) = (200, 600);
        let mut sequence = [Vec4::ZERO; 4];
        for i in 0..4 {
            sequence[i] = Vec4::new(
                rnd.random_range(min_light_interval..max_light_interval) as f32,
                rnd.random_range(min_glass_interval..max_glass_interval) as f32,
                rnd.random_range(min_light_interval..max_light_interval) as f32,
                rnd.random_range(min_glass_interval..max_glass_interval) as f32,
            );
        }
        Self { sequence }
    }
}

impl Material for BlinkingLEDMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blinking_led.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

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
            color: vec4(1.0, 1.0, 1.0, 1.0),
            background_color: vec4(0.0, 0.0, 0.0, 0.2),
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

    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn emission(mut self, intensity: f32) -> Self {
        let rgb = self.color.xyz() * intensity;
        self.color.x = rgb.x;
        self.color.y = rgb.y;
        self.color.z = rgb.z;
        self
    }

    pub fn background_color(mut self, background_color: Vec4) -> Self {
        self.background_color = background_color;
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

#[derive(Debug, Clone, ShaderType)]
pub struct Text<const N: usize> {
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

impl<const N: usize> Text<N> {
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
