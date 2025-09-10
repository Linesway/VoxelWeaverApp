
use std::{collections::HashSet, sync::{Arc, Mutex}};

use eframe::wgpu::{self, util::DeviceExt};

use super::{Terrain, TerrainChunk, TerrainVertex, CHUNK_SIZE};

mod tri_table;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    begin: glam::Vec4
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TerrainMeshgenData {
    curr_idx: u32
}

pub struct TerrainMeshGenerator {
    uniform_buffer: wgpu::Buffer, 
    uniform_bind_group: wgpu::BindGroup,

    tricount_pipeline: wgpu::ComputePipeline,
    tricount_layout: wgpu::PipelineLayout,
    tricount_bind_group: wgpu::BindGroup,
    tricount_buffer: wgpu::Buffer,
    tricount_read_buffer: wgpu::Buffer,

    meshgen_pipeline: wgpu::ComputePipeline,
    meshgen_layout: wgpu::PipelineLayout,
    meshgen_bind_group_layout: wgpu::BindGroupLayout,
    meshgen_data_buffer: wgpu::Buffer,
    meshgen_tri_table_buffer: wgpu::Buffer,

    generation_priority: Vec<glam::I64Vec3>
}

impl TerrainMeshGenerator {

    fn make_tricount_shader(sdf_code: &str) -> String {
        include_str!("meshgen/tricount.wgsl").to_string() + sdf_code
    }
    
    fn make_meshgen_shader(sdf_code: &str) -> String {
        include_str!("meshgen/meshgen.wgsl").to_string() + sdf_code
    }

