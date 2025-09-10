
use std::fmt::Write;

use crate::biome::Biomes;

use super::CompilationTarget;

fn i32_var_decl(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "var",
        CompilationTarget::UnrealHLSL => "int",
    }
}

fn f32_var_decl(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "var",
        CompilationTarget::UnrealHLSL => "float",
    }
}

fn f32_typename(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "f32",
        CompilationTarget::UnrealHLSL => "float",
    }
}

fn vec2_var_decl(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "var",
        CompilationTarget::UnrealHLSL => "float2",
    }
}

fn vec2_typename(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "vec2",
        CompilationTarget::UnrealHLSL => "float2",
    }
}

fn mix_fn_name(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "mix",
        CompilationTarget::UnrealHLSL => "lerp",
    }
}

fn fract_fn_name(target: CompilationTarget) -> &'static str {
    match target {
        CompilationTarget::WGSL => "fract",
        CompilationTarget::UnrealHLSL => "frac",
    }
}

fn compile_biome_distribution_layer(out: &mut String, biomes: &Biomes, min_depth: i32, max_depth: i32, target: CompilationTarget) {

    match target {
        CompilationTarget::WGSL => { let _ = writeln!(out, "\tvar biome_w: array<f32, {}>;", biomes.biomes.len()); },
        CompilationTarget::UnrealHLSL => { let _ = writeln!(out, "\tfloat biome_w[{}];", biomes.biomes.len()); },
    }

    for i in 0..biomes.biomes.len() {
        let _ = writeln!(out, "\tbiome_w[{}] = 0.0;", i);
    }

    if biomes.biomes.len() == 1 {
        let _ = writeln!(out, "\tbiome_w[0] = 1.0;");
    } else {
        
        let mut biomes_in_layer = Vec::new();
        for (idx, biome) in biomes.biomes.iter().enumerate() {
            let intersection_min = biome.min_depth.max(min_depth);
            let intersection_max = biome.max_depth.min(max_depth);
            if intersection_max <= intersection_min {
                continue;
            }
            biomes_in_layer.push((idx, biome));
        }

        // https://www.shadertoy.com/view/ldB3zc
        let _ = writeln!(out, "\t{} biome_scl = {};", f32_var_decl(target), 1.0 / biomes.biome_size); 
        let _ = writeln!(out, "\t{} biome_smoothing = {};", f32_var_decl(target), biomes.biome_blending); 
        let _ = writeln!(out, "\t{0} biome_uv = {1}(pos.x * biome_scl, pos.z * biome_scl);", vec2_var_decl(target), vec2_typename(target)); 
        let _ = writeln!(out, "\t{} biome_uv_n = floor(biome_uv);", vec2_var_decl(target)); 
        let _ = writeln!(out, "\t{} biome_uv_f = {}(biome_uv);", vec2_var_decl(target), fract_fn_name(target)); 
        let _ = writeln!(out, "\t{} dist = 8.0;", f32_var_decl(target));
        let _ = writeln!(out, "\tfor({} i = -2; i <= 2; i++) {{", i32_var_decl(target));
        let _ = writeln!(out, "\t\tfor({} j = -2; j <= 2; j++) {{", i32_var_decl(target));
        let _ = writeln!(out, "\t\t\t{0} g = {1}({2}(i), {2}(j));", vec2_var_decl(target), vec2_typename(target), f32_typename(target));
        let _ = writeln!(out, "\t\t\t{} o = hash2(biome_uv_n + g);", vec2_var_decl(target));
        let _ = writeln!(out, "\t\t\t{} d = length(g - biome_uv_f + o);", f32_var_decl(target));
        let seed_vec = match target {
            CompilationTarget::WGSL => "vec2(f32(seed), f32(seed))",
            CompilationTarget::UnrealHLSL => "float2(seed, seed)",
        };
        let _ = writeln!(out, "\t\t\t{} biome = hash2(biome_uv_n + g + {}).x;", f32_var_decl(target), seed_vec);
        let _ = writeln!(out, "\t\t\t{} h = smoothstep(-1.0, 1.0, (dist - d) / biome_smoothing);", f32_var_decl(target));
        let _ = writeln!(out, "\t\t\tdist = {}(dist, d, h) - h * (1.0 - h) * biome_smoothing / (1.0 + 3.0 * biome_smoothing);", mix_fn_name(target));

        let total_biome_freq: f32 = biomes_in_layer.iter().map(|(_, biome)| biome.frequency()).sum(); 
        let mut curr_biome_min = 0.0;
        for i in 0..biomes_in_layer.len() {
            let min_biome = curr_biome_min / total_biome_freq;
            let max_biome = (curr_biome_min + biomes_in_layer[i].1.frequency()) / total_biome_freq;
            curr_biome_min += biomes_in_layer[i].1.frequency();

            let _ = writeln!(out, "\t\t\tif (biome >= {} && biome < {}) {{", min_biome, max_biome);
            let _ = writeln!(out, "\t\t\t\tbiome_w[{0}] = {1}(biome_w[{0}], 1.0, h) - h * (1.0 - h) * biome_smoothing / (1.0 + 3.0 * biome_smoothing);", biomes_in_layer[i].0, mix_fn_name(target));
            let _ = writeln!(out, "\t\t\t}} else {{");
            let _ = writeln!(out, "\t\t\t\tbiome_w[{0}] = {1}(biome_w[{0}], 0.0, h) - h * (1.0 - h) * biome_smoothing / (1.0 + 3.0 * biome_smoothing);", biomes_in_layer[i].0, mix_fn_name(target));
            let _ = writeln!(out, "\t\t\t}}");
        }
        let _ = writeln!(out, "\t\t}}");
        let _ = writeln!(out, "\t}}");

        let _ = writeln!(out, "\t{} biome_w_sum = 0.0;", f32_var_decl(target));
        let _ = writeln!(out, "\tfor ({} i = 0; i < {}; i++) {{", i32_var_decl(target), biomes.biomes.len());
        let _ = writeln!(out, "\t\tbiome_w_sum += biome_w[i];");
        let _ = writeln!(out, "\t}}");
        let _ = writeln!(out, "\tfor ({} i = 0; i < {}; i++) {{", i32_var_decl(target), biomes.biomes.len());
        let _ = writeln!(out, "\t\tbiome_w[i] /= biome_w_sum;");
        let _ = writeln!(out, "\t}}");
    }

    match target {
        CompilationTarget::WGSL => {
            let _ = writeln!(out, "\treturn biome_w;");
        },
        CompilationTarget::UnrealHLSL => {
            let _ = writeln!(out, "\tBiomeWeights<ProjectName> weights;");   
            let _ = writeln!(out, "\tweights.w = biome_w;");   
            let _ = writeln!(out, "\treturn weights;");
        }
    }

}

