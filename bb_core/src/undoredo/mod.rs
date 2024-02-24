mod api;

use bevy::{app::Plugin, asset::Handle, core::Name, ecs::system::Resource, reflect::TypeRegistry, render::{mesh::Mesh, view::{InheritedVisibility, ViewVisibility, Visibility}}, sprite::{ColorMaterial, Mesh2d}, transform::components::{GlobalTransform, Transform}};
use bevy_spts_changeset::{changes::ChangeSet, resource::ChangesetResource};

#[allow(unused_imports)]
pub use api::{UndoRedoApi, UndoRedoResult};

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

