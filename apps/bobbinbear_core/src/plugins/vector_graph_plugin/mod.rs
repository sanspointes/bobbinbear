mod systems;
mod types;

pub use self::systems::*;
pub use self::types::{Fill, Stroke, VectorGraph};
use self::types::{FillTessellator, StrokeTessellator};

use bevy::prelude::*;

pub struct VectorGraphPlugin;
impl Plugin for VectorGraphPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = lyon_tessellation::FillTessellator::new();
        let stroke_tess = lyon_tessellation::StrokeTessellator::new();
        app.insert_resource(FillTessellator(fill_tess))
            .insert_resource(StrokeTessellator(stroke_tess));

        app.add_systems(
            PostUpdate,
            sys_mesh_vector_graph.after(bevy::transform::TransformSystem::TransformPropagate),
        );
        // app.add_systems(PostStartup, sys_setup_selection_bounds)
        //     .add_systems(PostUpdate, sys_selection_bounds_handle_change.after(sys_update_global_bounds_2d).in_set(EditorSet::PostPlugins));
        //
        // app.add_plugins(DefaultRaycastingPlugin::<Selectable>::default())
        //     .add_systems(PostStartup, sys_setup_selection_raycast)
        //     .add_systems(
        //         First,
        //         sys_selection_raycast_update_ray.before(RaycastSystem::BuildRays::<Selectable>),
        //     );
        //
        // app.register_type::<Selected>()
        //     .register_type::<Selectable>();
    }
}

/// [`SystemSet`] for the system that builds the meshes for newly-added
/// or changed shapes. Resides in [`PostUpdate`] schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct BuildShapes;
