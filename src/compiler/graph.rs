
use std::collections::HashMap;

use crate::graph::{graph_toposort, GraphProjectInfo, NodeId, TerrainGraph};

use super::CompilationTarget;

pub fn compile_graph(out: &mut String, graph: &TerrainGraph, target: CompilationTarget, info: &GraphProjectInfo) {
    
    let Some(sorted_nodes) = graph_toposort(graph) else {
        return;
    };

    let mut output_names: HashMap<(NodeId, u32), String> = HashMap::new();
    let mut curr_output_idx = 0;
    for node_id in sorted_nodes {
        let node = &graph.nodes[&node_id];

        let mut args = HashMap::new();
        for (arg_name, _ty, inp) in node.ty.inputs() {
            let val = if let Some((out_node_id, out_idx)) = &inp.connection {
                output_names[&(*out_node_id, *out_idx)].clone()
            } else {
                inp.val.to_string(target)
            };
            
            args.insert(arg_name, val);
        }

        let mut out_varnames = HashMap::new();
        for (out_idx, (out_name, _output_id)) in node.ty.outputs().iter().enumerate() {
            let name = format!("val_{}", curr_output_idx);
            curr_output_idx += 1;
            out_varnames.insert(*out_name, name.clone());
            output_names.insert((node_id, out_idx as u32), name);
        }

        let ty = &graph.nodes[&node_id].ty;
        match target {
            CompilationTarget::WGSL => ty.compile_wgsl(args, out_varnames, out, info),
            CompilationTarget::UnrealHLSL => ty.compile_hlsl(args, out_varnames, out, info),
        }
    }

}
