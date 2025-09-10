use core::f32;
use std::collections::HashMap;

use crate::graph::{GraphProjectInfo, NodeInput, NodeType, Type, Value};


pub struct Add {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Add {
    const LABEL: &'static str = "Add";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} + {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} + {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Subtract {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Subtract {
    const LABEL: &'static str = "Subtract";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} - {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} - {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Multiply {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Multiply {
    const LABEL: &'static str = "Multiply";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} * {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} * {};\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Divide {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Divide {
    const LABEL: &'static str = "Divide";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = {} / dezero({});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} / dezero({});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Power {
    base: NodeInput,
    exp: NodeInput
}

impl NodeType for Power {
    const LABEL: &'static str = "Power";

    fn make() -> Self {
        Self {
            base: Value::scalar(0.0).into(),
            exp: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("base", Type::Scalar, &self.base),
            ("exp", Type::Scalar, &self.exp)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("base", Type::Scalar, &mut self.base),
            ("exp", Type::Scalar, &mut self.exp)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = pow({}, {});\n", out_varnames["out"], args["base"], args["exp"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = pow({}, {});\n", out_varnames["out"], args["base"], args["exp"]).as_str());
    }

}

pub struct Log {
    base: NodeInput,
    x: NodeInput
}

impl NodeType for Log {
    const LABEL: &'static str = "Log";

    fn make() -> Self {
        Self {
            base: Value::scalar(f32::consts::E).into(),
            x: Value::scalar(1.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("base", Type::Scalar, &self.base),
            ("x", Type::Scalar, &self.x)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("base", Type::Scalar, &mut self.base),
            ("x", Type::Scalar, &mut self.x)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = log({}) / log(dezero({}));\n", out_varnames["out"], args["x"], args["base"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = log({}) / log(dezero({}));\n", out_varnames["out"], args["x"], args["base"]).as_str());
    }

}

pub struct Min {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Min {
    const LABEL: &'static str = "Min";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = min({}, {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = min({}, {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Max {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for Max {
    const LABEL: &'static str = "Max";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = max({}, {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = max({}, {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct GreaterThan {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for GreaterThan {
    const LABEL: &'static str = "Greater Than";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = select(0.0, 1.0, {} > {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} > {} ? 1.0 : 0.0;\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct LessThan {
    a: NodeInput,
    b: NodeInput
}

impl NodeType for LessThan {
    const LABEL: &'static str = "Less Than";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = select(0.0, 1.0, {} < {});\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = {} < {} ? 1.0 : 0.0;\n", out_varnames["c"], args["a"], args["b"]).as_str());
    }

}

pub struct Equal {
    a: NodeInput,
    b: NodeInput,
    eps: NodeInput,
}

impl NodeType for Equal {
    const LABEL: &'static str = "Equal";

    fn make() -> Self {
        Self {
            a: Value::scalar(0.0).into(),
            b: Value::scalar(0.0).into(),
            eps: Value::scalar(0.05).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("a", Type::Scalar, &self.a),
            ("b", Type::Scalar, &self.b),
            ("eps", Type::Scalar, &self.eps)
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("a", Type::Scalar, &mut self.a),
            ("b", Type::Scalar, &mut self.b),
            ("eps", Type::Scalar, &mut self.eps)
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("c", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = select(0.0, 1.0, abs({} - {}) < {});\n", out_varnames["c"], args["a"], args["b"], args["eps"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = (abs({} - {}) < {}) ? 1.0 : 0.0;\n", out_varnames["c"], args["a"], args["b"], args["eps"]).as_str());
    }

}

pub struct Floor {
    x: NodeInput,
}

