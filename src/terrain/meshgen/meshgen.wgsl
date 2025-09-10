
@group(0) @binding(0)
var<storage, read_write> mesh: array<f32>;

struct Params {
    curr_idx: atomic<u32>
}

@group(0) @binding(1)
var<storage, read_write> params: Params; 

@group(0) @binding(2)
var<storage> tri_table: array<i32, 4096>;

struct Uniforms {
    begin: vec3<f32>
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

fn lerp_verts(a: vec3<f32>, a_val: f32, b: vec3<f32>, b_val: f32) -> vec3<f32> {
    if abs(a_val) < 0.01 {
        return a;
    }
    if abs(b_val) < 0.01 {
        return b;
    }
    let m = -a_val / (b_val - a_val);
    return a + m * (b - a);
}

fn lerp_terrain_biome_weights(a_: array<f32, N_BIOMES>, a_val: f32, b_: array<f32, N_BIOMES>, b_val: f32) -> array<f32, N_BIOMES> {
    var a = a_;
    var b = b_;
    var w: array<f32, N_BIOMES>;
    if abs(a_val) < 0.01 {
        return a;
    }
    if abs(b_val) < 0.01 {
        return b;
    }
    let m = -a_val / (b_val - a_val);
    for(var i = 0u; i < N_BIOMES; i++) {
        w[i] = a[i] + m * (b[i] - a[i]);
    }
    return w;
}

fn set_vert(idx: u32, pos: vec3<f32>, norm: vec3<f32>, material: ReducedMaterial) {
    mesh[idx * 9 + 0] = pos.x;
    mesh[idx * 9 + 1] = pos.y;
    mesh[idx * 9 + 2] = pos.z;
    mesh[idx * 9 + 3] = norm.x;
    mesh[idx * 9 + 4] = norm.y;
    mesh[idx * 9 + 5] = norm.z;
    mesh[idx * 9 + 6] = bitcast<f32>(material.tex1);
    mesh[idx * 9 + 7] = bitcast<f32>(material.tex2);
    mesh[idx * 9 + 8] = material.w;
}

@compute @workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) id : vec3<u32>
) {

    let scl = 1.0;

    let v000 = scl * vec3(f32(id.x + 0), f32(id.y + 0), f32(id.z + 0)) + uniforms.begin; 
    let v100 = scl * vec3(f32(id.x + 1), f32(id.y + 0), f32(id.z + 0)) + uniforms.begin; 
    let v010 = scl * vec3(f32(id.x + 0), f32(id.y + 1), f32(id.z + 0)) + uniforms.begin; 
    let v001 = scl * vec3(f32(id.x + 0), f32(id.y + 0), f32(id.z + 1)) + uniforms.begin; 
    let v110 = scl * vec3(f32(id.x + 1), f32(id.y + 1), f32(id.z + 0)) + uniforms.begin; 
    let v101 = scl * vec3(f32(id.x + 1), f32(id.y + 0), f32(id.z + 1)) + uniforms.begin; 
    let v011 = scl * vec3(f32(id.x + 0), f32(id.y + 1), f32(id.z + 1)) + uniforms.begin; 
    let v111 = scl * vec3(f32(id.x + 1), f32(id.y + 1), f32(id.z + 1)) + uniforms.begin; 

    let t000 = sdf(v000);
    let t100 = sdf(v100);
    let t010 = sdf(v010);
    let t001 = sdf(v001);
    let t110 = sdf(v110);
    let t101 = sdf(v101);
    let t011 = sdf(v011);
    let t111 = sdf(v111);

    let s000 = t000.terrain.sdf;
    let s100 = t100.terrain.sdf;
    let s010 = t010.terrain.sdf;
    let s001 = t001.terrain.sdf;
    let s110 = t110.terrain.sdf;
    let s101 = t101.terrain.sdf;
    let s011 = t011.terrain.sdf;
    let s111 = t111.terrain.sdf;

    let idx = 
        (u32(s000 > 0.0) << 0) |
        (u32(s100 > 0.0) << 1) |
        (u32(s010 > 0.0) << 3) |
        (u32(s110 > 0.0) << 2) |
        (u32(s001 > 0.0) << 4) |
        (u32(s101 > 0.0) << 5) |
        (u32(s011 > 0.0) << 7) |
        (u32(s111 > 0.0) << 6);

    let e0  = lerp_verts(v000, s000, v100, s100);    
    let e1  = lerp_verts(v100, s100, v110, s110);    
    let e2  = lerp_verts(v010, s010, v110, s110);    
    let e3  = lerp_verts(v000, s000, v010, s010);    
    let e4  = lerp_verts(v001, s001, v101, s101);    
    let e5  = lerp_verts(v101, s101, v111, s111);    
    let e6  = lerp_verts(v011, s011, v111, s111);    
    let e7  = lerp_verts(v001, s001, v011, s011);    
    let e8  = lerp_verts(v000, s000, v001, s001);    
    let e9  = lerp_verts(v100, s100, v101, s101);    
    let e10 = lerp_verts(v110, s110, v111, s111);    
    let e11 = lerp_verts(v010, s010, v011, s011);

    let b0  = lerp_terrain_biome_weights(t000.biome_w, s000, t100.biome_w, s100);    
    let b1  = lerp_terrain_biome_weights(t100.biome_w, s100, t110.biome_w, s110);    
    let b2  = lerp_terrain_biome_weights(t010.biome_w, s010, t110.biome_w, s110);    
    let b3  = lerp_terrain_biome_weights(t000.biome_w, s000, t010.biome_w, s010);    
    let b4  = lerp_terrain_biome_weights(t001.biome_w, s001, t101.biome_w, s101);    
    let b5  = lerp_terrain_biome_weights(t101.biome_w, s101, t111.biome_w, s111);    
    let b6  = lerp_terrain_biome_weights(t011.biome_w, s011, t111.biome_w, s111);    
    let b7  = lerp_terrain_biome_weights(t001.biome_w, s001, t011.biome_w, s011);    
    let b8  = lerp_terrain_biome_weights(t000.biome_w, s000, t001.biome_w, s001);    
    let b9  = lerp_terrain_biome_weights(t100.biome_w, s100, t101.biome_w, s101);    
    let b10 = lerp_terrain_biome_weights(t110.biome_w, s110, t111.biome_w, s111);    
    let b11 = lerp_terrain_biome_weights(t010.biome_w, s010, t011.biome_w, s011);

    var edges =    array(e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11);
    var biome_ws = array(b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11);

    for (var i = 0u; i < 15u; i += 3u) {
        if tri_table[idx * 16u + i] < 0 {
            break;
        }
        let p1 = edges[tri_table[idx * 16 + i + 0]];
        let p2 = edges[tri_table[idx * 16 + i + 1]];
        let p3 = edges[tri_table[idx * 16 + i + 2]];

        let biome_w1 = biome_ws[tri_table[idx * 16 + i + 0]];
        let biome_w2 = biome_ws[tri_table[idx * 16 + i + 1]];
        let biome_w3 = biome_ws[tri_table[idx * 16 + i + 2]];

        let norm = normalize(cross(p1 - p2, p1 - p3));

        let begin = atomicAdd(&params.curr_idx, 3u);

        set_vert(begin + 0, p1, norm, reduce_material(biome_w1));
        set_vert(begin + 1, p2, norm, reduce_material(biome_w2));
        set_vert(begin + 2, p3, norm, reduce_material(biome_w3));

    }

}
