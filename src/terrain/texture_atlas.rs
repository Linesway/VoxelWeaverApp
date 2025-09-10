
use eframe::wgpu::{self, include_wgsl, util::DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureBlitUniforms {
    x: u32,
    y: u32,
    size: u32
}

pub const ATLAS_SIZE: u32 = 8192;
pub const TEXTURE_SIZE: u32 = 512;
pub const ATLAS_CELLS: u32 = ATLAS_SIZE / TEXTURE_SIZE;

pub struct TextureBlitter {
    #[allow(unused)]
    error: wgpu::Texture,
    error_view: wgpu::TextureView,
    #[allow(unused)]
    error_sampler: wgpu::Sampler,

    blit_target_bind_group_layout: wgpu::BindGroupLayout,

    blit_uniform_buffer: wgpu::Buffer,
    blit_source_bind_group_layout: wgpu::BindGroupLayout,
    blit_pipeline: wgpu::ComputePipeline
}

impl TextureBlitter {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {

        let error = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("texture_error"),
                size: wgpu::Extent3d {
                    width: 2,
                    height: 2,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }
        );
        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &error,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All 
            },
            &[
                255, 0, 255, 255,
                0,   0, 0,   255,
                0,   0, 0,   255,
                255, 0, 255, 255,
            ],
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(8),
                rows_per_image: Some(2) 
            },
            wgpu::Extent3d {
                width: 2,
                height: 2,
                depth_or_array_layers: 1
            }
        );
        let error_view = error.create_view(&wgpu::TextureViewDescriptor::default());
        let error_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("texture_error_sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        let blit_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("texture_blit_uniforms"),
                contents: bytemuck::cast_slice(&[
                    TextureBlitUniforms {
                        x: 0,
                        y: 0,
                        size: TEXTURE_SIZE
                    }
                ]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let blit_target_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_blit_target_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2 
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
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

        let blit_source_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("blit_source_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false 
                        },
                        count: None,
                    }
                ] 
            }
        );

        let blit_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("texture_blit_pipeline_layout"),
                bind_group_layouts: &[&blit_target_bind_group_layout, &blit_source_bind_group_layout],
                push_constant_ranges: &[] 
            }
        );

        let blit_shader = device.create_shader_module(include_wgsl!("texture_atlas/blit.wgsl"));

        let blit_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("blit_pipeline"),
                layout: Some(&blit_layout),
                module: &blit_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default() 
            }
        );


        Self {
            error,
            error_view,
            error_sampler,
            blit_target_bind_group_layout,
            blit_uniform_buffer,
            blit_source_bind_group_layout,
            blit_pipeline,
        }
    }

    pub fn blit(&self, device: &wgpu::Device, queue: &wgpu::Queue, atlas: &TextureAtlas, src: &wgpu::TextureView, slot: u32) {

        let cell_x = slot % ATLAS_CELLS;
        let cell_y = slot / ATLAS_CELLS;

        queue.write_buffer(&self.blit_uniform_buffer, 0, bytemuck::cast_slice(&[
            TextureBlitUniforms {
                x: cell_x * TEXTURE_SIZE,
                y: cell_y * TEXTURE_SIZE,
                size: TEXTURE_SIZE,
            }
        ]));

        let src_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("blit_source_bind_group"),
                layout: &self.blit_source_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(src),
                    }
                ]
            }
        );

        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("texture_blit_encoder"),
            }
        );

        let mut pass = encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor {
                label: Some("blit_compute_pass"),
                timestamp_writes: None
            }
        );

        pass.set_pipeline(&self.blit_pipeline);
        pass.set_bind_group(0, &atlas.blit_target_bind_group, &[]);
        pass.set_bind_group(1, &src_bind_group, &[]);
        pass.dispatch_workgroups(TEXTURE_SIZE / 16, TEXTURE_SIZE / 16, 1);

        drop(pass);

        queue.submit([encoder.finish()]);
    }

    pub fn blit_error(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, atlas: &TextureAtlas, slot: u32) {
        self.blit(device, queue, atlas, &self.error_view, slot);
    }

}

pub struct TextureAtlas {
    #[allow(unused)]
    atlas: wgpu::Texture,
    pub atlas_view: wgpu::TextureView,
    pub atlas_sampler: wgpu::Sampler,
    blit_target_bind_group: wgpu::BindGroup,
}

impl TextureAtlas {
    
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, blitter: &mut TextureBlitter) -> Self {
        let size = wgpu::Extent3d {
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            depth_or_array_layers: 1,
        };

        let atlas = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("texture_atlas"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
                view_formats: &[],
            }
        );

        let atlas_view = atlas.create_view(&wgpu::TextureViewDescriptor::default());
        let atlas_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("texture_atlas_sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        let blit_target_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("blit_target_bind_group"),
                layout: &blitter.blit_target_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(blitter.blit_uniform_buffer.as_entire_buffer_binding()),
                    }
                ] 
            }
        );

        let atlas = Self {
            atlas,
            atlas_view,
            atlas_sampler,

            blit_target_bind_group
        };

        atlas
    }

}