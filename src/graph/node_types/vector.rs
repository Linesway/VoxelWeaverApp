use std::collections::HashMap;

use crate::graph::{GraphProjectInfo, NodeInput, NodeType, Type, Value};
use std::fmt::Write;

pub struct CombineXYZ {
    x: NodeInput,
    y: NodeInput,
    z: NodeInput
}

impl NodeType for CombineXYZ {
    const LABEL: &'static str = "Combine XYZ";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
            y: Value::scalar(0.0).into(),
            z: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
            ("y", Type::Scalar, &self.y),
            ("z", Type::Scalar, &self.z),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
            ("y", Type::Scalar, &mut self.y),
            ("z", Type::Scalar, &mut self.z),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("vec", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tlet {} = vec3({}, {}, {});", out_varnames["vec"], args["x"], args["y"], args["z"]);
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tfloat3 {} = float3({}, {}, {});", out_varnames["vec"], args["x"], args["y"], args["z"]);
    }

}

pub struct SeparateXYZ {
    vec: NodeInput
}

impl NodeType for SeparateXYZ {
    const LABEL: &'static str = "Separate XYZ";

    fn make() -> Self {
        Self {
            vec: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![("vec", Type::Vector, &self.vec)]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![("vec", Type::Vector, &mut self.vec)]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("x", Type::Scalar),
            ("y", Type::Scalar),
            ("z", Type::Scalar),
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tlet {} = {}.x;", out_varnames["x"], args["vec"]);
        let _ = writeln!(out, "\tlet {} = {}.y;", out_varnames["y"], args["vec"]);
        let _ = writeln!(out, "\tlet {} = {}.z;", out_varnames["z"], args["vec"]);
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        let _ = writeln!(out, "\tfloat {} = {}.x;", out_varnames["x"], args["vec"]);
        let _ = writeln!(out, "\tfloat {} = {}.y;", out_varnames["y"], args["vec"]);
        let _ = writeln!(out, "\tfloat {} = {}.z;", out_varnames["z"], args["vec"]);
    }
}

pub struct VectorAdd {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for VectorAdd {
    const LABEL: &'static str = "Vector Add";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} + {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} + {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}


pub struct VectorSubtract {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for VectorSubtract {
    const LABEL: &'static str = "Vector Subtract";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} - {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} - {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct VectorMultiply {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for VectorMultiply {
    const LABEL: &'static str = "Vector Multiply";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} * {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} * {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct VectorScale {
    vec: NodeInput,
    scl: NodeInput
}

impl NodeType for VectorScale {
    const LABEL: &'static str = "Vector Scale";

    fn make() -> Self {
        Self {
            vec: Value::vector(0.0, 0.0, 0.0).into(),
            scl: Value::scalar(1.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("vec", Type::Vector, &self.vec),
            ("scl", Type::Scalar, &self.scl)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("vec", Type::Vector, &mut self.vec),
            ("scl", Type::Scalar, &mut self.scl)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} * {};\n", out_varnames["out"], args["vec"], args["scl"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} * {};\n", out_varnames["out"], args["vec"], args["scl"]).as_str());
    }

}

pub struct DotProduct {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for DotProduct {
    const LABEL: &'static str = "Dot Product";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("dot", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = dot({}, {});\n", out_varnames["dot"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = dot({}, {});\n", out_varnames["dot"], args["a"], args["b"]).as_str());
    }

}

pub struct CrossProduct {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for CrossProduct {
    const LABEL: &'static str = "Cross Product";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("cross", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = cross({}, {});\n", out_varnames["cross"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = cross({}, {});\n", out_varnames["cross"], args["a"], args["b"]).as_str());
    }

}

pub struct Length {
    vec: NodeInput,
}

impl NodeType for Length {
    const LABEL: &'static str = "Length";

    fn make() -> Self {
        Self {
            vec: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("vec", Type::Vector, &self.vec),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("vec", Type::Vector, &mut self.vec),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("length", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = length({});\n", out_varnames["length"], args["vec"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = length({});\n", out_varnames["length"], args["vec"]).as_str());
    }

}

pub struct Distance {
    a: NodeInput,
    b: NodeInput,
}

impl NodeType for Distance {
    const LABEL: &'static str = "Distance";

    fn make() -> Self {
        Self {
            a: Value::vector(0.0, 0.0, 0.0).into(),
            b: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Vector, &self.a),
            ("b", Type::Vector, &self.b),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Vector, &mut self.a),
            ("b", Type::Vector, &mut self.b),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("distance", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = distance({}, {});\n", out_varnames["distance"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = distance({}, {});\n", out_varnames["distance"], args["a"], args["b"]).as_str());
    }

}

pub struct Normalize {
    vec: NodeInput,
}

impl NodeType for Normalize {
    const LABEL: &'static str = "Normalize";

    fn make() -> Self {
        Self {
            vec: Value::vector(0.0, 0.0, 0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("vec", Type::Vector, &self.vec),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("vec", Type::Vector, &mut self.vec),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("vec", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = normalize({});\n", out_varnames["vec"], args["vec"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat3 {} = normalize({});\n", out_varnames["vec"], args["vec"]).as_str());
    }

}

pub struct Position {

}

impl NodeType for Position {
    const LABEL: &'static str = "Position";

    fn make() -> Self {
        Self {

        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("position", Type::Vector)
        ]
    }

    fn compile_wgsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = pos;\n", out_varnames["position"]).as_str());
    }

    fn compile_hlsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat3 {} = pos;\n", out_varnames["position"]).as_str());
    }

}

