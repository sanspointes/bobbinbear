use bevy::{
    math::{Vec3, Vec3A},
    render::mesh::{Mesh, MeshVertexAttributeId, VertexAttributeValues},
    utils::thiserror::Error,
};
use bevy_mod_raycast::primitives::IntersectionData;

#[derive(Debug, Error)]
pub enum TriangleIntersectionAttributeDataError {
    #[error("Intersection data does not have a triangle index.")]
    NoTriangleIndex,
    #[error("Mesh does have the desired attribute {0:?}.")]
    NoMeshAttribute(MeshVertexAttributeId),
}

#[derive(Debug)]
pub enum TriangleIntersectionAttributeData {
    Float32(f32),
    Float32x3(Vec3),
}

pub fn get_intersection_triangle_attribute_data(
    mesh: &Mesh,
    intersection: &IntersectionData,
    attr_id: MeshVertexAttributeId,
) -> Result<TriangleIntersectionAttributeData, TriangleIntersectionAttributeDataError> {
    let triangle_indices = intersection
        .triangle_indices()
        .ok_or(TriangleIntersectionAttributeDataError::NoTriangleIndex)?;
    let barycentric_coord = intersection.barycentric_coord();
    // Not sure why the provided barycentric_coord is not correct for me.
    // This maps it to something that works.
    let barycentric_coord = Vec3::new(
        barycentric_coord.z,
        barycentric_coord.x,
        barycentric_coord.y,
    );

    let attr_data =
        mesh.attribute(attr_id)
            .ok_or(TriangleIntersectionAttributeDataError::NoMeshAttribute(
                attr_id,
            ))?;

    let value = match attr_data {
        VertexAttributeValues::Float32(data) => {
            let v1 = data[triangle_indices[0]];
            let v2 = data[triangle_indices[1]];
            let v3 = data[triangle_indices[2]];
            TriangleIntersectionAttributeData::Float32(
                v1 * barycentric_coord.x + v2 * barycentric_coord.y + v3 * barycentric_coord.z,
            )
        }
        VertexAttributeValues::Float32x3(data) => {
            let v1: Vec3A = data[triangle_indices[0]].into();
            let v2: Vec3A = data[triangle_indices[1]].into();
            let v3: Vec3A = data[triangle_indices[2]].into();
            TriangleIntersectionAttributeData::Float32x3(
                (v1 * barycentric_coord.x + v2 * barycentric_coord.y + v3 * barycentric_coord.z)
                    .into(),
            )
        }
        remaining => todo!(
            "Implement get_intersection_attribute_data for {}.",
            remaining.enum_variant_name()
        ),
    };
    Ok(value)
}
