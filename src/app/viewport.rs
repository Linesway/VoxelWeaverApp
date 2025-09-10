

use eframe::wgpu;

use crate::terrain::{biome_preview::BiomePreviewRenderer, meshgen::TerrainMeshGenerator, renderer::TerrainRenderer, texture_atlas::TextureAtlas, Terrain};

use super::App;

pub mod terrain;
pub mod biomes;

pub struct TerrainRenderResources {
    pub terrain: Terrain,
    pub renderer: TerrainRenderer,
    pub mesh_generator: TerrainMeshGenerator,
    pub texture_atlas: TextureAtlas,
    pub request_redraw: bool,
    pub biome_preview_renderer: BiomePreviewRenderer
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewportTab {
    Terrain,
    Biomes
}

impl App {

    pub fn render_viewport(&mut self, ui: &mut egui::Ui, device: &wgpu::Device, format: wgpu::TextureFormat, resources: &mut TerrainRenderResources) {

        egui::TopBottomPanel::top("viewport_options")
            .frame(egui::Frame::none().fill(ui.visuals().window_fill).inner_margin(4.0))
            .show_inside(ui, |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.viewport_tab == ViewportTab::Terrain, "Terrain").clicked() {
                            self.viewport_tab = ViewportTab::Terrain;
                        }
                        if ui.selectable_label(self.viewport_tab == ViewportTab::Biomes, "Biomes").clicked() {
                            self.viewport_tab = ViewportTab::Biomes;
                        }
                    });
                })
            });
        
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                match self.viewport_tab {
                    ViewportTab::Terrain => self.render_terrain_viewport(ui, device, resources),
                    ViewportTab::Biomes => self.render_biomes_viewport(ui, device, format, resources),
                }
            });

    }

}
