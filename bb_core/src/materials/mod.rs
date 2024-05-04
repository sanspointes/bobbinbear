use bevy::{prelude::*, sprite::Material2dPlugin};

mod ui_el_material;

pub use ui_el_material::*;

/// Plugin that supports all the custom materials used by bobbin bear.
/// Mainly just adds systems that sync the materials with the component state.
pub struct BobbinMaterialsPlugin;

impl Plugin for BobbinMaterialsPlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<UiElementMaterial>();
        app.add_plugins(Material2dPlugin::<UiElementMaterial>::default())
            .add_systems(PostUpdate, sys_update_ui_element_materials);
    }
}
