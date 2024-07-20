#import bevy_sprite::{ mesh2d_functions as mesh_functions, mesh2d_view_bindings::view }

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct Vertex {
    @builtin(instance_index) instance_index: u32,
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    // @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    // @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    // @location(4) color: vec4<f32>,
#endif
    @location(5) theme_mix: f32,
    @location(6) theme_base: f32,
    @location(7) theme_base_opacity: f32,
};

struct VertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output 
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    // @location(0) world_position: vec4<f32>,
    // @location(1) world_normal: vec3<f32>,
    // @location(2) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
    // @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    // @location(4) color: vec4<f32>,
    #endif
    @location(5) theme_mix: f32,
    @location(6) theme_base: f32,
    @location(7) theme_base_opacity: f32,
}

struct State {
    selected: u32,
    hovered: u32,
    _wasm_padding_12b: u32,
    _wasm_padding_16b: u32,
}

@group(2) @binding(0) var<uniform> state: State;
@group(2) @binding(1) var<uniform> theme_color: vec4<f32>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_POSITIONS
    // Expands verts outwards (by normal direction) when hovered.
    let scale = (f32(state.hovered) * 1.);
    let pos = vertex.position + vertex.normal * scale;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(pos, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(world_position);
#endif

#ifdef VERTEX_NORMALS
    // out.world_normal = mesh_functions::mesh2d_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif

#ifdef VERTEX_TANGENTS
    // out.world_tangent = mesh_functions::mesh2d_tangent_local_to_world(
    //    model,
    //    vertex.tangent
    // );
#endif
    out.theme_mix = vertex.theme_mix;
    out.theme_base = vertex.theme_base;
    out.theme_base_opacity = vertex.theme_base_opacity;
#ifdef VERTEX_COLORS
    // out.color = vertex.color;
#endif
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
#ifdef VERTEX_COLORS
    var color = in.color;
#ifdef TONEMAP_IN_SHADER
    color = tonemapping::tone_mapping(color, view.color_grading);
#endif
    return color;
#else

    var theme_mix = in.theme_mix;
    if (state.selected == 1) {
        theme_mix = 1. - theme_mix;
    }

    let base = vec3<f32>(in.theme_base);
    var c = mix(base, theme_color.rgb, theme_mix);
    if (state.hovered == 1) {
        c += vec3(0.25, 0.25, 0.25);
    }

    var alpha = in.theme_base_opacity;
    if (state.hovered == 1) {
        alpha = 1.;
    }
    if (state.selected == 1) {
        alpha = 1.;
    }

    return vec4<f32>(c, alpha);
#endif
}
