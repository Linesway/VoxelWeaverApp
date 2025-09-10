
use crate::graph::{NodeInput, NodeType, Type, Value};
use std::fmt::Write;

pub struct Noise3D {
    pos: NodeInput,
    size: NodeInput,
    amplitude: NodeInput,
    lacunarity: NodeInput,
    gain: NodeInput,
}

impl NodeType for Noise3D {
    const LABEL: &'static str = "Noise 3D";

    fn make() -> Self {
        Self {
            pos: Value::vector(0.0, 0.0, 0.0).into(),
            size: Value::scalar(1.0).into(),
            amplitude: Value::scalar(1.0).into(),
            lacunarity: Value::scalar(2.0).into(),
            gain: Value::scalar(0.5).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, crate::graph::Type, &NodeInput)> {
        vec![
            ("pos", Type::Vector, &self.pos),
            ("size", Type::Scalar, &self.size),
            ("amplitude", Type::Scalar, &self.amplitude),
            ("lacunarity", Type::Scalar, &self.lacunarity),
            ("gain", Type::Scalar, &self.gain),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, crate::graph::Type, &mut NodeInput)> {
        vec![
            ("pos", Type::Vector, &mut self.pos),
            ("size", Type::Scalar, &mut self.size),
            ("amplitude", Type::Scalar, &mut self.amplitude),
            ("lacunarity", Type::Scalar, &mut self.lacunarity),
            ("gain", Type::Scalar, &mut self.gain),
        ]
    }

    fn outputs() -> Vec<(&'static str, crate::graph::Type)> {
        vec![("noise", Type::Scalar)]
    }

    fn compile_wgsl(&self, args: std::collections::HashMap<&'static str, String>, out_varnames: std::collections::HashMap<&'static str, String>, out: &mut String, _info: &crate::graph::GraphProjectInfo) {
        let _ = write!(out, "\tvar {} = 0.0", out_varnames["noise"]);
        let mut size = args["size"].clone(); 
        let mut amp = args["amplitude"].clone(); 
        for _i in 0..4 {
            let _ = write!(out, "+ {0} * noise01(seed, {1} * max({2} * 0.2, 0.0))", amp, args["pos"], size);
            let _ = write!(size, " * {}", args["lacunarity"]);
            let _ = write!(amp, " * {}", args["gain"]);
        }
        let _ = writeln!(out, ";");
    }

    fn compile_hlsl(&self, args: std::collections::HashMap<&'static str, String>, out_varnames: std::collections::HashMap<&'static str, String>, out: &mut String, _info: &crate::graph::GraphProjectInfo) {
        let _ = write!(out, "\tfloat {} = 0.0", out_varnames["noise"]);
        let mut size = args["size"].clone(); 
        let mut amp = args["amplitude"].clone(); 
        for _i in 0..4 {
            let _ = write!(out, "+ {0} * noise01(seed, {1} * max({2} * 0.2, 0.0))", amp, args["pos"], size);
            let _ = write!(size, " * {}", args["lacunarity"]);
            let _ = write!(amp, " * {}", args["gain"]);
        }
        let _ = writeln!(out, ";");
    }

}

pub struct Noise2D {
    pos: NodeInput,
    size: NodeInput,
    amplitude: NodeInput,
    lacunarity: NodeInput,
    gain: NodeInput,
}

impl NodeType for Noise2D {
    const LABEL: &'static str = "Noise 2D";

    fn make() -> Self {
        Self {
            pos: Value::vector(0.0, 0.0, 0.0).into(),
            size: Value::scalar(1.0).into(),
            amplitude: Value::scalar(1.0).into(),
            lacunarity: Value::scalar(2.0).into(),
            gain: Value::scalar(0.5).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, crate::graph::Type, &NodeInput)> {
        vec![
            ("pos", Type::Vector, &self.pos),
            ("size", Type::Scalar, &self.size),
            ("amplitude", Type::Scalar, &self.amplitude),
            ("lacunarity", Type::Scalar, &self.lacunarity),
            ("gain", Type::Scalar, &self.gain),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, crate::graph::Type, &mut NodeInput)> {
        vec![
            ("pos", Type::Vector, &mut self.pos),
            ("size", Type::Scalar, &mut self.size),
            ("amplitude", Type::Scalar, &mut self.amplitude),
            ("lacunarity", Type::Scalar, &mut self.lacunarity),
            ("gain", Type::Scalar, &mut self.gain),
        ]
    }

    fn outputs() -> Vec<(&'static str, crate::graph::Type)> {
        vec![("noise", Type::Scalar)]
    }

    fn compile_wgsl(&self, args: std::collections::HashMap<&'static str, String>, out_varnames: std::collections::HashMap<&'static str, String>, out: &mut String, _info: &crate::graph::GraphProjectInfo) {
        let _ = write!(out, "\tvar {} = 0.0", out_varnames["noise"]);
        let mut size = args["size"].clone(); 
        let mut amp = args["amplitude"].clone(); 
        for _i in 0..4 {
            let _ = write!(out, "+ {0} * noise01(seed, vec3(1.0, 0.0, 1.0) * {1} * max({2} * 0.2, 0.0))", amp, args["pos"], size);
            let _ = write!(size, " * {}", args["lacunarity"]);
            let _ = write!(amp, " * {}", args["gain"]);
        }
        let _ = writeln!(out, ";");
    }

    fn compile_hlsl(&self, args: std::collections::HashMap<&'static str, String>, out_varnames: std::collections::HashMap<&'static str, String>, out: &mut String, _info: &crate::graph::GraphProjectInfo) {
        let _ = write!(out, "\tfloat {} = 0.0", out_varnames["noise"]);
        let mut size = args["size"].clone(); 
        let mut amp = args["amplitude"].clone(); 
        for _i in 0..4 {
            let _ = write!(out, "+ {0} * noise01(seed, float3(1.0, 0.0, 1.0) * {1} * max({2} * 0.2, 0.0))", amp, args["pos"], size);
            let _ = write!(size, " * {}", args["lacunarity"]);
            let _ = write!(amp, " * {}", args["gain"]);
        }
        let _ = writeln!(out, ";");
    }
    
}
