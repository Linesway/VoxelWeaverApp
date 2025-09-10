
pub mod node_types;

use std::collections::{HashMap, HashSet};

use node_types::NODE_TYPES;
use serde_json::json;

use crate::{app::action::{Action, ActionManager}, biome::Biomes};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodeId(u64);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Scalar(f32),
    Vector(glam::Vec3),
    Terrain
}

impl Value {

    pub fn scalar(val: f32) -> Self {
        Value::Scalar(val)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Value::Vector(glam::vec3(x, y, z))
    }

    pub fn terrain() -> Self {
        Self::Terrain
    }

}

#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Type {
    Scalar,
    Vector,
    Terrain
}

pub struct NodeInput {
    pub val: Value,
    pub connection: Option<(NodeId, u32)>
}

impl From<Value> for NodeInput {

    fn from(val: Value) -> Self {
        Self {
            val,
            connection: None
        }
    }

}

pub struct GraphProjectInfo<'a> {
    pub biomes: &'a Biomes
} 

pub trait NodeType {

    const LABEL: &'static str;

    fn make() -> Self;
    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)>;
    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)>;
    fn outputs() -> Vec<(&'static str, Type)>;
    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo);
    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo);

    fn custom_ui_height() -> f32 {
        return 0.0;
    }

    fn custom_ui(&mut self, _ui: &mut egui::Ui, _info: &GraphProjectInfo) {

    }

    fn custom_serialize(&self) -> serde_json::Value {
        serde_json::Value::Null
    } 

    fn custom_deserialize(&mut self, _data: &serde_json::Value) {

    }

}

pub trait NodeTypeDyn {

    fn label(&self) -> &'static str;
    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)>;
    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)>;
    fn outputs(&self) -> Vec<(&'static str, Type)>;
    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo);
    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo);
    fn custom_ui_height(&self) -> f32;
    fn custom_ui(&mut self, ui: &mut egui::Ui, info: &GraphProjectInfo);
    fn custom_serialize(&self) -> serde_json::Value;
    fn custom_deserialize(&mut self, data: &serde_json::Value);

}

impl<T: NodeType> NodeTypeDyn for T {

    fn label(&self) -> &'static str {
        Self::LABEL
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        self.inputs()
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        self.inputs_mut()
    }

    fn outputs(&self) -> Vec<(&'static str, Type)> {
        Self::outputs()
    }

    fn compile_wgsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        self.compile_wgsl(args, out_varnames, out, info);
    }

    fn compile_hlsl(&self, args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        self.compile_hlsl(args, out_varnames, out, info);
    }

    fn custom_ui_height(&self) -> f32 {
        Self::custom_ui_height()
    }

    fn custom_ui(&mut self, ui: &mut egui::Ui, info: &GraphProjectInfo) {
        self.custom_ui(ui, info);
    }

    fn custom_serialize(&self) -> serde_json::Value {
        self.custom_serialize()
    }

    fn custom_deserialize(&mut self, data: &serde_json::Value) {
        self.custom_deserialize(data); 
    }

}

pub struct Node {
    pub pos: egui::Pos2,
    pub ty: Box<dyn NodeTypeDyn>
}

pub struct TerrainGraph {
    pub nodes: HashMap<NodeId, Node>,
    pub curr_node_id: NodeId,
    pub curr_drag_delta: egui::Vec2,

    // Editor data
    pub transform: egui::emath::TSTransform,
    pub editor_searching: bool,
    pub editor_search_query: String, 
}

#[derive(Clone, Copy)]
pub struct Connection {
    pub from: NodeId,
    pub out_idx: u32,
    pub to: NodeId,
    pub inp_idx: u32
}

