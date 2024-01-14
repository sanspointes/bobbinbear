use anyhow::anyhow;
use bevy::{
    math::{vec3, Vec4Swizzles},
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

pub fn translate_mesh(mesh: &Mesh, translate: Vec3) -> anyhow::Result<Mesh> {
    let attr_pos = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .ok_or(anyhow!("No position attribute to translate."))?;

    let attr_pos = attr_pos
        .as_float3()
        .ok_or(anyhow!("Can't get position attribute as float3."))?;

    let mut out_mesh = mesh.clone();

    let new_verts: Vec<_> = attr_pos
        .iter()
        .map(|vert| {
            vec3(
                vert[0] + translate.x,
                vert[1] + translate.y,
                vert[2] + translate.z,
            )
        })
        .collect();
    out_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_verts);

    Ok(out_mesh)
}

pub fn add_vertex_colors_mesh(mesh: &mut Mesh, color: Color) {
    let count = mesh.count_vertices();

    let color_data = vec![[color.r(), color.g(), color.b(), color.a()]; count];
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, color_data);
}

pub fn combine_meshes(
    meshes: &[Mesh],
    transforms: &[Transform],
    use_normals: bool,
    use_tangents: bool,
    use_uvs: bool,
    use_colors: bool,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut tangets: Vec<[f32; 4]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut indices_offset = 0;

    if meshes.len() != transforms.len() {
        panic!(
            "meshes.len({}) != transforms.len({})",
            meshes.len(),
            transforms.len()
        );
    }

    for (mesh, trans) in meshes.iter().zip(transforms) {
        if let Indices::U32(mesh_indices) = &mesh.indices().unwrap() {
            let mat = trans.compute_matrix();

            let positions_len;

            if let Some(VertexAttributeValues::Float32x3(vert_positions)) =
                &mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                positions_len = vert_positions.len();
                for p in vert_positions {
                    positions.push(mat.transform_point3(Vec3::from(*p)).into());
                }
            } else {
                panic!("no positions")
            }

            if use_uvs {
                if let Some(VertexAttributeValues::Float32x2(vert_uv)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_UV_0)
                {
                    for uv in vert_uv {
                        uvs.push(*uv);
                    }
                } else {
                    panic!("no uvs")
                }
            }

            if use_normals {
                // Comment below taken from mesh_normal_local_to_world() in mesh_functions.wgsl regarding
                // transform normals from local to world coordinates:

                // NOTE: The mikktspace method of normal mapping requires that the world normal is
                // re-normalized in the vertex shader to match the way mikktspace bakes vertex tangents
                // and normal maps so that the exact inverse process is applied when shading. Blender, Unity,
                // Unreal Engine, Godot, and more all use the mikktspace method. Do not change this code
                // unless you really know what you are doing.
                // http://www.mikktspace.com/

                let inverse_transpose_model = mat.inverse().transpose();
                let inverse_transpose_model = Mat3 {
                    x_axis: inverse_transpose_model.x_axis.xyz(),
                    y_axis: inverse_transpose_model.y_axis.xyz(),
                    z_axis: inverse_transpose_model.z_axis.xyz(),
                };

                if let Some(VertexAttributeValues::Float32x3(vert_normals)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
                {
                    for n in vert_normals {
                        normals.push(
                            inverse_transpose_model
                                .mul_vec3(Vec3::from(*n))
                                .normalize_or_zero()
                                .into(),
                        );
                    }
                } else {
                    panic!("no normals")
                }
            }

            if use_tangents {
                if let Some(VertexAttributeValues::Float32x4(vert_tangets)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_TANGENT)
                {
                    for t in vert_tangets {
                        tangets.push(*t);
                    }
                } else {
                    panic!("no tangets")
                }
            }

            if use_colors {
                if let Some(VertexAttributeValues::Float32x4(vert_colors)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_COLOR)
                {
                    for c in vert_colors {
                        colors.push(*c);
                    }
                } else {
                    panic!("no colors")
                }
            }

            for i in mesh_indices {
                indices.push(*i + indices_offset);
            }
            indices_offset += positions_len as u32;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    if use_normals {
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    }

    if use_tangents {
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangets);
    }

    if use_uvs {
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }

    if use_colors {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
