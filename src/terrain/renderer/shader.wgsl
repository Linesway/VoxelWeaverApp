
struct Uniforms {
    trans: mat4x4<f32>,
    bounds_min: vec4<f32>,
    bounds_max: vec4<f32>,
};

@group(0) @binding(0) 
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var atlas: texture_2d<f32>;

@group(1) @binding(1)
var atlas_sampler: sampler;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) norm: vec3<f32>,
    @location(2) mat1: u32,
    @location(3) mat2: u32,
    @location(4) mat_weight: f32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) norm: vec3<f32>,
    @location(1) pos: vec3<f32>,
    @location(2) @interpolate(flat) mat1: u32,
    @location(3) @interpolate(flat) mat2: u32,
    @location(4) mat_weight: f32
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.norm = model.norm;
    out.clip_position = uniforms.trans * vec4<f32>(model.pos, 1.0);
    out.pos = model.pos;
    out.mat1 = model.mat1;
    out.mat2 = model.mat2;
    out.mat_weight = model.mat_weight;
    return out;
}

const atlas_size: f32 = 8192.0; 
const mat_size: f32 = 512.0;
const atlas_x_cells = u32(atlas_size / mat_size);

fn sample_atlas_cell(cell_x: f32, cell_y: f32, uv: vec2<f32>) -> vec4<f32> {
    let cell_uv = vec2(cell_x, cell_y) + fract(uv) * (mat_size - 4.0) / mat_size + 2.0 / mat_size; 
    let tex_uv = cell_uv / (atlas_size / mat_size);
    return textureSample(atlas, atlas_sampler, tex_uv);
}

fn sample_albedo(id: u32, uv: vec2<f32>) -> vec4<f32> {
    return sample_atlas_cell(f32(id % atlas_x_cells), f32(id / atlas_x_cells), uv * 0.05);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if  in.pos.x < uniforms.bounds_min.x || in.pos.y < uniforms.bounds_min.y || in.pos.z < uniforms.bounds_min.z ||
        in.pos.x > uniforms.bounds_max.x || in.pos.y > uniforms.bounds_max.y || in.pos.z > uniforms.bounds_max.z {
            discard;
    } 

    let norm = normalize(in.norm);
    let norm_abs = abs(norm);
    let triplanar_weights = norm_abs / (norm_abs.x + norm_abs.y + norm_abs.z);
    let albedo1 =
        sample_albedo(u32(in.mat1), in.pos.yz) * triplanar_weights.x +
        sample_albedo(u32(in.mat1), in.pos.xz) * triplanar_weights.y +
        sample_albedo(u32(in.mat1), in.pos.xy) * triplanar_weights.z;
    let albedo2 =
        sample_albedo(u32(in.mat2), in.pos.yz) * triplanar_weights.x +
        sample_albedo(u32(in.mat2), in.pos.xz) * triplanar_weights.y +
        sample_albedo(u32(in.mat2), in.pos.xy) * triplanar_weights.z;
    let albedo = in.mat_weight * albedo2 + (1.0 - in.mat_weight) * albedo1;

    let light = dot(norm, normalize(vec3(1.0, 1.0, 1.0))) * 0.25 + 0.75;
    
    return albedo * light; 
    // return vec4(f32(in.mat1) / 2.0, f32(in.mat2) / 2.0, 0.0, 1.0);
    // return vec4(light, light, light, 1.0);
}