impl TerrainGraph {

    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            curr_node_id: NodeId(1),
            curr_drag_delta: egui::Vec2::ZERO,
            transform: egui::emath::TSTransform::IDENTITY,
            editor_searching: false,
            editor_search_query: String::new(),
        }
    }

    pub fn add_node<T: NodeType + 'static>(&mut self, pos: egui::Pos2, ty: T) -> NodeId {
        self.add_node_from_box_ty(pos, Box::new(ty))
    }

    pub fn add_node_from_box_ty(&mut self, pos: egui::Pos2, ty: Box<dyn NodeTypeDyn>) -> NodeId {
        self.curr_node_id.0 += 1;
        let id = NodeId(self.curr_node_id.0 - 1);
        self.add_node_from_box_ty_with_id(pos, ty, id);
        id
    }
    
    pub fn add_node_from_box_ty_with_id(&mut self, pos: egui::Pos2, ty: Box<dyn NodeTypeDyn>, id: NodeId) {
        self.nodes.insert(id, Node {
            pos,
            ty 
        });
    }

    pub fn add_node_from_box_ty_with_action(&mut self, pos: egui::Pos2, ty: Box<dyn NodeTypeDyn>, actions: &mut ActionManager) {
        let node = self.add_node_from_box_ty(pos, ty);
        actions.push_undo_action(Action::GraphDeleteNode(node));
    }

    pub fn delete_node(&mut self, id: NodeId) -> (Vec<Connection>, Option<Node>) {
        let mut broken_connections = Vec::new();
        for (other_node_id, node) in &mut self.nodes {
            for (inp_idx, (_, _, inp)) in node.ty.inputs_mut().into_iter().enumerate() {
                if let Some((dependency_node_id, out_idx)) = inp.connection {
                    if dependency_node_id == id {
                        broken_connections.push(Connection {
                            from: id,
                            out_idx,
                            to: *other_node_id,
                            inp_idx: inp_idx as u32
                        });
                        inp.connection = None;
                    }
                }
            }
        }
        (broken_connections, self.nodes.remove(&id))
    }

    pub fn delete_node_with_action(&mut self, id: NodeId, actions: &mut ActionManager) {
        let (broken_connections, node) = self.delete_node(id);
        let Some(node) = node else { return; };
        let mut acts = vec![
            Action::GraphAddNode(id, node.pos, node.ty)
        ];
        for connection in broken_connections {
            acts.push(Action::GraphConnect(connection));
        }
        actions.push_undo_action(Action::Compound(acts));
    }

    pub fn connect(&mut self, conn: Connection) -> (bool, Option<Connection>) {
        let Some(to_node) = self.nodes.get_mut(&conn.to) else { return (false, None); };
        let inp = &mut to_node.ty.inputs_mut()[conn.inp_idx as usize].2;
        let broken_connection = inp.connection.map(|(old_from, old_out_idx)| Connection { from: old_from, out_idx: old_out_idx, to: conn.to, inp_idx: conn.inp_idx });
        inp.connection = Some((conn.from, conn.out_idx));

        if graph_toposort(&self).is_none() {
            // New connection creates a cycle, undo.
            let to_node = self.nodes.get_mut(&conn.to).unwrap();
            let inp = &mut to_node.ty.inputs_mut()[conn.inp_idx as usize].2;
            inp.connection = None;
            if let Some(broken_connection) = broken_connection {
                self.connect(broken_connection);
            }
            return (false, None);
        } 

        (true, broken_connection)
    }

    pub fn connect_with_action(&mut self, conn: Connection, actions: &mut ActionManager) {
        let disconnect_action = Action::GraphDisconnect { to_node: conn.to, in_idx: conn.inp_idx };
        let (connected, broken_connection) = self.connect(conn);
        if let Some(broken_connection) = broken_connection {
            if connected { 
                actions.push_undo_action(Action::Compound(vec![disconnect_action, Action::GraphConnect(broken_connection)]));
            }
        } else {
            if connected {
                actions.push_undo_action(disconnect_action);
            }
        }
    }

    pub fn disconnect(&mut self, node: NodeId, inp_idx: u32) -> Option<Connection> {
        let Some(to_node) = self.nodes.get_mut(&node) else { return None; };
        let inp = &mut to_node.ty.inputs_mut()[inp_idx as usize].2;
        let broken_connection = inp.connection.map(|(old_from, old_out_idx)| Connection { from: old_from, out_idx: old_out_idx, to: node, inp_idx: inp_idx });
        inp.connection = None;
        broken_connection
    }

    pub fn disconnect_with_action(&mut self, node: NodeId, inp_idx: u32, actions: &mut ActionManager) {
        if let Some(conn) = self.disconnect(node, inp_idx) {
            actions.push_undo_action(Action::GraphConnect(conn));
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "transform": self.transform,
            "curr_node_id": self.curr_node_id.0,
            "nodes": serde_json::Value::Array(
                self.nodes.iter().map(|(id, node)| json!({
                    "id": id.0,
                    "pos": node.pos,
                    "ty": node.ty.label(),
                    "data": node.ty.custom_serialize(),
                    "inputs": serde_json::Value::Array(node.ty.inputs().into_iter().map(|(_label, _ty, inp)| json!({
                        "val": inp.val,
                        "conn": inp.connection
                    })).collect())
                })).collect()
            ) 
        })
    }

    pub fn from_json(data: &serde_json::Value) -> Option<Self> {
        let data = data.as_object()?;
        Some(Self {
            nodes: data.get("nodes")?.as_array()?.iter().filter_map(|node_data| {
                let node_data = node_data.as_object()?;
                let id = NodeId(node_data.get("id")?.as_u64()?);

                let label = node_data.get("ty")?.as_str()?;
                let mut ty = None;
                'type_loop: for (_, node_types) in NODE_TYPES {
                    for node_type in *node_types {
                        if node_type.label == label {
                            let mut node = (node_type.make)(); 
                            let mut inputs = node.inputs_mut();
                            for (i, inp_data) in node_data.get("inputs")?.as_array()?.iter().enumerate() {
                                if i >= inputs.len() {
                                    break;
                                }
                                let inp_data = inp_data.as_object()?;
                                let val = serde_json::from_value(inp_data.get("val")?.clone()).ok()?;
                                let conn = serde_json::from_value(inp_data.get("conn")?.clone()).ok()?;
                                inputs[i].2.val = val;
                                inputs[i].2.connection = conn;
                            }
                            ty = Some(node);
                            break 'type_loop;
                        } 
                    }
                }

                let mut ty = ty?;
                ty.custom_deserialize(node_data.get("data").unwrap_or(&serde_json::Value::Null));
                Some((id, Node {
                    pos: serde_json::from_value(node_data.get("pos")?.clone()).ok()?,
                    ty,
                }))
            }).collect(),
            curr_node_id: NodeId(data.get("curr_node_id")?.as_u64()?),
            curr_drag_delta: egui::Vec2::ZERO,
            transform: serde_json::from_value(data.get("transform")?.clone()).ok()?,
            editor_searching: false,
            editor_search_query: String::new(),
        })
    }

}

