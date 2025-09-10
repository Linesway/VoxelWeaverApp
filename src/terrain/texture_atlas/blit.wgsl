
@group(0) @binding(0)
var atlas: texture_storage_2d<rgba8unorm, write>;

struct Loc {
    x: u32,
    y: u32,
    size: u32
}

@group(0) @binding(1)
var<uniform> loc: Loc;

@group(1) @binding(0)
var src: texture_2d<f32>;

@compute @workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) id : vec3<u32>
) {
    let uv = clamp(vec2(f32(id.x - 1), f32(id.y - 1)) / f32(loc.size - 3), vec2(0.0, 0.0), vec2(1.0, 1.0));
    let pixCoords = floor(uv * vec2<f32>(textureDimensions(src)));
    let col: vec4<f32> = textureLoad(src, vec2(u32(pixCoords.x), u32(pixCoords.y)), 0);
    let col_rgb = vec4(
        pow(col.x, 1.0 / 2.2),
        pow(col.y, 1.0 / 2.2),
        pow(col.z, 1.0 / 2.2),
        1.0
    );
    textureStore(atlas, vec2(loc.x + id.x, loc.y + id.y), col_rgb);
}
