use bevy::{
    math::{primitives::Circle, Vec3},
    render::mesh::Mesh,
};

use crate::materials::ATTRIBUTE_THEME_MIX;

pub fn build_mesh_endpoint_handle() -> Mesh {
    let mut mesh = Mesh::from(Circle::new(5.));
    mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![0.; mesh.count_vertices()]);

    let mut inner_mesh = Mesh::from(Circle::new(3.5));
    inner_mesh.translate_by(Vec3::new(0., 0., 1.));
    inner_mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![1.; inner_mesh.count_vertices()]);

    mesh.merge(inner_mesh);

    mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
    mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
    mesh
}
