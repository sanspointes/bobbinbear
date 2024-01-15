use bb_vector_network::bb_graph::BBGraph;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_prototype_lyon::{prelude::tess::StrokeTessellator, draw::Stroke};

use super::{FillTessellator, Fill, VectorGraph};

#[allow(clippy::type_complexity)]
pub fn sys_mesh_vector_graph(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (Option<&Fill>, Option<&Stroke>, &VectorGraph, &mut Mesh2dHandle),
        Or<(Changed<VectorGraph>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (maybe_fill_mode, maybe_stroke_mode, path, mut mesh) in &mut query {
        let mut buffers = VertexBuffers::new();

        if let Some(fill_mode) = maybe_fill_mode {
            fill(&mut fill_tess, &path.0, fill_mode, &mut buffers);
        }

        if let Some(stroke_mode) = maybe_stroke_mode {
            stroke(&mut stroke_tess, &path.0, stroke_mode, &mut buffers);
        }

        if (maybe_fill_mode, maybe_stroke_mode) == (None, None) {
            fill(
                &mut fill_tess,
                &path.0,
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
    path: &BBGraph,
    mode: &Fill,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { color: mode.color }),
    ) {
        error!("FillTessellator error: {:?}", e);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &StrokeOptions
fn stroke(
    tess: &mut ResMut<StrokeTessellator>,
    path: &tess::path::Path,
    mode: &Stroke,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
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
