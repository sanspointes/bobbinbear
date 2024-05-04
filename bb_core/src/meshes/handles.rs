use bevy::{
    math::{primitives::Circle, Vec3}, render::mesh::{Mesh, VertexAttributeValues}
};

use crate::materials::ATTRIBUTE_THEME_MIX;

pub fn build_mesh_endpoint_handle() -> Mesh {
    let mut mesh = Mesh::from(Circle::new(5.));
    mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![0.; mesh.count_vertices()]);
    // mesh.attributes_mut().find(|attr| attr.0 === MeshVertexAttributeId)

    let mut inner_mesh = Mesh::from(Circle::new(3.5));
    inner_mesh.translate_by(Vec3::new(0., 0., 1.));
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![1.; inner_mesh.count_vertices()]);

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
