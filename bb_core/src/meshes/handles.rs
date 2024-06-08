use bevy::{
    math::{primitives::{ Circle, Rectangle }, Vec3}, render::mesh::{Mesh, VertexAttributeValues}
};

use crate::materials::{ATTRIBUTE_THEME_BASE, ATTRIBUTE_THEME_BASE_OPACITY, ATTRIBUTE_THEME_MIX};

pub fn build_mesh_endpoint_handle() -> Mesh {
    let mut mesh = Mesh::from(Circle::new(5.));
    mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![0.; mesh.count_vertices()]);
    mesh.insert_attribute(ATTRIBUTE_THEME_BASE, vec![1.; mesh.count_vertices()]);
    mesh.insert_attribute(ATTRIBUTE_THEME_BASE_OPACITY, vec![1.; mesh.count_vertices()]);

    let mut inner_mesh = Mesh::from(Circle::new(3.5));
    inner_mesh.translate_by(Vec3::new(0., 0., 1.));
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![1.; inner_mesh.count_vertices()]);
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_BASE, vec![1.; inner_mesh.count_vertices()]);
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_BASE_OPACITY, vec![1.; inner_mesh.count_vertices()]);

    mesh.merge(inner_mesh);

    mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);

    // Update normals to point outward of mesh
    let position_attr = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(vert_positions) = position_attr else {
        panic!("Impossible.");
    };
    let vert_normals: Vec<_> = vert_positions.iter().map(|v| {
        Vec3::from_array(*v).normalize().to_array()
    }).collect();
    let normals_attr = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL.id).unwrap();
    *normals_attr = VertexAttributeValues::Float32x3(vert_normals);

    mesh
}

pub fn build_mesh_control_handle() -> Mesh {
    let mut mesh = Mesh::from(Rectangle::new(5., 5.));
    mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![0.; mesh.count_vertices()]);
    mesh.insert_attribute(ATTRIBUTE_THEME_BASE, vec![1.; mesh.count_vertices()]);
    mesh.insert_attribute(ATTRIBUTE_THEME_BASE_OPACITY, vec![1.; mesh.count_vertices()]);

    let mut inner_mesh = Mesh::from(Rectangle::new(3.5, 3.5));
    inner_mesh.translate_by(Vec3::new(0., 0., 1.));
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![1.; inner_mesh.count_vertices()]);
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_BASE, vec![1.; inner_mesh.count_vertices()]);
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_BASE_OPACITY, vec![1.; inner_mesh.count_vertices()]);

    mesh.merge(inner_mesh);

    mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);

    // Update normals to point outward of mesh
    let position_attr = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(vert_positions) = position_attr else {
        panic!("Impossible.");
    };
    let vert_normals: Vec<_> = vert_positions.iter().map(|v| {
        Vec3::from_array(*v).normalize().to_array()
    }).collect();
    let normals_attr = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL.id).unwrap();
    *normals_attr = VertexAttributeValues::Float32x3(vert_normals);

    mesh
}
