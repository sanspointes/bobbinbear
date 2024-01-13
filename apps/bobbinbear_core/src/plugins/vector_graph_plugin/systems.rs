use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use lyon_tessellation::BuffersBuilder;

use super::types::{
    Fill, FillTessellator, Stroke, StrokeTessellator, VectorGraph, VertexBuffers, VertexConstructor,
};

#[allow(clippy::type_complexity)]
pub fn sys_mesh_vector_graph(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (
            Option<&Fill>,
            Option<&Stroke>,
            &mut VectorGraph,
            &mut Mesh2dHandle,
        ),
        Or<(Changed<VectorGraph>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (maybe_fill_mode, maybe_stroke_mode, mut vector_graph, mut mesh) in &mut query {
        let mut buffers = VertexBuffers::new();

        if let Some(fill_mode) = maybe_fill_mode {
            match vector_graph.0.update_regions() {
                Ok(_) => {},
                Err(reason) => println!("sys_mesh_vector_graph: Error updating BBGraph.\nReason: {reason:?}"),
            };
            fill(&mut fill_tess, &vector_graph, fill_mode, &mut buffers);
        }

        if let Some(stroke_mode) = maybe_stroke_mode {
            stroke(&mut stroke_tess, &vector_graph, stroke_mode, &mut buffers);
        }

        if (maybe_fill_mode, maybe_stroke_mode) == (None, None) {
            fill(
                &mut fill_tess,
                &vector_graph,
                &Fill::color(Color::FUCHSIA),
                &mut buffers,
            );
        }

        mesh.0 = meshes.add(build_mesh(&buffers));
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &FillOptions
fn fill(
    tess: &mut ResMut<FillTessellator>,
    vector_graph: &VectorGraph,
    mode: &Fill,
    buffers: &mut VertexBuffers,
) {
    let path = match vector_graph.0.generate_fill_path() {
        Ok(path) => path,
        Err(reason) => {
            println!("Tesselation Error: {reason:?}.");
            return;
        }
    };
    if let Err(e) = tess.tessellate_path(
        path.as_slice(),
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { color: mode.color }),
    ) {
        error!("FillTessellator error: {:?}", e);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &StrokeOptions
fn stroke(
    tess: &mut ResMut<StrokeTessellator>,
    vector_graph: &VectorGraph,
    mode: &Stroke,
    buffers: &mut VertexBuffers,
) {
    let path = match vector_graph.0.generate_stroke_path() {
        Ok(path) => path,
        Err(reason) => {
            println!("Tesselation Error: {reason:?}.");
            return;
        }
    };
    if let Err(e) = tess.tessellate_path(
        path.as_slice(),
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { color: mode.color }),
    ) {
        error!("StrokeTessellator error: {:?}", e);
    }
}

fn build_mesh(buffers: &VertexBuffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| [v.position[0], v.position[1], 0.0])
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        buffers
            .vertices
            .iter()
            .map(|v| v.color)
            .collect::<Vec<[f32; 4]>>(),
    );

    mesh
}