    pub fn new(device: &wgpu::Device, sdf_code: &str) -> Self {

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("terrain_uniform_buffer"),
                contents: bytemuck::cast_slice(&[Uniforms { begin: glam::Vec4::ZERO }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("terrain_uniform_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    }
                ]
            }
        );

        let uniform_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("terrain_bind_group"),
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }
                ],
            }
        );

        let tricount_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("terrain_tricount_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    }
                ] 
            }
        );

        let tricount_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("terrain_tricount_buffer"),
                size: std::mem::size_of::<u32>() as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );

        let tricount_read_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("terrain_tricount_read_buffer"),
                size: std::mem::size_of::<u32>() as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }
        );

        let tricount_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("terrain_tricount_bind_group"),
                layout: &tricount_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: tricount_buffer.as_entire_binding(),
                    }
                ]
            }
        );

        let tricount_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("terrain_tricount_pipeline_layout"),
                bind_group_layouts: &[&tricount_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[]
            }
        );

        let tricount_shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("terrain_tricount_shader"),
                source: wgpu::ShaderSource::Wgsl(Self::make_tricount_shader(sdf_code).into()),
            }
        );

        let tricount_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("terrain_tricount_pipeline"),
                layout: Some(&tricount_layout),
                module: &tricount_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }
        );

        let meshgen_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("terrain_meshgen_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    }
                ]
            }
        );

        let meshgen_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("terrain_meshgen_pipeline_layout"),
                bind_group_layouts: &[&meshgen_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[] 
            }
        );

        let meshgen_shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("terrain_meshgen_shader"),
                source: wgpu::ShaderSource::Wgsl(Self::make_meshgen_shader(sdf_code).into())
            }
        );

        let meshgen_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("terrain_meshgen"),
                layout: Some(&meshgen_layout),
                module: &meshgen_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }
        );

        let meshgen_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("terrain_meshgen_uniforms"),
                contents: bytemuck::cast_slice(&[TerrainMeshgenData::default()]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        let meshgen_tri_table_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("terrain_meshgen_tri_table_buffer"),
                contents: bytemuck::cast_slice(&tri_table::TRI_TABLE),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            uniform_buffer,
            uniform_bind_group,

            tricount_pipeline,
            tricount_layout,
            tricount_bind_group,
            tricount_buffer,
            tricount_read_buffer,

            meshgen_pipeline,
            meshgen_layout,
            meshgen_bind_group_layout,
            meshgen_data_buffer: meshgen_uniform_buffer,
            meshgen_tri_table_buffer,

            generation_priority: Vec::new()
        }

    }

    fn generate_chunk(&self, device: &wgpu::Device, queue: &wgpu::Queue, begin: glam::Vec3, tri_counter: Arc<Mutex<u64>>) -> TerrainChunk {

        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("terrain_meshgen_encoder"),
            }
        );
        
        let mut compute_pass = encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor {
                label: Some("terrain_tricount"),
                timestamp_writes: None 
            }
        );

        // Set up shared uniforms
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[
            Uniforms {
                begin: glam::vec4(begin.x, begin.y, begin.z, 0.0)
            }
        ])); 

        // Count the triangles in the chunk's mesh
        queue.write_buffer(&self.tricount_buffer, 0, bytemuck::cast_slice(&[0u32]));
        
        compute_pass.set_pipeline(&self.tricount_pipeline);
        compute_pass.set_bind_group(0, &self.tricount_bind_group, &[]);
        compute_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
        compute_pass.dispatch_workgroups(CHUNK_SIZE / 16, CHUNK_SIZE / 16, CHUNK_SIZE);

        drop(compute_pass);

        encoder.copy_buffer_to_buffer(&self.tricount_buffer, 0, &self.tricount_read_buffer, 0, std::mem::size_of::<u32>() as u64);

        queue.submit([encoder.finish()]);

        self.tricount_read_buffer.slice(..).map_async(wgpu::MapMode::Read, move |_| {});

        device.poll(wgpu::MaintainBase::Wait);

        let tricount = {
            let bytes = self.tricount_read_buffer.slice(..).get_mapped_range();
            u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        };

        self.tricount_read_buffer.unmap();

        // If we have no triangles, we don't need a mesh
        if tricount == 0 {
            return TerrainChunk::new(None, 0, tri_counter);
        }

        // Generate the mesh
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("terrain_meshgen_encoder"),
            }
        );

        let mesh = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("terrain_chunk_mesh"),
                size:  (tricount as u64) * 3 * (std::mem::size_of::<TerrainVertex>() as u64), 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }
        );

        queue.write_buffer(&self.meshgen_data_buffer, 0, bytemuck::cast_slice(&[
            TerrainMeshgenData {
                curr_idx: 0,
            }
        ]));
        
        let meshgen_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("terrain_meshgen_bind_group"),
                layout: &self.meshgen_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: mesh.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.meshgen_data_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.meshgen_tri_table_buffer.as_entire_binding()
                    }
                ],
            }
        );

        {
            let mut compute_pass = encoder.begin_compute_pass(
                &wgpu::ComputePassDescriptor {
                    label: Some("terrain_meshgen"),
                    timestamp_writes: None 
                }
            );

            compute_pass.set_pipeline(&self.meshgen_pipeline);
            compute_pass.set_bind_group(0, &meshgen_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            compute_pass.dispatch_workgroups(CHUNK_SIZE / 16, CHUNK_SIZE / 16, CHUNK_SIZE);
        }

        queue.submit([encoder.finish()]);

        TerrainChunk::new(Some(mesh), tricount, tri_counter) 

    }

    // returns whether there are more chunks to generate
    pub fn generate(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, terrain: &mut Terrain, center: glam::Vec3, size: f32, tri_counter: &Arc<Mutex<u64>>) -> bool {
        let min = center - glam::Vec3::splat(size / 2.0);
        let max = center + glam::Vec3::splat(size / 2.0);

        let min_int = (min / (CHUNK_SIZE as f32)).floor();
        let min_int = glam::I64Vec3::new(min_int.x as i64, min_int.y as i64, min_int.z as i64);
        let max_int = (max / (CHUNK_SIZE as f32)).ceil();
        let max_int = glam::I64Vec3::new(max_int.x as i64, max_int.y as i64, max_int.z as i64);

        let mut chunks_needed = HashSet::new();
        for x in min_int.x..max_int.x {
            for y in min_int.y..max_int.y {
                for z in min_int.z..max_int.z {
                    let loc = glam::i64vec3(x, y, z);
                    chunks_needed.insert(loc);
                }
            }
        }

        terrain.chunks.retain(|loc, _| chunks_needed.contains(loc));
        self.generation_priority.retain(|loc| chunks_needed.contains(loc) && !terrain.chunks.contains_key(loc));

        // We'll generate chunks closer to the center first
        let mut chunks_to_generate: Vec<glam::I64Vec3> = chunks_needed.clone().into_iter().collect();
        let center_int = (min_int + max_int) / 2;
        let chunk_sorter = |a: &glam::I64Vec3, b: &glam::I64Vec3| b.distance_squared(center_int).cmp(&a.distance_squared(center_int));
        chunks_to_generate.sort_by(chunk_sorter); 
        self.generation_priority.sort_by(chunk_sorter);
        
        // Generate a fixed number of chunks per frame 
        let mut chunks_per_frame = 1;
        while chunks_per_frame > 0 {
            let loc = if let Some(loc) = self.generation_priority.pop() {
                loc
            } else if let Some(loc) = chunks_to_generate.pop() {
                loc
            } else {
                break;
            };
            if terrain.chunks.get(&loc).is_some() {
                continue;
            }
            chunks_per_frame -= 1;
            let chunk_begin = glam::vec3((loc.x * (CHUNK_SIZE as i64)) as f32, (loc.y * (CHUNK_SIZE as i64)) as f32, (loc.z * (CHUNK_SIZE as i64)) as f32);
            let chunk = self.generate_chunk(
                device,
                queue,
                chunk_begin,
                tri_counter.clone()
            );
            if chunk.mesh.is_some() {
                // chunk contains some triangles, therefore nearby chunks are likely to contain more.
                // prioritize them.
                for axis in glam::I64Vec3::AXES {
                    self.generation_priority.push(loc + axis);
                    self.generation_priority.push(loc - axis);
                }
            }
            terrain.chunks.insert(loc, chunk);
        }

        return !chunks_to_generate.is_empty();
    }

    pub fn update_shaders(&mut self, device: &wgpu::Device, sdf_code: &str) {

        let tricount_shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("terrain_tricount_shader"),
                source: wgpu::ShaderSource::Wgsl(Self::make_tricount_shader(sdf_code).into()),
            }
        );

        let tricount_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("terrain_tricount_pipeline"),
                layout: Some(&self.tricount_layout),
                module: &tricount_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }
        );

        self.tricount_pipeline = tricount_pipeline;

        let meshgen_shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("terrain_meshgen_shader"),
                source: wgpu::ShaderSource::Wgsl(Self::make_meshgen_shader(sdf_code).into())
            }
        );

        let meshgen_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("terrain_meshgen"),
                layout: Some(&self.meshgen_layout),
                module: &meshgen_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }
        );

        self.meshgen_pipeline = meshgen_pipeline;
    }

}