impl NodeType for Floor {
    const LABEL: &'static str = "Floor";

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
        out.push_str(format!("\tlet {} = floor({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = floor({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Round {
    x: NodeInput,
}

impl NodeType for Round {
    const LABEL: &'static str = "Round";

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
        out.push_str(format!("\tlet {} = round({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = round({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Ceil {
    x: NodeInput,
}

impl NodeType for Ceil {
    const LABEL: &'static str = "Ceil";

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
        out.push_str(format!("\tlet {} = ceil({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = ceil({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Frac {
    x: NodeInput,
}

impl NodeType for Frac {
    const LABEL: &'static str = "Frac";

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
        out.push_str(format!("\tlet {} = fract({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = frac({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Abs {
    x: NodeInput,
}

impl NodeType for Abs {
    const LABEL: &'static str = "Abs";

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
        out.push_str(format!("\tlet {} = abs({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = abs({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Sign {
    x: NodeInput,
}

impl NodeType for Sign {
    const LABEL: &'static str = "Sign";

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
        out.push_str(format!("\tlet {} = sign({});\n", out_varnames["out"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = sign({});\n", out_varnames["out"], args["x"]).as_str());
    }

}

pub struct Clamp {
    x: NodeInput,
    min: NodeInput,
    max: NodeInput,
}

impl NodeType for Clamp {
    const LABEL: &'static str = "Clamp";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
            min: Value::scalar(0.0).into(),
            max: Value::scalar(1.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
            ("min", Type::Scalar, &self.min),
            ("max", Type::Scalar, &self.max),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
            ("min", Type::Scalar, &mut self.min),
            ("max", Type::Scalar, &mut self.max),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = clamp({}, {}, {});\n", out_varnames["out"], args["x"], args["min"], args["max"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = clamp({}, {}, {});\n", out_varnames["out"], args["x"], args["min"], args["max"]).as_str());
    }

}

pub struct Lerp {
    x: NodeInput,
    min: NodeInput,
    max: NodeInput,
}

impl NodeType for Lerp {
    const LABEL: &'static str = "Lerp";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
            min: Value::scalar(0.0).into(),
            max: Value::scalar(1.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
            ("min", Type::Scalar, &self.min),
            ("max", Type::Scalar, &self.max),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
            ("min", Type::Scalar, &mut self.min),
            ("max", Type::Scalar, &mut self.max),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {} = mix({}, {}, {});\n", out_varnames["out"], args["min"], args["max"], args["x"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {} = lerp({}, {}, {});\n", out_varnames["out"], args["min"], args["max"], args["x"]).as_str());
    }

}

pub struct MapRange {
    x: NodeInput,
    from_min: NodeInput,
    from_max: NodeInput,
    to_min: NodeInput,
    to_max: NodeInput,
}

impl NodeType for MapRange {
    const LABEL: &'static str = "Map Range";

    fn make() -> Self {
        Self {
            x: Value::scalar(0.0).into(),
            from_min: Value::scalar(0.0).into(),
            from_max: Value::scalar(1.0).into(),
            to_min: Value::scalar(0.0).into(),
            to_max: Value::scalar(1.0).into(),
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![
            ("x", Type::Scalar, &self.x),
            ("from min", Type::Scalar, &self.from_min),
            ("from max", Type::Scalar, &self.from_max),
            ("to min", Type::Scalar, &self.from_min),
            ("to max", Type::Scalar, &self.from_max),
        ]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![
            ("x", Type::Scalar, &mut self.x),
            ("from min", Type::Scalar, &mut self.from_min),
            ("from max", Type::Scalar, &mut self.from_max),
            ("to min", Type::Scalar, &mut self.to_min),
            ("to max", Type::Scalar, &mut self.to_max),
        ]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("out", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tlet {0} = {2} + ({1} - {2}) * ({5} - {4}) / ({3} - {2});\n", out_varnames["out"], args["x"], args["from min"], args["from max"], args["to min"], args["to max"]).as_str());
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, _info: &GraphProjectInfo) {
        out.push_str(format!("\tfloat {0} = {2} + ({1} - {2}) * ({5} - {4}) / ({3} - {2});\n", out_varnames["out"], args["x"], args["from min"], args["from max"], args["to min"], args["to max"]).as_str());
    }

}
