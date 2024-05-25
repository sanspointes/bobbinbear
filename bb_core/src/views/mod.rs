use bevy::app::{Plugin, PostUpdate};

use crate::{plugins::model_view::RegisterView, views::vector_edge::VectorEdgeVM};

use self::{vector_edge::sys_update_edge_when_endpoint_move, vector_endpoint::VectorEndpointVM};

pub mod vector_edge;
pub mod vector_endpoint;

pub struct BobbinViewsPlugin;

impl Plugin for BobbinViewsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<VectorEndpointVM>();
        app.register_viewable::<VectorEndpointVM>(PostUpdate, PostUpdate);

        app.register_type::<VectorEdgeVM>()
            .register_viewable::<VectorEdgeVM>(PostUpdate, PostUpdate)
            .add_systems(PostUpdate, sys_update_edge_when_endpoint_move);
    }
}
