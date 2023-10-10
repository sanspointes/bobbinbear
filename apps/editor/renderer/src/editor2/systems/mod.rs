pub mod focus_rings;

use std::matches;

use bevy::{
    prelude::*,
    render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

use super::entities::{
    vector::{VecNodeTag, VectorObjectTag},
    Bounded,
};

/// Contains systems that run after the main message handler that perform specific tasks

/// Recalculates the bounds component for entities with Bounded::NeedsCalculate
///
/// * `meshes`:
/// * `q_changed_vector_objects`:
/// * `q_vector_objects`:
pub fn calculate_vector_object_bounds(
    meshes: Res<Assets<Mesh>>,
    mut q_changed_vector_objects: Query<
        (&mut Bounded, &GlobalTransform, &Mesh2dHandle, &Children),
        (With<VectorObjectTag>, Changed<Bounded>),
    >,
    q_all_nodes: Query<&Transform, With<VecNodeTag>>,
) {
    for (mut bounded, global_transform, handle, children) in q_changed_vector_objects.iter_mut() {
        if matches!(*bounded, Bounded::NeedsCalculate) {
            // Only calculate off mesh if it has some verticies
            let mesh = meshes.get(&handle.0).map(|mesh| {
                if mesh.count_vertices() > 2 {
                    return Some(mesh);
                }
                return None;
            }).flatten();

            if let Some(mesh) = mesh {
                let topology = mesh.primitive_topology();
                match topology {
                    PrimitiveTopology::PointList | PrimitiveTopology::TriangleList => {
                        let vertices: Vec<Vec3> = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                            None => panic!("Mesh does not contain vertex positions"),
                            Some(vertex_values) => match &vertex_values {
                                VertexAttributeValues::Float32x3(positions) => positions
                                    .iter()
                                    .map(|coordinates| {
                                        global_transform.transform_point(Vec3::from(*coordinates))
                                    })
                                    // .map(|coordinates| Vec3::from(*coordinates))
                                    .collect(),
                                _ => panic!("Unexpected vertex types in ATTRIBUTE_POSITION"),
                            },
                        };
                        let new_bounds = Bounded::from_vec3s(&vertices);

                        *bounded = new_bounds;
                    }
                    topology => {
                        unimplemented!(
                            "Calculate assets bound not implemented for topology {topology:?}"
                        );
                    }
                }
            } else {
                let nodes: Vec<_> = q_all_nodes
                    .iter_many(children)
                    .map(|x| global_transform.transform_point(x.translation))
                    .collect();
                if nodes.len() > 1 {
                    *bounded = Bounded::from_vec3s(&nodes);
                } else {
                    *bounded = Bounded::from_vec3s(&vec![global_transform.translation()]);
                }
            }
        }
    }
}