pub fn compile_biome_distribution(out: &mut String, biomes: &Biomes, target: CompilationTarget) {

    let mut depth_cuts = Vec::new();
    for biome in &biomes.biomes {
        depth_cuts.push(biome.min_depth);
        depth_cuts.push(biome.max_depth);
    }
    depth_cuts.sort();
    let mut depth_cuts_deduplicated = Vec::new();
    depth_cuts_deduplicated.push(depth_cuts[0]);
    for i in 1..depth_cuts.len() {
        if depth_cuts[i] != depth_cuts[i - 1] {
            depth_cuts_deduplicated.push(depth_cuts[i]);
        }
    }
    let depth_cuts = depth_cuts_deduplicated;

    // Per-layer biome distribution 
    for i in 0..=depth_cuts.len() {
        match target {
            CompilationTarget::WGSL => { let _ = writeln!(out, "fn biome_distribution{}(seed: i32, pos: vec3<f32>) -> array<f32, {}> {{", i, biomes.biomes.len()); },
            CompilationTarget::UnrealHLSL => { let _ = writeln!(out, "BiomeWeights<ProjectName> biome_distribution{}<ProjectName>(int seed, float3 pos) {{", i); }
        }

        let (min_depth, max_depth) = if i == 0 {
            (i32::MIN, depth_cuts[0] + 1)
        } else if i == depth_cuts.len() {
            (*depth_cuts.last().unwrap() - 1, i32::MAX)
        } else {
            (depth_cuts[i - 1], depth_cuts[i])
        };

        compile_biome_distribution_layer(out, biomes, min_depth, max_depth, target);
        let _ = writeln!(out, "}}\n");
    }

    match target {
        CompilationTarget::WGSL => { let _ = writeln!(out, "fn biome_distribution(seed: i32, pos: vec3<f32>) -> array<f32, {}> {{", biomes.biomes.len()); },
        CompilationTarget::UnrealHLSL => { let _ = writeln!(out, "BiomeWeights<ProjectName> biome_distribution<ProjectName>(int seed, float3 pos) {{"); }
    }

    if depth_cuts.len() == 1 {
        let _ = writeln!(out, "\tif (pos.y < {}) {{", depth_cuts[0] as f32 - 0.5);
        let _ = writeln!(out, "\t\treturn biome_distribution0<ProjectName>(seed, pos);");
        let _ = writeln!(out, "\t}} else if (pos.y < {}) {{", depth_cuts[0] as f32 + 0.5);
        let _ = writeln!(out, "\t\treturn lerp_biome_w<ProjectName>(biome_distribution0<ProjectName>(seed, pos), biome_distribution1<ProjectName>(seed, pos), pos.y - {});", depth_cuts[0] as f32 - 0.5);
        let _ = writeln!(out, "\t}} else {{");
        let _ = writeln!(out, "\t\treturn biome_distribution1<ProjectName>(seed, pos);");
        let _ = writeln!(out, "\t}}");
    } else {
        let _ = writeln!(out, "\tif (pos.y < {}) {{", depth_cuts[0] as f32 - 0.5);
        let _ = writeln!(out, "\t\treturn biome_distribution0<ProjectName>(seed, pos);");
        let _ = writeln!(out, "\t}} else if (pos.y < {}) {{", depth_cuts[0] as f32 + 0.5);
        let _ = writeln!(out, "\t\treturn lerp_biome_w<ProjectName>(biome_distribution0<ProjectName>(seed, pos), biome_distribution1<ProjectName>(seed, pos), pos.y - {});", depth_cuts[0] as f32 - 0.5);
        for i in 1..depth_cuts.len() {
            let _ = writeln!(out, "\t}} else if (pos.y < {}) {{", depth_cuts[i] as f32 - 0.5);
            let _ = writeln!(out, "\t\treturn biome_distribution{}<ProjectName>(seed, pos);", i);
            let _ = writeln!(out, "\t}} else if (pos.y < {}) {{", depth_cuts[i] as f32 + 0.5);
            let _ = writeln!(out, "\t\treturn lerp_biome_w<ProjectName>(biome_distribution{}<ProjectName>(seed, pos), biome_distribution{}<ProjectName>(seed, pos), pos.y - {});", i, i + 1, depth_cuts[i] as f32 - 0.5);
        } 
        let _ = writeln!(out, "\t}} else {{");
        let _ = writeln!(out, "\t\treturn biome_distribution{}<ProjectName>(seed, pos);", depth_cuts.len());
        let _ = writeln!(out, "\t}}");
    }

    let _ = writeln!(out, "}}\n");

}

