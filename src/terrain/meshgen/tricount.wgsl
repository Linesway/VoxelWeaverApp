
@group(0) @binding(0) 
var<storage, read_write> tricount: atomic<u32>;

struct Uniforms {
    begin: vec3<f32>
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

@compute @workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) id : vec3<u32>
) {

    var tricounts = array(
        0u, 1u, 1u, 2u, 1u, 2u, 2u, 3u, 1u, 2u, 2u, 3u, 2u, 3u, 3u, 2u, 1u, 2u, 2u, 3u, 2u, 3u, 3u, 4u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 3u,  
        1u, 2u, 2u, 3u, 2u, 3u, 3u, 4u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 3u, 2u, 3u, 3u, 2u, 3u, 4u, 4u, 3u, 3u, 4u, 4u, 3u, 4u, 5u, 5u, 2u,  
        1u, 2u, 2u, 3u, 2u, 3u, 3u, 4u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 3u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 5u, 3u, 4u, 4u, 5u, 4u, 5u, 5u, 4u,  
        2u, 3u, 3u, 4u, 3u, 4u, 2u, 3u, 3u, 4u, 4u, 5u, 4u, 5u, 3u, 2u, 3u, 4u, 4u, 3u, 4u, 5u, 3u, 2u, 4u, 5u, 5u, 4u, 5u, 2u, 4u, 1u,  
        1u, 2u, 2u, 3u, 2u, 3u, 3u, 4u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 3u, 2u, 3u, 3u, 4u, 3u, 4u, 4u, 5u, 3u, 2u, 4u, 3u, 4u, 3u, 5u, 2u,  
        2u, 3u, 3u, 4u, 3u, 4u, 4u, 5u, 3u, 4u, 4u, 5u, 4u, 5u, 5u, 4u, 3u, 4u, 4u, 3u, 4u, 5u, 5u, 4u, 4u, 3u, 5u, 2u, 5u, 4u, 2u, 1u,  
        2u, 3u, 3u, 4u, 3u, 4u, 4u, 5u, 3u, 4u, 4u, 5u, 2u, 3u, 3u, 2u, 3u, 4u, 4u, 5u, 4u, 5u, 5u, 2u, 4u, 3u, 5u, 4u, 3u, 2u, 4u, 1u,  
        3u, 4u, 4u, 5u, 4u, 5u, 3u, 4u, 4u, 5u, 5u, 2u, 3u, 4u, 2u, 1u, 2u, 3u, 3u, 2u, 3u, 4u, 2u, 1u, 3u, 2u, 4u, 1u, 2u, 1u, 1u, 0u 
    );

    let scl = 1.0;

    let v000 = u32(sdf(scl * vec3(f32(id.x + 0), f32(id.y + 0), f32(id.z + 0)) + uniforms.begin).terrain.sdf > 0.0); 
    let v100 = u32(sdf(scl * vec3(f32(id.x + 1), f32(id.y + 0), f32(id.z + 0)) + uniforms.begin).terrain.sdf > 0.0); 
    let v010 = u32(sdf(scl * vec3(f32(id.x + 0), f32(id.y + 1), f32(id.z + 0)) + uniforms.begin).terrain.sdf > 0.0); 
    let v001 = u32(sdf(scl * vec3(f32(id.x + 0), f32(id.y + 0), f32(id.z + 1)) + uniforms.begin).terrain.sdf > 0.0); 
    let v110 = u32(sdf(scl * vec3(f32(id.x + 1), f32(id.y + 1), f32(id.z + 0)) + uniforms.begin).terrain.sdf > 0.0); 
    let v101 = u32(sdf(scl * vec3(f32(id.x + 1), f32(id.y + 0), f32(id.z + 1)) + uniforms.begin).terrain.sdf > 0.0); 
    let v011 = u32(sdf(scl * vec3(f32(id.x + 0), f32(id.y + 1), f32(id.z + 1)) + uniforms.begin).terrain.sdf > 0.0); 
    let v111 = u32(sdf(scl * vec3(f32(id.x + 1), f32(id.y + 1), f32(id.z + 1)) + uniforms.begin).terrain.sdf > 0.0); 

    let idx = 
        v000 << 0 |
        v100 << 1 |
        v010 << 3 |
        v110 << 2 |
        v001 << 4 |
        v101 << 5 |
        v011 << 7 |
        v111 << 6;

    if tricounts[idx] != 0 {
        atomicAdd(&tricount, tricounts[idx]);
    }
}
