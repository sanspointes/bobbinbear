use bevy::{
    asset::{Asset, Assets, Handle},
    ecs::{
        component::Component,
        query::{Changed, Or},
        reflect::ReflectComponent,
        system::{Query, ResMut},
    },
    reflect::{std_traits::ReflectDefault, Reflect},
    render::{
        color::Color,
        mesh::{Mesh, MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexFormat,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use crate::SHADER_HANDLE;

/// Vertex attribute that mixes from FillColor -> StrokeColor
pub const ATTRIBUTE_SHAPE_MIX: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_ShapeMix", 3920, VertexFormat::Float32);

#[derive(Asset, AsBindGroup, Reflect, Default, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct VectorGraphicMaterial {
    #[uniform(0)]
    fill_color: Color,
    #[uniform(1)]
    stroke_color: Color,
}

impl Material2d for VectorGraphicMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Handle(SHADER_HANDLE.clone())
    }
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Handle(SHADER_HANDLE.clone())
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Position + theme mix vertex attributes
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            ATTRIBUTE_SHAPE_MIX.at_shader_location(5),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct FillColor(pub Color);

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct StrokeColor(pub Color);

#[allow(clippy::type_complexity)]
pub fn sys_sync_vector_graphic_material(
    mut res_vector_graphic_materials: ResMut<Assets<VectorGraphicMaterial>>,
    q_vector_graphic: Query<
        (
            &Handle<VectorGraphicMaterial>,
            Option<&FillColor>,
            Option<&StrokeColor>,
        ),
        Or<(Changed<FillColor>, Changed<StrokeColor>)>
    >,
) {

    for (handle, fill, stroke) in q_vector_graphic.iter() {
        let Some(mat) = res_vector_graphic_materials.get_mut(handle) else {
            continue;
        };
        if let Some(fill) = fill {
            mat.fill_color = fill.0;
        }
        if let Some(stroke) = stroke {
            mat.stroke_color = stroke.0;
        }
    }
}
