use bevy::{prelude::*, math::prelude::{Rectangle, Circle}, sprite::Mesh2dHandle};
use serde::{Serialize, Deserialize};

use super::core::{DerivedMesh, DerivedMaterial};

#[derive(Component, Serialize, Deserialize)]
pub enum Node {
    Control,
    Endpoint,
}

/// Adds meshes to DerivedMesh entities that have Node components.
///
/// * `meshes`: 
/// * `q_nodes`: 
#[allow(clippy::type_complexity)]
pub fn sys_derived_mesh_for_node(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    q_nodes: Query<(Entity, &Node), (With<DerivedMesh>, Without<Mesh2dHandle>)>,
) {
    for (e, node) in &q_nodes {
        let mesh = match node {
            Node::Control => meshes.add(Rectangle::new(5., 5.)),
            Node::Endpoint => meshes.add(Circle::new(5.)),
        };
        commands.entity(e).insert(mesh);
    }
}

/// Adds meshes to DerivedMesh entities that have Node components.
///
/// * `meshes`: 
/// * `q_nodes`: 
#[allow(clippy::type_complexity)]
pub fn sys_derived_material_for_node(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_nodes: Query<(Entity, &Node), (With<DerivedMaterial>, Without<Handle<ColorMaterial>>)>,
) {
    for (e, node) in &q_nodes {
        let mesh = match node {
            Node::Control => materials.add(ColorMaterial::from(Color::BLUE)),
            Node::Endpoint => materials.add(ColorMaterial::from(Color::BLUE)),
        };
        commands.entity(e).insert(mesh);
    }
}
