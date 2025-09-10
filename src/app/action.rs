
use std::path::PathBuf;

use eframe::wgpu;

use crate::{biome::Biome, graph::{Connection, NodeId, NodeTypeDyn, Value}, project::Project};

pub enum Action {
    GraphMoveNode(NodeId, egui::Vec2),
    GraphAddNode(NodeId, egui::Pos2, Box<dyn NodeTypeDyn>),
    GraphDeleteNode(NodeId),
    GraphConnect(Connection),
    GraphDisconnect {
        to_node: NodeId,
        in_idx: u32
    },
    GraphSetInput {
        node: NodeId,
        in_idx: u32,
        val: Value
    },

    BiomeCreate(usize, Biome),
    BiomeDelete(usize),
    BiomeSetSize(f32),
    BiomeSetBlending(f32),
    BiomeSetName(usize, String),
    BiomeSetFrequency(usize, f32),
    BiomeSetTexture(usize, PathBuf),
    BiomeSetMinDepth(usize, i32),
    BiomeSetMaxDepth(usize, i32),
    BiomeSetParameter {
        biome: usize,
        param: String,
        val: f32
    },
    BiomeCreateParameter {
        param: String
    },
    BiomeDeleteParameter {
        param: String
    },

    Compound(Vec<Action>)
}

impl Action {

    fn perform(self, project: &mut Project, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        match self {
            Action::GraphMoveNode(node_id, offset) => {
                project.terrain_graph.nodes.get_mut(&node_id).unwrap().pos += offset;
                Action::GraphMoveNode(node_id, -offset)
            },
            Action::GraphAddNode(node_id, pos, ty) => {
                project.terrain_graph.add_node_from_box_ty_with_id(pos, ty, node_id);
                Action::GraphDeleteNode(node_id)
            },
            Action::GraphDeleteNode(node_id) => {
                let node = project.terrain_graph.delete_node(node_id).1.unwrap();
                Action::GraphAddNode(node_id, node.pos, node.ty)
            },
            Action::GraphConnect(connection) => {
                project.terrain_graph.connect(connection);
                Action::GraphDisconnect { to_node: connection.to, in_idx: connection.inp_idx }
            },
            Action::GraphDisconnect { to_node, in_idx } => {
                let node = project.terrain_graph.nodes.get_mut(&to_node).unwrap();
                let inp = &mut node.ty.inputs_mut()[in_idx as usize].2;
                let (from, out_idx) = inp.connection.unwrap();
                inp.connection = None;
                Action::GraphConnect(Connection{
                    from,
                    out_idx,
                    to: to_node,
                    inp_idx: in_idx
                })
            },
            Action::GraphSetInput { node: node_id, in_idx, val } => {
                let node = project.terrain_graph.nodes.get_mut(&node_id).unwrap();
                let inp = &mut node.ty.inputs_mut()[in_idx as usize].2;
                let old_val = std::mem::replace(&mut inp.val, val);
                Action::GraphSetInput { node: node_id, in_idx, val: old_val }
            },

            Action::BiomeCreate(idx, biome) => {
                project.biomes.biomes.insert(idx, biome);
                Action::BiomeDelete(idx)
            },
            Action::BiomeDelete(idx) => {
                let biome = project.biomes.biomes.remove(idx);
                Action::BiomeCreate(idx, biome)
            },
            Action::BiomeSetSize(size) => {
                let old_size = project.biomes.biome_size;
                project.biomes.biome_size = size;
                Action::BiomeSetSize(old_size)
            },
            Action::BiomeSetBlending(blending) => {
                let old_blending = project.biomes.biome_blending;
                project.biomes.biome_blending = blending;
                Action::BiomeSetBlending(old_blending)
            },
            Action::BiomeSetName(idx, name) => {
                let old_name = std::mem::replace(&mut project.biomes.biomes[idx].name, name);
                Action::BiomeSetName(idx, old_name)
            },
            Action::BiomeSetFrequency(idx, freq) => {
                let old_freq = project.biomes.biomes[idx].frequency;
                project.biomes.biomes[idx].frequency = freq;
                Action::BiomeSetFrequency(idx, old_freq)
            },
            Action::BiomeSetTexture(idx, texture) => {
                let old_texture = std::mem::replace(&mut project.biomes.biomes[idx].texture, texture);
                Action::BiomeSetTexture(idx, old_texture)
            },
            Action::BiomeSetMinDepth(idx, min_depth) => {
                let old_min_depth = project.biomes.biomes[idx].min_depth;
                project.biomes.biomes[idx].min_depth = min_depth;
                Action::BiomeSetMinDepth(idx, old_min_depth)
            },
            Action::BiomeSetMaxDepth(idx, max_depth) => {
                let old_max_depth = project.biomes.biomes[idx].max_depth;
                project.biomes.biomes[idx].max_depth = max_depth;
                Action::BiomeSetMaxDepth(idx, old_max_depth)
            },
            Action::BiomeSetParameter { biome, param, val } => {
                let old_val = *project.biomes.biomes[biome].params.get(&param).unwrap_or(&0.0);
                project.biomes.biomes[biome].params.insert(param.clone(), val);
                Action::BiomeSetParameter { biome, param, val: old_val }
            },
            Action::BiomeCreateParameter { param } => {
                project.biomes.biome_params.push(param.clone());
                Action::BiomeDeleteParameter { param }
            },
            Action::BiomeDeleteParameter { param } => {
                if let Some(idx) = project.biomes.biome_params.iter().position(|other| other == &param) {
                    project.biomes.biome_params.remove(idx);
                }
                Action::BiomeCreateParameter { param }
            }
            Action::Compound(acts) => {
                let mut inv = Vec::new();
                for act in acts {
                    inv.push(act.perform(project, device, queue));
                }
                inv.reverse();
                Action::Compound(inv)
            }
        }
    }

}

pub struct ActionManager {
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>
}

impl ActionManager {

    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new()
        }
    }

    pub fn push_undo_action(&mut self, act: Action) {
        self.redo_stack.clear();
        self.undo_stack.push(act);
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo(&mut self, project: &mut Project, device: &wgpu::Device, queue: &wgpu::Queue) {
        let Some(act) = self.undo_stack.pop() else { return; };
        self.redo_stack.push(act.perform(project, device, queue));
    }

    pub fn redo(&mut self, project: &mut Project, device: &wgpu::Device, queue: &wgpu::Queue) {
        let Some(act) = self.redo_stack.pop() else { return; };
        self.undo_stack.push(act.perform(project, device, queue));
    }

}
