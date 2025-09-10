
use std::{collections::HashMap, sync::{Arc, Mutex}};

use glam::Vec3;
use eframe::egui_wgpu::wgpu;

pub mod renderer;
pub mod meshgen;
pub mod texture_atlas;
pub mod biome_preview;

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TerrainVertex {
    pos: Vec3,
    norm: Vec3,
    mat1: u32,
    mat2: u32,
    mat_weight: f32
}

impl TerrainVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Uint32, 3 => Uint32, 4 => Float32];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

}

pub const CHUNK_SIZE: u32 = 64;

struct TerrainChunk {
    mesh: Option<wgpu::Buffer>,
    tris: u32,
    tri_counter: Arc<Mutex<u64>>
}

impl TerrainChunk {

    pub(crate) fn new(mesh: Option<wgpu::Buffer>, tris: u32, tri_counter: Arc<Mutex<u64>>) -> Self {
        *tri_counter.lock().unwrap() += tris as u64;
        Self {
            mesh,
            tris,
            tri_counter,
        }
    }
    
}

impl Drop for TerrainChunk {

    fn drop(&mut self) {
        *self.tri_counter.lock().unwrap() -= self.tris as u64;
    }

}

pub struct Terrain {
    chunks: HashMap<glam::I64Vec3, TerrainChunk>
}

impl Terrain {

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

}
