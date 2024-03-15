mod api;

use bevy::{
    app::Plugin,
    asset::Handle,
    core::Name,
    ecs::system::Resource,
    reflect::TypeRegistry,
    render::{
        mesh::Mesh,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::ColorMaterial,
    transform::components::{GlobalTransform, Transform},
};
use bevy_spts_changeset::{changes::ChangeSet, resource::ChangesetResource};

#[allow(unused_imports)]
pub use api::{UndoRedoApi, UndoRedoResult};
use bevy_spts_fragments::prelude::Uid;
use bevy_spts_vectorgraphic::prelude::*;

use crate::selected::Selected;

pub struct UndoRedoPlugin;

#[derive(Default)]
struct UndoRedoTag;

impl Plugin for UndoRedoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut type_registry = TypeRegistry::new();
        type_registry.register::<Transform>();
        type_registry.register::<GlobalTransform>();
        type_registry.register::<Name>();
        type_registry.register::<Handle<ColorMaterial>>();
        type_registry.register::<Handle<Mesh>>();
        type_registry.register::<Visibility>();
        type_registry.register::<ViewVisibility>();
        type_registry.register::<InheritedVisibility>();
        // Vector graphics
        type_registry.register::<VectorGraphic>();
        type_registry.register::<VectorGraphicPathStorage>();
        type_registry.register::<Endpoint>();
        type_registry.register::<Edge>();
        type_registry.register::<EdgeVariant>();
        type_registry.register::<StrokeOptions>();
        type_registry.register::<FillOptions>();
        // State tags
        type_registry.register::<Selected>();

        app.register_type::<Uid>();
        app.register_type::<VectorGraphic>();
        app.register_type::<VectorGraphicPathStorage>();
        app.register_type::<Endpoint>();
        app.register_type::<Edge>();
        app.register_type::<StrokeOptions>();
        app.register_type::<FillOptions>();

        let changeset_res = ChangesetResource::<UndoRedoTag>::new(type_registry);
        app.insert_resource(changeset_res);
        app.insert_resource(UndoRedoResource::default());
    }
}

#[derive(Resource, Default)]
pub struct UndoRedoResource {
    undo_stack: Vec<ChangeSet>,
    redo_stack: Vec<ChangeSet>,
}