pub fn compile_biome_parameters(out: &mut String, biomes: &Biomes, target: CompilationTarget) {

    for (param_idx, name) in biomes.biome_params.iter().enumerate() {
        let _ = write!(out, "\t{} b_{} = ", f32_var_decl(target), param_idx);
        for (biome_idx, biome) in biomes.biomes.iter().enumerate() {
            if biome_idx > 0 {
                let _ = write!(out, " + ");
            }
            let _ = write!(out, "biome_w{}[{}] * {}", match target {
                CompilationTarget::WGSL => "",
                CompilationTarget::UnrealHLSL => ".w",
            }, biome_idx, biome.params.get(name).unwrap_or(&0.0));
        } 
        let _ = writeln!(out, ";");
    }

}

pub fn compile_biome_preview(biomes: &Biomes) -> String {

    let mut out = format!("const N_BIOMES = {}u;\n", biomes.biomes.len()); 

    compile_biome_distribution(&mut out, biomes, CompilationTarget::WGSL); 

    let _ = writeln!(out, "fn preview_color(pos: vec3<f32>) -> vec3<f32> {{");
    let _ = writeln!(out, "let biome_w = biome_distribution(666, pos);");
    let _ = write!(&mut out, "\treturn ");

    for (biome_idx, biome) in biomes.biomes.iter().enumerate() {
        if biome_idx > 0 {
            let _ = write!(&mut out, " + ");
        }
        let _ = write!(&mut out, "vec3({:?}, {:?}, {:?}) * biome_w[{}]", biome.color[0], biome.color[1], biome.color[2], biome_idx);
    }

    let _ = writeln!(&mut out, ";");
    let _ = writeln!(&mut out, "}}");

    (out + include_str!("common.wgsl") + include_str!("fnl.wgsl")).replace("<ProjectName>", "")
}