pub fn graph_toposort(graph: &TerrainGraph) -> Option<Vec<NodeId>> {
    // uses wikipedia's DFS algo

    let mut temp_mark = HashSet::new();
    let mut perm_mark = HashSet::new();
    let mut sorted_nodes = Vec::new();
    let mut contains_cycle = false;

    fn visit(node: NodeId, perm_mark: &mut HashSet<NodeId>, temp_mark: &mut HashSet<NodeId>, contains_cycle: &mut bool, graph: &TerrainGraph, sorted_nodes: &mut Vec<NodeId>) {

        if perm_mark.contains(&node) {
            return;
        }
        if temp_mark.contains(&node) {
            *contains_cycle = true;
            return;
        }
        
        temp_mark.insert(node);

        for (_, _, input) in graph.nodes[&node].ty.inputs() {
            if let Some((dependency_node, _)) = &input.connection {
                visit(*dependency_node, perm_mark, temp_mark, contains_cycle, graph, sorted_nodes);
            }
        }

        sorted_nodes.push(node);
        perm_mark.insert(node);

    }

    for node in graph.nodes.keys() {
        if perm_mark.contains(node) {
            continue;
        }

        visit(*node, &mut perm_mark, &mut temp_mark, &mut contains_cycle, &graph, &mut sorted_nodes);
    }

    if contains_cycle {
        None
    } else {
        Some(sorted_nodes)
    }
}
