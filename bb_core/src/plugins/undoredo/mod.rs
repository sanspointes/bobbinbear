mod api;

use bevy::{
    app::Plugin,
    asset::Handle,
    core::Name,
    ecs::system::Resource,
    render::{
        mesh::Mesh,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    scene::SceneFilter,
    sprite::ColorMaterial,
    transform::components::{GlobalTransform, Transform},
};
use bevy_mod_raycast::deferred::RaycastMesh;
use bevy_spts_changeset::prelude::{Changeset, ChangesetResource};

#[allow(unused_imports)]
pub use api::{UndoRedoApi, UndoRedoResult};
use bevy_spts_vectorgraphic::prelude::*;

use crate::{
    plugins::{inspecting::Inspected, selected::Selected},
    tools::{PenToolBuildingFromEndpointTag, PenToolBuildingVectorObjectTag},
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

use super::selected::Selectable;

pub struct UndoRedoPlugin;

#[derive(Default)]
pub struct UndoRedoTag;

impl Plugin for UndoRedoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        SceneFilter::deny_all();
        let filter = SceneFilter::default()
            .allow::<Transform>()
            .allow::<GlobalTransform>()
            .allow::<Name>()
            .allow::<Handle<ColorMaterial>>()
            .allow::<Handle<VectorGraphicMaterial>>()
            .allow::<Handle<Mesh>>()
            .allow::<Visibility>()
            .allow::<ViewVisibility>()
            .allow::<InheritedVisibility>()
            // Vector graphic
            .allow::<VectorGraphic>()
            .allow::<VectorGraphicPathStorage>()
            .allow::<VectorEdgeVM>()
            .allow::<VectorEndpointVM>()
            .allow::<RaycastMesh<Selectable>>()
            .allow::<Endpoint>()
            .allow::<Edge>()
            .allow::<EdgeVariant>()
            .allow::<StrokeOptions>()
            .allow::<StrokeColor>()
            .allow::<FillOptions>()
            .allow::<FillColor>()
            // State tags
            .allow::<Selected>()
            .allow::<Selectable>()
            .allow::<Inspected>()
            // PenTool
            .allow::<PenToolBuildingVectorObjectTag>()
            .allow::<PenToolBuildingFromEndpointTag>();

        app.register_type::<Transform>();
        app.register_type::<GlobalTransform>();
        app.register_type::<Name>();
        app.register_type::<Handle<ColorMaterial>>();
        app.register_type::<Handle<VectorGraphicMaterial>>();
        app.register_type::<Handle<Mesh>>();
        app.register_type::<Visibility>();
        app.register_type::<ViewVisibility>();
        app.register_type::<InheritedVisibility>();
        // Vector graphics
        app.register_type::<VectorEdgeVM>();
        app.register_type::<VectorEndpointVM>();
        app.register_type::<VectorGraphic>();
        app.register_type::<VectorGraphicPathStorage>();
        app.register_type::<Endpoint>();
        app.register_type::<Edge>();
        app.register_type::<EdgeVariant>();
        app.register_type::<StrokeOptions>();
        app.register_type::<StrokeColor>();
        app.register_type::<FillOptions>();
        app.register_type::<FillColor>();
        // State tags
        app.register_type::<Selected>();
        app.register_type::<Inspected>();
        // PenTool
        app.register_type::<PenToolBuildingVectorObjectTag>();
        app.register_type::<PenToolBuildingFromEndpointTag>();

        let changeset_res = ChangesetResource::<UndoRedoTag>::new().with_filter(filter);
        app.insert_resource(changeset_res);
        app.insert_resource(UndoRedoResource::default());
    }
}

#[derive(Resource, Default)]
pub struct UndoRedoResource {
    last_execute_seconds: f64,
    undo_stack: Vec<Changeset>,
    redo_stack: Vec<Changeset>,
}
