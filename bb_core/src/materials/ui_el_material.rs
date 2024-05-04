use std::mem::size_of;

use bevy::{
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError, VertexAttribute, VertexFormat
        },
    },
    sprite::{Material2d, Material2dKey},
};


pub const ATTRIBUTE_THEME_MIX: MeshVertexAttribute = MeshVertexAttribute::new("Vertex_ThemeMix", 3330, VertexFormat::Float32);
use crate::plugins::selected::{Hovered, Selected};

#[repr(C)]
#[derive(ShaderType, Debug, Clone, Default, Reflect)]
#[reflect(Default, Debug)]
pub struct UiElState {
    pub selected: u32,
    pub hovered: u32,

    // Needs to be 16 byte aligned on wasm
    _wasm_padding_12b: u32,
    _wasm_padding_16b: u32,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct UiElementMaterial {
    #[uniform(0)]
    pub state: UiElState,
    #[uniform(1)]
    pub theme_color: Color,
}

impl Default for UiElementMaterial {
    fn default() -> Self {
        Self {
            state: UiElState::default(),
            theme_color: Color::rgba(0.033, 0.527, 0.869, 1.)
        }
    }
}

impl UiElementMaterial {
    pub fn get_hovered(&self) -> bool {
        self.state.hovered == 1
    }
    pub fn set_hovered(&mut self, hovered: bool) {
        if hovered {
            self.state.hovered = 1;
        } else {
            self.state.hovered = 0;
        }
    }
    pub fn get_selected(&self) -> bool {
        self.state.selected == 1
    }
    pub fn set_selected(&mut self, selected: bool) {
        if selected {
            self.state.selected = 1;
        } else {
            self.state.selected = 0;
        }
    }
}

impl Material2d for UiElementMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ui_element_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/ui_element_material.wgsl".into()
    }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Position + theme mix vertex attributes
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_THEME_MIX.at_shader_location(5),
        ])?;
        println!("descriptor layout: {:?}", descriptor.layout);
        // Adds theme_mix varying for frag shader
        // layout.layout().attributes.push(VertexAttribute {
        //     format: VertexFormat::Float32,
        //     offset: size_of::<f32>() as u64,
        //     shader_location: 5,
        // });
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

// TODO: Implement a resource that caches these materials so they can be re-used for instancing.
pub fn sys_update_ui_element_materials(
    mut material_store: ResMut<Assets<UiElementMaterial>>,
    mut q: Query<(&Selected, &Hovered, &Handle<UiElementMaterial>)>,
) {
    for (selected, hovered, handle) in q.iter_mut() {
        if let Some(ui_el_material) = material_store.get_mut(handle) {
            let is_selected = selected.is_selected();
            ui_el_material.set_selected(is_selected);

            let is_hovered = hovered.is_hovered();
            ui_el_material.set_hovered(is_hovered);
        }
    }
}
