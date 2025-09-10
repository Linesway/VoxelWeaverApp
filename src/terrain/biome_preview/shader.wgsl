
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) pos: vec3<f32>
};

struct Uniforms {
    aspect: f32,
    depth: f32
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms; 

var<private> QUAD_VERTS: array<vec2<f32>, 6> = array(
    vec2(-1.0, -1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),

    vec2( 1.0,  1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0, -1.0),
);

@vertex
fn vs_main(
    @builtin(vertex_index) vert_idx: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(QUAD_VERTS[vert_idx], 0.0, 1.0);
    out.pos = 1000.0 * vec3(QUAD_VERTS[vert_idx].x, uniforms.depth, QUAD_VERTS[vert_idx].y) * vec3(uniforms.aspect, 1.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(preview_color(in.pos), 1.0); 
}
