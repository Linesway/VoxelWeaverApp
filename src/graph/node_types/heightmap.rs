use std::collections::HashMap;

use crate::graph::{GraphProjectInfo, NodeInput, NodeType, Type, Value};


pub struct NoiseHeightmap {
    pub scale: NodeInput,
    pub min: NodeInput,
    pub max: NodeInput,
}

impl NodeType for NoiseHeightmap {

    const LABEL: &'static str = "Noise Heightmap";

    fn make() -> Self {
        Self {
            scale: Value::scalar(1.0).into(),
            min: Value::scalar(0.0).into(),
            max: Value::scalar(10.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("scale", Type::Scalar, &self.scale),
            ("min", Type::Scalar, &self.min),
            ("max", Type::Scalar, &self.max),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("scale", Type::Scalar, &mut self.scale),
            ("min", Type::Scalar, &mut self.min),
            ("max", Type::Scalar, &mut self.max),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("height", Type::Scalar)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = noise_height(seed, pos, {}, {}, {});\n", out_varnames["height"], args["min"], args["max"], args["scale"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = noise_height(seed, pos, {}, {}, {});\n", out_varnames["height"], args["min"], args["max"], args["scale"]).as_str());
    }
    
}

pub struct RidgeHeightmap {
    pub scale: NodeInput,
    pub min: NodeInput,
    pub max: NodeInput
}

impl NodeType for RidgeHeightmap {

    const LABEL: &'static str = "Ridge Heightmap";

    fn make() -> Self {
        Self {
            scale: Value::scalar(1.0).into(),
            min: Value::scalar(0.0).into(),
            max: Value::scalar(50.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("scale", Type::Scalar, &self.scale),
            ("min", Type::Scalar, &self.min),
            ("max", Type::Scalar, &self.max),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("scale", Type::Scalar, &mut self.scale),
            ("min", Type::Scalar, &mut self.min),
            ("max", Type::Scalar, &mut self.max),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("height", Type::Scalar)]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = ridge_height(seed, pos, {}, {}, {});\n", out_varnames["height"], args["min"], args["max"], args["scale"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = ridge_height(seed, pos, {}, {}, {});\n", out_varnames["height"], args["min"], args["max"], args["scale"]).as_str());
    }

}
