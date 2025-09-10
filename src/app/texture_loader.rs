
use std::{collections::{HashMap, HashSet}, path::PathBuf};

use eframe::wgpu;

use crate::terrain::texture_atlas::TextureBlitter;

use super::viewport::TerrainRenderResources;

pub struct Texture {
    #[allow(unused)]
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    #[allow(unused)]
    sampler: wgpu::Sampler,
    pub egui_texture: egui::TextureId,
    pub slot: u32 
}

impl Texture {

    fn new(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, device: &wgpu::Device, queue: &wgpu::Queue, renderer: &mut eframe::egui_wgpu::Renderer) -> Self {

        let dimensions = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
                view_formats: &[],
            }
        );
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let egui_texture = renderer.register_native_texture(device, &texture_view, wgpu::FilterMode::Linear);

        Self {
            texture,
            view: texture_view,
            sampler,
            egui_texture,
            slot: 0 // initialized by texture loader
        }
    }

}

pub struct TextureLoader {
    #[allow(unused)]
    thread: std::thread::JoinHandle<()>,
    rx: std::sync::mpsc::Receiver<(std::path::PathBuf, image::ImageBuffer<image::Rgba<u8>, Vec<u8>>)>,
    pub textures: HashMap<PathBuf, Texture>,
    curr_slot: u32 
}

impl TextureLoader {

    pub fn new(proj_path: PathBuf) -> Self {

        let (tx, rx) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || {


            let mut loaded = HashSet::new();

            // poor man's file watcher
            loop {
                for path in walkdir::WalkDir::new(&proj_path).into_iter()
                    .filter_map(|entry | entry.ok())
                    .map(|entry| entry.into_path()) { 
                        if loaded.contains(&path) {
                            continue;
                        }
                        let Some(ext) = path.extension() else { continue; };
                        let ext = ext.to_string_lossy().into_owned();
                        let Some(_format) = image::ImageFormat::from_extension(ext) else { continue; };
                        let Ok(data) = std::fs::read(&path) else { continue; };
                        let Ok(img) = image::load_from_memory(&data) else { return; };
                        loaded.insert(path.clone());
                        let Some(path) = pathdiff::diff_paths(path, &proj_path) else { return; };
                        let _ = tx.send((path, img.to_rgba8()));
                }
            }

        });

        Self {
            thread,
            rx,
            textures: HashMap::new(),
            curr_slot: 1
        }

    }

    pub fn tick(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, blitter: &mut TextureBlitter, renderer: &mut eframe::egui_wgpu::Renderer) {

        if let Ok((path, img)) = self.rx.try_recv() {
            let mut texture = Texture::new(img, device, queue, renderer);
            let resources = renderer.callback_resources.get_mut::<TerrainRenderResources>().unwrap();
            blitter.blit(device, queue, &resources.texture_atlas, &texture.view, self.curr_slot);
            texture.slot = self.curr_slot;
            self.curr_slot += 1;
            self.textures.insert(path, texture);
        }

    }

    pub fn get(&self, path: &PathBuf) -> u32 {
        self.textures.get(path).map(|tex| tex.slot).unwrap_or(0)
    }

}
