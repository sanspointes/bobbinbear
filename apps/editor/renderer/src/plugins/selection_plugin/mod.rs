mod raycast;
mod selection_bounds;

use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastMesh, RaycastSystem};

use crate::editor::EditorSet;

use self::{
    raycast::{sys_selection_raycast_update_ray, sys_setup_selection_raycast},
    selection_bounds::{sys_handle_selection_change, sys_setup_selection_bounds},
};

use super::bounds_2d_plugin::sys_update_global_bounds_2d;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
/// Contains the state for whether this component is selected.
pub enum Selected {
    #[default]
    No,
    Yes,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
/// Contains the state of this entity relating to interaction.
/// Locked entities cannot be selected.
pub enum Selectable {
    #[default]
    Default,
    Locked,
}

#[derive(Bundle, Default)]
pub struct SelectableBundle {
    raycast: RaycastMesh<Selectable>,
    selectable: Selectable,
    selected: Selected,
}

/// This plugin generalises some selection related behaviour.
/// - Provides a `Selectable` component to mark something as selectable.
/// - Provides a `SelectedTag` component to mark something as selected.
/// - Shows a combined bounding rect of all selected components.
/// - Sets up and updates a RaycastSource and exposes a
pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, sys_setup_selection_bounds)
            .add_systems(Update, (sys_handle_selection_change.after(sys_update_global_bounds_2d)).in_set(EditorSet::PostMsgs));

        app.add_plugins(DefaultRaycastingPlugin::<Selectable>::default())
            .add_systems(PostStartup, sys_setup_selection_raycast)
            .add_systems(
                First,
                sys_selection_raycast_update_ray.before(RaycastSystem::BuildRays::<Selectable>),
            );

        app.register_type::<Selected>()
            .register_type::<Selectable>();
    }
}
