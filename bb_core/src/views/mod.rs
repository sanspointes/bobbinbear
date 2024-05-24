use bevy::{
    app::{Plugin, PostUpdate},
    log::warn,
};

use crate::plugins::model_view::RegisterView;

use self::vector_endpoint::VectorEndpointVM;

pub mod vector_endpoint;

pub struct BobbinViewsPlugin;

impl Plugin for BobbinViewsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        warn!("BobbinViewsPlugin");
        app.register_type::<VectorEndpointVM>();
        app.register_viewable::<VectorEndpointVM>(PostUpdate, PostUpdate);
    }
}
