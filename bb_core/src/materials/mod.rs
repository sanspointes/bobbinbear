use bevy::prelude::*;

mod ui_el_material;

pub use ui_el_material::*;

/// Plugin that supports all the custom materials used by bobbin bear.
/// Mainly just adds systems that sync the materials with the component state.
pub struct BobbinMaterialsPlugin;

impl Plugin for BobbinMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiElementMaterialPlugin);
    }
}
