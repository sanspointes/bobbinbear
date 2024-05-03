use bevy::{app::{App, Plugin, PreStartup}, asset::Assets, ecs::system::{Res, ResMut, Resource, SystemParam}, prelude::Deref, render::mesh::Mesh, sprite::Mesh2dHandle};

use self::handles::build_mesh_endpoint_handle;

mod handles;


/// Plugin that supports all the custom materials used by bobbin bear.
/// Mainly just adds systems that sync the materials with the component state.
pub struct BobbinMeshesPlugin;

impl Plugin for BobbinMeshesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BobbinMeshesResource::default())
            .add_systems(PreStartup, sys_setup)
        ;
    }
}

#[derive(Resource, Default)]
pub struct BobbinMeshesResource {
    endpoint_mesh: Option<Mesh2dHandle>,
}

impl BobbinMeshesResource {
    pub fn endpoint_mesh(&self) -> Mesh2dHandle {
        self.endpoint_mesh.as_ref().unwrap().clone()
    }
}

fn sys_setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut res: ResMut<BobbinMeshesResource>,
) {
    let endpoint_mesh = build_mesh_endpoint_handle();
    let handle = Mesh2dHandle(meshes.add(endpoint_mesh));
    res.endpoint_mesh = Some(handle);
}

#[derive(SystemParam, Deref)]
pub struct BobbinMeshes<'w>(Res<'w, BobbinMeshesResource>);
