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
use bevy_spts_changeset::prelude::{ChangeSet, ChangesetResource};

#[allow(unused_imports)]
pub use api::{UndoRedoApi, UndoRedoResult};
use bevy_spts_vectorgraphic::prelude::*;

use crate::plugins::{selected::Selected, inspecting::Inspected};

pub struct UndoRedoPlugin;

#[derive(Default)]
struct UndoRedoTag;

impl Plugin for UndoRedoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let filter = SceneFilter::default()
            .allow::<Transform>()
            .allow::<GlobalTransform>()
            .allow::<Name>()
            .allow::<Handle<ColorMaterial>>()
            .allow::<Handle<Mesh>>()
            .allow::<Visibility>()
            .allow::<ViewVisibility>()
            .allow::<InheritedVisibility>()
            // Vector graphic
            .allow::<VectorGraphic>()
            .allow::<VectorGraphicPathStorage>()
            .allow::<Endpoint>()
            .allow::<Edge>()
            .allow::<EdgeVariant>()
            .allow::<StrokeOptions>()
            .allow::<FillOptions>()
            // State tags
            .allow::<Selected>()
            .allow::<Inspected>();

        app.register_type::<Transform>();
        app.register_type::<GlobalTransform>();
        app.register_type::<Name>();
        app.register_type::<Handle<ColorMaterial>>();
        app.register_type::<Handle<Mesh>>();
        app.register_type::<Visibility>();
        app.register_type::<ViewVisibility>();
        app.register_type::<InheritedVisibility>();
        // Vector graphics
        app.register_type::<VectorGraphic>();
        app.register_type::<VectorGraphicPathStorage>();
        app.register_type::<Endpoint>();
        app.register_type::<Edge>();
        app.register_type::<EdgeVariant>();
        app.register_type::<StrokeOptions>();
        app.register_type::<FillOptions>();
        // State tags
        app.register_type::<Selected>();
        app.register_type::<Inspected>();

        let changeset_res = ChangesetResource::<UndoRedoTag>::new().with_filter(filter);
        app.insert_resource(changeset_res);
        app.insert_resource(UndoRedoResource::default());
    }
}

#[derive(Resource, Default)]
pub struct UndoRedoResource {
    undo_stack: Vec<ChangeSet>,
    redo_stack: Vec<ChangeSet>,
}
