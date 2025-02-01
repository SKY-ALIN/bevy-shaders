use crate::color::{bevy_color_into_vec4, MY_BLUE_COLOR};
use bevy::prelude::*;
use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

/// Static led material.
/// 
/// It lets you add emission to material or make it like transparent like glass.
/// 
/// # Examples:
/// 
/// For the effect from the examples:
/// 
/// ```rust
/// LEDMaterial::default();
/// ```
/// 
/// You can specify your own color and emission intensity of cource:
/// 
/// ```rust
/// LEDMaterial::new(Color::WHITE).with_emission(3.0);
/// ```
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct LEDMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl LEDMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color: bevy_color_into_vec4(color),
        }
    }

    pub fn with_emission(mut self, intensity: f32) -> Self {
        let rgb = self.color.xyz() * intensity;
        self.color.x = rgb.x;
        self.color.y = rgb.y;
        self.color.z = rgb.z;
        self
    }
}

impl Default for LEDMaterial {
    fn default() -> Self {
        Self::new(MY_BLUE_COLOR).with_emission(3.0)
    }
}

impl Material for LEDMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/led.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
