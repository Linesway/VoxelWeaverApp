
pub mod biomes;

mod graph;
use biomes::{compile_biome_distribution, compile_biome_parameters};
use graph::compile_graph;
use std::fmt::Write;

use crate::{app::texture_loader::TextureLoader, biome::Biomes, graph::{GraphProjectInfo, TerrainGraph, Value}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CompilationTarget {
    WGSL,
    UnrealHLSL
}

impl CompilationTarget {

    pub fn blank_sdf(&self) -> &'static str {
        match self {
            CompilationTarget::WGSL => include_str!("compiler/blank.wgsl"),
            CompilationTarget::UnrealHLSL => include_str!("compiler/blank.ush"),
        }
    } 

    pub fn preamble(&self, biomes: &Biomes) -> String {
        match self {
            CompilationTarget::WGSL => concat!(
                include_str!("compiler/common.wgsl"),
                include_str!("compiler/fnl.wgsl"),
            ).to_owned() + format!("
const N_BIOMES = {0}u;
struct TerrainOutput {{
    terrain: Terrain,
    biome_w: array<f32, {0}>
}}\n", biomes.biomes.len()).as_str(),
            CompilationTarget::UnrealHLSL => {
                format!("
#define N_BIOMES<ProjectName> {}
struct BiomeWeights<ProjectName> {{
    float w[N_BIOMES<ProjectName>];
}};

BiomeWeights<ProjectName> lerp_biome_w<ProjectName>(BiomeWeights<ProjectName> a, BiomeWeights<ProjectName> b, float w) {{
    BiomeWeights<ProjectName> result;
    for(int i = 0; i < N_BIOMES<ProjectName>; i++) {{
        result.w[i] = a.w[i] * (1.0 - w) + b.w[i] * w;
    }}
    return result;
}}

", biomes.biomes.len()) + "#pragma once\n"
            }
        }
    }

    pub fn postamble(&self) -> &'static str {
        match self {
            CompilationTarget::WGSL => include_str!("compiler/postamble.wgsl"),
            CompilationTarget::UnrealHLSL => include_str!("compiler/postamble.ush"),
        }
    }

}

impl Value {

    fn to_string(&self, target: CompilationTarget) -> String {
        match self {
            Self::Scalar(val) => format!("{:?}", *val),
            Self::Vector(vec) => format!("{}({:?}, {:?}, {:?})", match target {
                CompilationTarget::WGSL => "vec3",
                CompilationTarget::UnrealHLSL => "float3" 
            }, vec.x, vec.y, vec.z),
            Self::Terrain => match target {
                CompilationTarget::WGSL => "make_terrain(1.0)".to_string(),
                CompilationTarget::UnrealHLSL => "make_terrain(1.0)".to_string(),
            } 
        }
    }

}

fn compile_texture_reducer(out: &mut String, biomes: &Biomes, textures: &TextureLoader) {
    let _ = writeln!(out, "fn reduce_material(biome_w_: array<f32, {}>) -> ReducedMaterial {{", biomes.biomes.len());
    let _ = writeln!(out, "\tvar biome_w = biome_w_;");

    match biomes.biomes.len() {
        0 => {
            let _ = writeln!(out, "\treturn ReducedMaterial(0, 0, 0.0);");
        },
        1 => {
            let slot = textures.get(&biomes.biomes[0].texture);
            let _ = writeln!(out, "\treturn ReducedMaterial({0}, {0}, 0.0);", slot);
        },
        2 => {
            let slot0 = textures.get(&biomes.biomes[0].texture);
            let slot1 = textures.get(&biomes.biomes[1].texture);
            let _ = writeln!(out, "\treturn ReducedMaterial({}, {}, biome_w[1] / (biome_w[0] + biome_w[1]));", slot0, slot1);
        },
        n_biomes => {
            let _ = writeln!(out, "\tvar tex0 = 0;");
            let _ = writeln!(out, "\tvar tex1 = 1;");
            let _ = writeln!(out, "\tif biome_w[1] > biome_w[0] {{");
            let _ = writeln!(out, "\t\ttex0 = 1;");
            let _ = writeln!(out, "\t\ttex1 = 0;");
            let _ = writeln!(out, "\t}}");
            let _ = writeln!(out, "\tfor(var i = 2; i < {}; i++) {{", n_biomes);
            let _ = writeln!(out, "\t\tif biome_w[i] > biome_w[tex0] {{");
            let _ = writeln!(out, "\t\t\ttex1 = tex0;");
            let _ = writeln!(out, "\t\t\ttex0 = i;");
            let _ = writeln!(out, "\t\t}} else if biome_w[i] > biome_w[tex1] {{");
            let _ = writeln!(out, "\t\t\ttex1 = i;");
            let _ = writeln!(out, "\t\t}}");
            let _ = writeln!(out, "\t}}");
            let _ = writeln!(out, "\tlet tex_w = biome_w[tex1] / (biome_w[tex0] + biome_w[tex1]);");
            let _ = write!(out, "\tvar tex_slot_map = array(");
            for biome in &biomes.biomes {
                let _ = write!(out, "{}u, ", textures.get(&biome.texture));
            }
            let _ = writeln!(out, ");");
            let _ = writeln!(out, "\treturn ReducedMaterial(tex_slot_map[tex0], tex_slot_map[tex1], tex_w);");
        }
    }

    let _ = writeln!(out, "}}");
}

pub fn compile(graph: &TerrainGraph, biomes: &Biomes, textures: &TextureLoader, target: CompilationTarget, project_name : Option<&str>) -> String {
    let mut out = target.preamble(biomes);

    compile_biome_distribution(&mut out, biomes, target);

    let _ = writeln!(out, "{}", match target {
        CompilationTarget::WGSL => include_str!("compiler/preamble.wgsl"),
        CompilationTarget::UnrealHLSL => include_str!("compiler/preamble.ush"),
    });

    match target {
        CompilationTarget::WGSL => { let _ = writeln!(out, "let biome_w = biome_distribution<ProjectName>(seed, pos);"); },
        CompilationTarget::UnrealHLSL => { let _ = writeln!(out, "BiomeWeights<ProjectName> biome_w = biome_distribution<ProjectName>(seed, pos);"); },
    }

    compile_biome_parameters(&mut out, biomes, target);

    compile_graph(&mut out, graph, target, &GraphProjectInfo {
        biomes,
    });

    out.push_str(&target.postamble());

    if target == CompilationTarget::WGSL {
        compile_texture_reducer(&mut out, biomes, textures);
        out = out.replace("<ProjectName>", "");
    }
    if target == CompilationTarget::UnrealHLSL {
        out.push_str(include_str!("compiler/vertex_color.ush"));
        out = out.replace("<ProjectName>", project_name.unwrap_or("DefaultProject"));
    }

    out
}
