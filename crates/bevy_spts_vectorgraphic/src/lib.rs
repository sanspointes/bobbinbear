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
pub mod commands_ext;
pub mod components;
pub mod lyon_components;
pub mod material;
pub mod systems;
mod utils;

#[cfg(feature = "changeset")]
mod changeset;

pub mod prelude {
    pub use super::{VectorGraphicPlugin, VectorGraphicSet};
    #[cfg(feature = "changeset")]
    pub use crate::changeset::*;
    pub use crate::commands_ext;
    pub use crate::components::*;
    pub use crate::lyon_components::*;
    pub use crate::material::*;
    pub use crate::systems::*;
}

// Re-export lyon
pub mod lyon_tessellation {
    pub use lyon_tessellation::*;
}
pub mod lyon_path {
    pub use lyon_path::*;
}

use bevy::{
    app::{App, Plugin, PostUpdate},
    asset::{load_internal_asset, Handle},
    ecs::{schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet}, system::Resource},
    prelude::{Deref, DerefMut},
    render::render_resource::Shader,
    sprite::Material2dPlugin,
    transform::TransformSystem,
};
use systems::{
    sys_add_spawned_edges_to_vector_graphic, sys_add_spawned_endpoints_to_vector_graphic,
    sys_check_vector_graphic_children_changed, sys_collect_vector_graph_path_endpoints,
    sys_remesh_vector_graphic, sys_remove_despawned_edges_from_vector_graphic,
    sys_remove_despawned_endpoints_from_vector_graphic,
};

use crate::material::{sys_sync_vector_graphic_material, VectorGraphicMaterial};

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(3296418235224473622063937256920);

#[derive(SystemSet, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum VectorGraphicSet {
    // Updates the parent VectorGraphic component's internals
    DetectChanges,
    UpdatePath,
    Remesh,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SptsFillTessellator(lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut)]
pub struct SptsStrokeTessellator(lyon_tessellation::StrokeTessellator);

pub struct VectorGraphicPlugin;
impl Plugin for VectorGraphicPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = lyon_tessellation::FillTessellator::new();
        let stroke_tess = lyon_tessellation::StrokeTessellator::new();
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "./shaders/vector_graphic_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(Material2dPlugin::<VectorGraphicMaterial>::default());

        app.insert_resource(SptsFillTessellator(fill_tess))
            .insert_resource(SptsStrokeTessellator(stroke_tess));

        app.configure_sets(
            PostUpdate,
            (
                VectorGraphicSet::DetectChanges,
                VectorGraphicSet::UpdatePath,
                VectorGraphicSet::Remesh,
            )
                .chain()
                .before(TransformSystem::TransformPropagate),
        );

        app.add_systems(
            PostUpdate,
            (
                sys_add_spawned_endpoints_to_vector_graphic,
                sys_add_spawned_edges_to_vector_graphic,
                sys_remove_despawned_endpoints_from_vector_graphic,
                sys_remove_despawned_edges_from_vector_graphic,
                sys_check_vector_graphic_children_changed,
            )
                .in_set(VectorGraphicSet::DetectChanges),
        );

        app.add_systems(
            PostUpdate,
            sys_collect_vector_graph_path_endpoints.in_set(VectorGraphicSet::UpdatePath),
        );

        app.add_systems(
            PostUpdate,
            sys_remesh_vector_graphic.in_set(VectorGraphicSet::Remesh),
        );

        app.add_systems(PostUpdate, sys_sync_vector_graphic_material);
    }
}
