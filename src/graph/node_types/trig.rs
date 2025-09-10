use std::collections::HashMap;

use crate::graph::{GraphProjectInfo, NodeInput, NodeType, Type, Value};


pub struct Sin {
    x: NodeInput,
}

impl NodeType for Sin {
    const LABEL: &'static str = "Sin";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = sin({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = sin({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Cos {
    x: NodeInput,
}

impl NodeType for Cos {
    const LABEL: &'static str = "Cos";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = cos({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = cos({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Tan {
    x: NodeInput,
}

impl NodeType for Tan {
    const LABEL: &'static str = "Tan";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = tan({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = tan({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Asin {
    x: NodeInput,
}

impl NodeType for Asin {
    const LABEL: &'static str = "Asin";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = asin({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = asin({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Acos {
    x: NodeInput,
}

impl NodeType for Acos {
    const LABEL: &'static str = "Acos";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = acos({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = acos({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Atan {
    x: NodeInput,
}

impl NodeType for Atan {
    const LABEL: &'static str = "Atan";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = atan({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = atan({});\n", out_varnames["out"], args["x"]).as_str());
    }

}
