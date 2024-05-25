//! Module for selected component and selection box components.

mod api;
mod material;
pub mod raycast;

use bevy::{ecs::reflect::ReflectComponent, prelude::*, sprite::Material2dPlugin};
use bevy_mod_raycast::deferred::{DeferredRaycastingPlugin, RaycastSystem};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub use api::SelectedApi;
use crate::ecs::{sys_update_proxied_component, ProxiedComponent};

use self::{material::SelectionBoundsMaterial, raycast::{sys_selection_raycast_update_helper, sys_selection_raycast_update_ray, sys_setup_selection_raycast, SelectableHits}};

#[derive(Component, Reflect, Default, Tsify, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// Component defining whether or not object is selected.
pub enum Selected {
    #[default]
    Deselected,
    Selected,
}

pub type ProxiedSelected = ProxiedComponent<Selected>;

impl Selected {
    pub fn is_selected(&self) -> bool {
        match self {
            Self::Deselected => false,
            Self::Selected => true,
        }
    }
}

#[derive(Component, Reflect, Default, Tsify, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// Component defining whether or not component is selectable.
pub enum Selectable {
    #[default]
    Default,
    Locked,
}
impl From<bool> for Selectable {
    fn from(value: bool) -> Self {
        if value {
            Selectable::Default
        } else {
            Selectable::Locked
        }
    }
}
impl From<Selectable> for bool {
    fn from(value: Selectable) -> Self {
        match value {
            Selectable::Default => true,
            Selectable::Locked => false,
        }
    }
}

pub type ProxiedSelectable = ProxiedComponent<Selectable>;

#[derive(Component, Reflect, Default, Tsify, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// Component defining whether or not object is selected.
pub enum Hovered {
    #[default]
    Unhovered,
    Hovered,
}

pub type ProxiedHovered = ProxiedComponent<Hovered>;

impl Hovered {
    pub fn is_hovered(&self) -> bool {
        match self {
            Self::Unhovered => false,
            Self::Hovered => true,
        }
    }
}

pub struct SelectedPlugin;
impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SelectionBoundsMaterial>::default());
        // app.add_systems(PostStartup, sys_setup_selection_bounds)
        //     .add_systems(PostUpdate, sys_selection_bounds_handle_change.after(sys_update_global_bounds_2d).in_set(EditorSet::PostPlugins));
        //
        // app.add_plugins(DeferredRaycastingPlugin::<Selectable>::default())
        //     .add_systems(PostStartup, sys_setup_selection_raycast)
        //     .add_systems(
        //         First,
        //         sys_selection_raycast_update_ray.before(RaycastSystem::BuildRays::<Selectable>),
        //     );
        app
            .register_type::<Selected>()
            .register_type::<ProxiedSelected>()
            .add_systems(PostUpdate, sys_update_proxied_component::<Selected>)

            .register_type::<Hovered>()
            .register_type::<ProxiedHovered>()
            .add_systems(PostUpdate, sys_update_proxied_component::<Hovered>)

            .register_type::<Selectable>()
            .register_type::<ProxiedSelectable>()
            .add_systems(PostUpdate, sys_update_proxied_component::<Selectable>)
            // Setup raycasting the Selectable component
            .insert_resource(SelectableHits::default())
            .add_plugins(DeferredRaycastingPlugin::<Selectable>::default())
            .add_systems(PostStartup, sys_setup_selection_raycast)
            .add_systems(
                First,
                sys_selection_raycast_update_ray.before(RaycastSystem::BuildRays::<Selectable>),
            )
            .add_systems(
                First,
                sys_selection_raycast_update_helper.after(RaycastSystem::UpdateIntersections::<Selectable>),
            )
        ;
    }
}
