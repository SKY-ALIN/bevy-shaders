use bevy::prelude::*;

pub(crate) const MY_BLUE_COLOR: Color = Color::linear_rgba(0.08, 0.37, 0.74, 1.0);
pub(crate) const MY_BLUE_GLASS_COLOR: Color = Color::linear_rgba(0.08, 0.37, 0.74, 0.4);

pub(crate) fn bevy_color_into_vec4(color: Color) -> Vec4 {
    let c = color.to_linear();
    Vec4::new(c.red, c.green, c.blue, c.alpha)
}
