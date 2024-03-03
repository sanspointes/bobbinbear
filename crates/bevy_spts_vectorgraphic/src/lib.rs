//! # Bevy Spts Vector Graphic
//!
//! This is an attempt to create an ECS native way of modifying vector graphics using
//! bevy_prototype_lyon.  It expects a specific hierarchy where an entity has a `VectorGraphic`
//! component and contains children with the `Endpoint` and `Edge` components.
//!
//! ## Each frame flow
//!
//! 1. User makes changes to `Endpoint` / `Edge` entities in `Update` set.
//! 2. If `Endpoint` component changes (topography) rebuild struct of internal loops, parent
//!    VectorGraphic needs remesh
//! 3. If `Transform` on entity with `Endpoint` component changes, parent VectorGraphic needs remesh
//! 4. If `Edge` component changes, parent VectorGraphic needs remesh
//! 5. Remesh the parent VectorGraphic if necessary.
//!
use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use lifecycle::{
    sys_add_spawned_edges_to_vector_graphic, sys_add_spawned_endpoints_to_vector_graphic,
    sys_remove_despawned_edges_from_vector_graphic,
    sys_remove_despawned_endpoints_from_vector_graphic,
};

mod components;
mod lifecycle;

#[derive(SystemSet, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum VectorGraphicSet {
    // Updates the parent VectorGraphic component's internals
    UpdateParent,
    UpdateMesh,
}

pub struct VectorGraphicPlugin;
impl Plugin for VectorGraphicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                sys_add_spawned_edges_to_vector_graphic,
                sys_add_spawned_endpoints_to_vector_graphic,
                sys_remove_despawned_edges_from_vector_graphic,
                sys_remove_despawned_endpoints_from_vector_graphic,
            )
                .in_set(VectorGraphicSet::UpdateParent),
        );
    }
}


