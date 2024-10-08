use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2d,
};

pub(super) fn selection_bounds_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Top Left Square
            [0., 1., 0.],
            [0., 0., 0.],
            [1., 1., 0.],
            [1., 0., 0.],
        ],
    );

    mesh
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct SelectionBoundsMaterial {
    #[uniform(0)]
    pub color: LinearRgba,

    #[uniform(1)]
    pub border_color: LinearRgba,

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
