
use std::collections::HashMap;
use crate::graph::{GraphProjectInfo, NodeInput, NodeType, Type, Value};
use std::fmt::Write;

pub struct HeightmapTerrain {
    pub height: NodeInput,
}

impl NodeType for HeightmapTerrain {

    const LABEL: &'static str = "Heightmap Terrain";

    fn make() -> Self {
        Self {
            height: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("height", Type::Scalar, &self.height),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("height", Type::Scalar, &mut self.height),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("terrain", Type::Terrain)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = make_terrain(2.0 * smoothstep(-2.5, 2.5, pos.y - {}) - 1.0);\n", out_varnames["terrain"], args["height"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = make_terrain(2.0 * smoothstep(-2.5, 2.5, pos.y - {}) - 1.0);\n", out_varnames["terrain"], args["height"]).as_str());
    }

}

pub struct FractalNoiseTerrain {
    pub scale: NodeInput,
}

impl NodeType for FractalNoiseTerrain {

    const LABEL: &'static str = "Fractal Noise Terrain";

    fn make() -> Self {
        Self {
            scale: Value::scalar(0.3).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("scale", Type::Scalar, &self.scale),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("scale", Type::Scalar, &mut self.scale),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }
    
    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = make_terrain(cool_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = make_terrain(cool_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

}

pub struct BlobCaveTerrain {
    pub scale: NodeInput
}

impl NodeType for BlobCaveTerrain {

    const LABEL: &'static str = "Blob Cave Terrain";

    fn make() -> Self {
        Self {
            scale: Value::scalar(0.3).into() 
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("scale", Type::Scalar, &self.scale)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("scale", Type::Scalar, &mut self.scale)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("terrain", Type::Terrain)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = make_terrain(blob_cave_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = make_terrain(blob_cave_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

}

pub struct SpaghettiCaveTerrain {
    pub scale: NodeInput
}

impl NodeType for SpaghettiCaveTerrain {

    const LABEL: &'static str = "Spaghetti Cave Terrain";

    fn make() -> Self {
        Self {
            scale: Value::scalar(0.3).into() 
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("scale", Type::Scalar, &self.scale)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("scale", Type::Scalar, &mut self.scale)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("terrain", Type::Terrain)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = make_terrain(spaghetti_cave_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = make_terrain(spaghetti_cave_noise(seed, pos * {}));\n", out_varnames["terrain"], args["scale"]).as_str());
    }

}

pub struct TerrainOutput {
    pub terrain: NodeInput
}

impl NodeType for TerrainOutput {
    
    const LABEL: &'static str = "Terrain Output";

    fn make() -> Self {
        Self {
            terrain: Value::terrain().into(),
        }
    }
    
    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![("terrain", Type::Terrain, &self.terrain)]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![("terrain", Type::Terrain, &mut self.terrain)]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![]
    }
    
    fn compile_wgsl(&self, args: HashMap<&'static str, String>, _out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tterrain_out.terrain = {};\n", args["terrain"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, _out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tterrain_out = {};\n", args["terrain"]).as_str());
    }

}

pub struct InvertTerrain {
    pub terrain: NodeInput
}

impl NodeType for InvertTerrain {
    const LABEL: &'static str = "Invert Terrain";

    fn make() -> Self {
        Self {
            terrain: Value::terrain().into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("terrain", Type::Terrain, &self.terrain)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("terrain", Type::Terrain, &mut self.terrain)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = terrain_invert({});\n", out_varnames["terrain"], args["terrain"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = terrain_invert({});\n", out_varnames["terrain"], args["terrain"]).as_str());
    }

}

pub struct ErodeTerrain {
    pub terrain: NodeInput,
    pub depth: NodeInput,
}

impl NodeType for ErodeTerrain {

    const LABEL: &'static str = "Erode Terrain";

    fn make() -> Self {
        Self {
            terrain: Value::terrain().into(),
            depth: Value::scalar(0.5).into()
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("terrain", Type::Terrain, &self.terrain),
            ("depth", Type::Scalar, &self.depth)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("terrain", Type::Terrain, &mut self.terrain),
            ("depth", Type::Scalar, &mut self.depth)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = terrain_erode({}, {});\n", out_varnames["terrain"], args["terrain"], args["depth"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = terrain_erode({}, {});\n", out_varnames["terrain"], args["terrain"], args["depth"]).as_str());
    }

}

pub struct TerrainUnion {
    pub a: NodeInput,
    pub b: NodeInput,
}

impl NodeType for TerrainUnion {

    const LABEL: &'static str = "Terrain Union";

    fn make() -> Self {
        Self {
            a: Value::terrain().into(),
            b: Value::terrain().into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Terrain, &self.a),
            ("b", Type::Terrain, &self.b),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Terrain, &mut self.a),
            ("b", Type::Terrain, &mut self.b),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = terrain_union({}, {});\n", out_varnames["terrain"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = terrain_union({}, {});\n", out_varnames["terrain"], args["a"], args["b"]).as_str());
    }

}

pub struct TerrainIntersection {
    pub a: NodeInput,
    pub b: NodeInput,
}

impl NodeType for TerrainIntersection {

    const LABEL: &'static str = "Terrain Intersection";

    fn make() -> Self {
        Self {
            a: Value::terrain().into(),
            b: Value::terrain().into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Terrain, &self.a),
            ("b", Type::Terrain, &self.b),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Terrain, &mut self.a),
            ("b", Type::Terrain, &mut self.b),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = terrain_intersect({}, {});\n", out_varnames["terrain"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tTerrain {} = terrain_intersect({}, {});\n", out_varnames["terrain"], args["a"], args["b"]).as_str());
    }

}

pub struct TerrainToSDF {
    pub terrain: NodeInput
}

impl NodeType for TerrainToSDF {
    const LABEL: &'static str = "Terrain to SDF";

    fn make() -> Self {
        Self {
            terrain: Value::terrain().into() 
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![("terrain", Type::Terrain, &self.terrain)]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![("terrain", Type::Terrain, &mut self.terrain)]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("sdf", Type::Scalar)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tlet {} = {}.sdf;", out_varnames["sdf"], args["terrain"]);
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tfloat {} = {}.sdf;", out_varnames["sdf"], args["terrain"]);
    }
}


pub struct SDFToTerrain {
    pub terrain: NodeInput
}

impl NodeType for SDFToTerrain {
    const LABEL: &'static str = "SDF To Terrain";

    fn make() -> Self {
        Self {
            terrain: Value::scalar(0.0).into() 
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![("sdf", Type::Scalar, &self.terrain)]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![("sdf", Type::Scalar, &mut self.terrain)]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("terrain", Type::Terrain)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tlet {} = Terrain({});", out_varnames["terrain"], args["sdf"]);
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tTerrain {};", out_varnames["terrain"]);
        let _ = writeln!(out, "\t{}.sdf = {};", out_varnames["terrain"], args["sdf"]);
    }
}