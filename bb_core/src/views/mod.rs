use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
};

use crate::{plugins::model_view::RegisterView, tools::ToolSet, views::vector_edge::VectorEdgeVM, PosSet};

use self::{
    vector_edge::sys_update_vector_edge_vm_mesh_when_endpoint_move,
    vector_endpoint::VectorEndpointVM,
};

#[derive(SystemSet, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ViewSpawnSet;

#[derive(SystemSet, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ViewDespawnSet;

pub mod vector_edge;
pub mod vector_endpoint;

pub struct BobbinViewsPlugin;

impl Plugin for BobbinViewsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(Update, ViewSpawnSet.after(ToolSet).before(PosSet::PositionObjects));
        app.configure_sets(Update, ViewDespawnSet.after(ToolSet).before(PosSet::PositionObjects));

        app.register_type::<VectorEndpointVM>();
        app.register_viewable_with_set::<VectorEndpointVM>(
            Update,
            ViewSpawnSet,
            Update,
            ViewDespawnSet,
        );

        app.register_type::<VectorEdgeVM>()
            .register_viewable_with_set::<VectorEdgeVM>(
                Update,
                ViewSpawnSet,
                Update,
                ViewDespawnSet,
            )
            .add_systems(
                Update,
                sys_update_vector_edge_vm_mesh_when_endpoint_move
                    .before(PosSet::Propagate)
                    .after(PosSet::PositionObjects),
            );
    }
}
