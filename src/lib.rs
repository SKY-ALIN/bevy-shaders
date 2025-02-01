use bevy::prelude::*;

pub mod blinking_led;
pub(crate) mod color;
pub mod led;
pub mod prelude;
pub mod text;

pub use blinking_led::BlinkingLEDMaterial;
pub use led::LEDMaterial;
pub use text::{TextData, TextMaterial};

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlinkingLEDMaterial>::default())
            .add_plugins(MaterialPlugin::<LEDMaterial>::default())
            .add_plugins(MaterialPlugin::<TextMaterial>::default());
    }
}
