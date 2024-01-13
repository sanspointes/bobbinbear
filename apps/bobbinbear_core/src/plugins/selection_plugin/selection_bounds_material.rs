use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, VertexFormat}, sprite::{Material2d, Mesh2d},
};

pub(super) fn selection_bounds_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleStrip);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        // Top Left Square
        [0., 1., 0.],
        [0., 0., 0.],
        [1., 1., 0.],
        [1., 0., 0.],
    ]);

    mesh.into()
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct SelectionBoundsMaterial {
    #[uniform(0)]
    pub color: Color,

    #[uniform(1)]
    pub border_color: Color,

    #[uniform(2)]
    pub border_width: f32,

    #[uniform(3)]
    pub dimensions: Vec2,
}

impl Material2d for SelectionBoundsMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/selection_bounds_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/selection_bounds_material.wgsl".into()
    }
}
