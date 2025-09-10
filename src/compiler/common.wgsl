
struct Terrain {
    sdf: f32,
}

fn hash2(p: vec2<f32>) -> vec2<f32> {
    let q = vec2(dot(p, vec2(127.1,311.7)), dot(p, vec2(269.5,183.3)));
    return fract(sin(q) * 43758.5453);
}

fn noise(seed: i32, p: vec3<f32>) -> f32 {
    return fnlSingleOpenSimplex23D(seed, p.x, p.y, p.z);
}

fn noise01(seed: i32, p: vec3<f32>) -> f32 {
    return 0.5 + 0.5 * noise(seed, p);
}

fn cool_noise(seed: i32, p: vec3<f32>) -> f32 {
    var res = 0.0;

    var amp = 1.0;
    var ampFac = 0.5;
    var scl = 0.05;
    var sclFac = 2.0;
    var max = 0.0;
    for (var i = 0; i < 4; i++) {
        res += noise(seed, p * scl) * amp;
        max += amp;
        amp *= ampFac; 
        scl *= sclFac;
    }

    return res / max;
}

fn blob_cave_noise(seed: i32, p: vec3<f32>) -> f32 {
    let threshold = 0.85;
    let scl = 0.03;
    let noise_a = noise(seed, p * scl);
    let noise_b = noise(seed, p * scl + vec3(1000.0));
    return -smoothmin(threshold - noise_a * noise_a, threshold - noise_b * noise_b, 0.3);
}

fn spaghetti_cave_noise(seed: i32, p: vec3<f32>) -> f32 {
    let scl = 0.085;
    let noise_a = 1.0 - abs(cool_noise(seed, p * scl));
    let noise_b = 1.0 - abs(cool_noise(seed + 1984, p * scl));
    return noise_a * noise_b * 2.0 - 1.4;
}

fn smoothmin(a: f32, b: f32, k: f32) -> f32 {
    let x = b - a;
    return 0.5 * (a + b - sqrt(x * x + 4.0 * k * k));
}

fn smoothmax(a: f32, b: f32, k: f32) -> f32 {
    let x = b - a;
    return 0.5 * (a + b + sqrt(x * x + 4.0 * k * k));
}

fn make_terrain(sdf: f32) -> Terrain {
    return Terrain(sdf);
}

fn terrain_union(a: Terrain, b: Terrain) -> Terrain {
    let sdf = smoothmin(a.sdf, b.sdf, 0.5);
    return make_terrain(sdf);
}

fn terrain_intersect(a: Terrain, b: Terrain) -> Terrain {
    let sdf = smoothmax(a.sdf, b.sdf, 0.5);
    return make_terrain(sdf);
}

fn terrain_invert(t: Terrain) -> Terrain {
    return Terrain(-t.sdf);
}

fn terrain_erode(t: Terrain, d: f32) -> Terrain {
    return Terrain(t.sdf + d);
}

fn dezero(a: f32) -> f32 {
    if a > 0.005 {
        return a;
    }
    if a < -0.005 {
        return a;
    }
    if a >= 0 {
        return 0.005;
    } else {
        return -0.005;
    }
}

fn map01(x: f32, min: f32, max: f32) -> f32 {
    return min + (max - min) * x;
}

fn calc_heightmap_coord(pos: vec3<f32>, scale: f32) -> vec3<f32> {
    return 100.0 + 0.007 * pos * scale * vec3(1.0, 0.0, 1.0);
}

fn noise_height(seed: i32, pos: vec3<f32>, min: f32, max: f32, scale: f32) -> f32 {
    var scl = 1.0;
    var amp = 1.0;
    var total_amp = 0.0;
    var total_noise = 0.0;
    for (var i = 0; i < 4; i++) {
        total_noise += noise01(seed, calc_heightmap_coord(pos, scl * scale)) * amp;
        total_amp += amp;
        amp *= 0.5;
        scl *= 2.0;
    }
    return map01(total_noise / total_amp, min, max);
}

fn ridge_height(seed: i32, pos: vec3<f32>, min: f32, max: f32, scale: f32) -> f32 {
    return map01(1.0 - abs(noise(seed, calc_heightmap_coord(pos, scale))), min, max);
}

struct ReducedMaterial {
    tex1: u32,
    tex2: u32,
    w: f32
}

fn lerp_biome_w(a_: array<f32, N_BIOMES>, b_: array<f32, N_BIOMES>, w: f32) -> array<f32, N_BIOMES> {
    var a = a_;
    var b = b_;
    var result: array<f32, N_BIOMES>;
    for(var i = 0u; i < N_BIOMES; i++) {
        result[i] = a[i] * (1.0 - w) + b[i] * w;
    }
    return result;
}
