use crate::color::{bevy_color_into_vec4, MY_BLUE_COLOR, MY_BLUE_GLASS_COLOR};
use bevy::prelude::*;
use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use rand::prelude::*;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[repr(C)]
pub struct BlinkingLEDMaterial {
    #[uniform(0)]
    sequence: [Vec4; 4],
    #[uniform(1)]
    active_color: Vec4,
    #[uniform(2)]
    passive_color: Vec4,
}

impl BlinkingLEDMaterial {
    pub fn new(active_color: Color, passive_color: Color) -> Self {
        Self {
            sequence: [Vec4::ZERO; 4],
            active_color: bevy_color_into_vec4(active_color),
            passive_color: bevy_color_into_vec4(passive_color),
        }
    }

    pub fn with_random_sequence(mut self) -> Self {
        let mut rnd = rand::rng();
        let (min_light_interval, max_light_interval) = (50, 200);
        let (min_glass_interval, max_glass_interval) = (200, 600);
        for i in 0..4 {
            self.sequence[i] = Vec4::new(
                rnd.random_range(min_light_interval..max_light_interval) as f32,
                rnd.random_range(min_glass_interval..max_glass_interval) as f32,
                rnd.random_range(min_light_interval..max_light_interval) as f32,
                rnd.random_range(min_glass_interval..max_glass_interval) as f32,
            );
        }
        self
    }

    pub fn with_sequence(mut self, sequence: [f32; 16]) -> Self {
        self.sequence = [
            Vec4::new(sequence[0], sequence[1], sequence[2], sequence[3]),
            Vec4::new(sequence[4], sequence[5], sequence[6], sequence[7]),
            Vec4::new(sequence[8], sequence[9], sequence[10], sequence[11]),
            Vec4::new(sequence[12], sequence[13], sequence[14], sequence[15]),
        ];
        self
    }

    pub fn with_active_emission(mut self, intensity: f32) -> Self {
        let rgb = self.active_color.xyz() * intensity;
        self.active_color.x = rgb.x;
        self.active_color.y = rgb.y;
        self.active_color.z = rgb.z;
        self
    }

    pub fn with_passive_emission(mut self, intensity: f32) -> Self {
        let rgb = self.passive_color.xyz() * intensity;
        self.passive_color.x = rgb.x;
        self.passive_color.y = rgb.y;
        self.passive_color.z = rgb.z;
        self
    }
}

impl Default for BlinkingLEDMaterial {
    fn default() -> Self {
        Self::new(MY_BLUE_COLOR, MY_BLUE_GLASS_COLOR)
            .with_random_sequence()
            .with_active_emission(3.0)
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
