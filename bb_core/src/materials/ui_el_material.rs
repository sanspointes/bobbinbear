
use bevy::{
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError, VertexFormat
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};


pub const ATTRIBUTE_THEME_MIX: MeshVertexAttribute = MeshVertexAttribute::new("Vertex_ThemeMix", 3330, VertexFormat::Float32);
pub const ATTRIBUTE_THEME_BASE: MeshVertexAttribute = MeshVertexAttribute::new("Vertex_ThemeBase", 3331, VertexFormat::Float32);
pub const ATTRIBUTE_THEME_BASE_OPACITY: MeshVertexAttribute = MeshVertexAttribute::new("Vertex_ThemeBaseOpacity", 3332, VertexFormat::Float32);

use crate::plugins::selected::{Hovered, Selected};

pub struct UiElementMaterialPlugin;

impl Plugin for UiElementMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiElementMaterial>();
        app.add_plugins(Material2dPlugin::<UiElementMaterial>::default());
        app.insert_resource(Assets::<UiElementMaterial>::default());

        let mut materials = app.world.resource_mut::<Assets<UiElementMaterial>>();
        let default = materials.add(UiElementMaterial::default());
        let selected = materials.add(UiElementMaterial {
            state: UiElState {
                selected: 1,
                ..Default::default()
            },
            ..Default::default()
        });
        let hovered = materials.add(UiElementMaterial {
            state: UiElState {
                hovered: 1,
                ..Default::default()
            },
            ..Default::default()
        });
        let selected_and_hovered = materials.add(UiElementMaterial {
            state: UiElState {
                selected: 1,
                hovered: 1,
                ..Default::default()
            },
            ..Default::default()
        });

        app.insert_resource(UiElementMaterialCache {
            default,
            selected,
            hovered,
            selected_and_hovered
        });

        app.add_systems(PostUpdate, sys_update_ui_element_materials);
    }
}

#[derive(Debug, Resource)]
pub struct UiElementMaterialCache {
    pub default: Handle<UiElementMaterial>,
    pub selected: Handle<UiElementMaterial>,
    pub hovered: Handle<UiElementMaterial>,
    pub selected_and_hovered: Handle<UiElementMaterial>,
}

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
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            ATTRIBUTE_THEME_MIX.at_shader_location(5),
            ATTRIBUTE_THEME_BASE.at_shader_location(6),
            ATTRIBUTE_THEME_BASE_OPACITY.at_shader_location(7),
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
    material_cache: ResMut<UiElementMaterialCache>,
    mut q: Query<(&Selected, &Hovered, &mut Handle<UiElementMaterial>)>,
) {
    for (selected, hovered, mut handle) in q.iter_mut() {
        match (selected, hovered) {
            (Selected::Deselected, Hovered::Unhovered) => *handle = material_cache.default.clone(),
            (Selected::Deselected, Hovered::Hovered) => *handle = material_cache.hovered.clone(),
            (Selected::Selected, Hovered::Unhovered) => *handle = material_cache.selected.clone(),
            (Selected::Selected, Hovered::Hovered) => *handle = material_cache.selected_and_hovered.clone(),
        }
    }
}
