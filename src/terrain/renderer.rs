
use eframe::wgpu::{self, util::DeviceExt};
use glam::{Mat4, Vec3};

use super::{texture_atlas::TextureAtlas, Terrain, TerrainVertex};

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TerrainRendererUniforms {
    trans: [[f32; 4]; 4],
    bounds_min: glam::Vec4,
    bounds_max: glam::Vec4,
}

pub struct TerrainRenderer {
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    renderer_bind_group: wgpu::BindGroup,
    atlas_bind_group_layout: wgpu::BindGroupLayout,
    atlas_bind_group: Option<wgpu::BindGroup>
}

impl TerrainRenderer {

    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("terrain_renderer_uniforms"),
                contents: bytemuck::cast_slice(&[TerrainRendererUniforms::default()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let renderer_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("terrain_renderer_uniform_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None 
                        },
                        count: None,
                    }
                ],
            }
        );

        let renderer_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("terain_renderer_bind_group"),
                layout: &renderer_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }
                ],
            }
        );

        let atlas_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("terrain_renderer_atlas_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false 
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }
                ] 
            }
        );

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("terrain_renderer_pipeline_layout"),
                bind_group_layouts: &[&renderer_bind_group_layout, &atlas_bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        let shader = device.create_shader_module(wgpu::include_wgsl!("renderer/shader.wgsl"));

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("terrain_renderer_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[TerrainVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target_format, 
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24Plus,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                ),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                },
                multiview: None,
            } 
        );

        Self {
            render_pipeline: pipeline,
            uniform_buffer,
            renderer_bind_group,
            atlas_bind_group_layout,
            atlas_bind_group: None
        }
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, cam: Mat4, bounds_min: Vec3, bounds_max: Vec3, atlas: &TextureAtlas) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[
            TerrainRendererUniforms {
                trans: cam.to_cols_array_2d(),
                bounds_min: glam::vec4(bounds_min.x, bounds_min.y, bounds_min.z, 0.0),
                bounds_max: glam::vec4(bounds_max.x, bounds_max.y, bounds_max.z, 0.0),
            }
        ]));

        self.atlas_bind_group = Some(device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("atlas_bind_group"),
                layout: &self.atlas_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas.atlas_view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&atlas.atlas_sampler)
                    }
                ] 
            }
        ));
    }

    pub fn render<'a>(&'a self, terrain: &'a Terrain, pass: &mut wgpu::RenderPass<'a>) {
        for (_loc, chunk) in &terrain.chunks {
            if let Some(mesh) = &chunk.mesh {
                pass.set_pipeline(&self.render_pipeline);
                pass.set_vertex_buffer(0, mesh.slice(..));
                pass.set_bind_group(0, &self.renderer_bind_group, &[]);
                if let Some(atlas_bind_group) = &self.atlas_bind_group {
                    pass.set_bind_group(1, atlas_bind_group, &[]);
                }
                pass.draw(0..(3 * chunk.tris), 0..1);
            }
        }
    }

}
