
use eframe::wgpu;

use crate::{app::App, biome::BIOME_DEPTH_LIMIT, compiler::biomes::compile_biome_preview, terrain::biome_preview::BiomePreviewRenderer};

use super::TerrainRenderResources;

struct RenderCallback {
    apsect: f32,
    depth: f32,
}

impl eframe::egui_wgpu::CallbackTrait for RenderCallback {

    fn prepare(
            &self,
            _device: &eframe::wgpu::Device,
            queue: &eframe::wgpu::Queue,
            _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut eframe::wgpu::CommandEncoder,
            callback_resources: &mut eframe::egui_wgpu::CallbackResources,
        ) -> Vec<eframe::wgpu::CommandBuffer> {
            let TerrainRenderResources { biome_preview_renderer, .. } = callback_resources.get().unwrap();
            biome_preview_renderer.prepare(queue, self.apsect, self.depth);
            vec![]
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'a>,
        callback_resources: &'a eframe::egui_wgpu::CallbackResources,
    ) {
        
        let TerrainRenderResources { biome_preview_renderer, .. } = callback_resources.get().unwrap();
        biome_preview_renderer.render(render_pass);

    }

}

impl App {

    pub fn render_biomes_viewport(&mut self, ui: &mut egui::Ui, device: &wgpu::Device, format: wgpu::TextureFormat, resources: &mut TerrainRenderResources) {

        let min_depth = self.project.biomes.biomes.iter().map(|biome| biome.min_depth).min().unwrap_or(-BIOME_DEPTH_LIMIT).max(-200) as f32; 
        let max_depth = self.project.biomes.biomes.iter().map(|biome| biome.max_depth).max().unwrap_or(BIOME_DEPTH_LIMIT).min(200) as f32; 

        egui::TopBottomPanel::bottom(ui.next_auto_id())
            .show_inside(ui, |ui| {
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    ui.label("Biome preview depth: ");
                    ui.add(egui::Slider::new(&mut self.biome_preview_depth, min_depth..=max_depth));
                });
            });

        let biome_preview_code = compile_biome_preview(&self.project.biomes);
        if biome_preview_code != self.prev_biome_preview_code {
            resources.biome_preview_renderer = BiomePreviewRenderer::new(device, format, biome_preview_code.clone());
            self.prev_biome_preview_code = biome_preview_code;
        }

        let (rect, _resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

        let cb = eframe::egui_wgpu::Callback::new_paint_callback(
            rect,
            RenderCallback {
                apsect: rect.aspect_ratio(),
                depth: self.biome_preview_depth
            }
        );
        ui.painter().add(cb);

    }

}
