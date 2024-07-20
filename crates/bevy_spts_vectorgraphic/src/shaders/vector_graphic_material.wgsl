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
    @location(5) shape_mix: f32,
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
    @location(5) shape_mix: f32,
}

@group(2) @binding(0) var<uniform> fill_color: vec4<f32>;
@group(2) @binding(1) var<uniform> stroke_color: vec4<f32>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_POSITIONS
    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    let pos = vertex.position + vec3<f32>(0., vertex.shape_mix * 0.01, 0.);
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
    out.shape_mix = vertex.shape_mix;
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
    return mix(fill_color, stroke_color, in.shape_mix);
#endif
}
