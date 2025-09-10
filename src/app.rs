
mod viewport;
pub mod graph;
mod biome;
pub mod action;
pub mod texture_loader;

use core::f32;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use action::ActionManager;
use texture_loader::TextureLoader;
use viewport::{TerrainRenderResources, ViewportTab};
use crate::biome::Biomes;
use crate::compiler::biomes::compile_biome_preview;
use crate::compiler::{compile, CompilationTarget};
use crate::graph::node_types::terrain::{HeightmapTerrain, TerrainOutput};
use crate::graph::{NodeInput, TerrainGraph, Value};
use crate::project::Project;
use crate::terrain::biome_preview::BiomePreviewRenderer;
use crate::terrain::texture_atlas::{TextureAtlas, TextureBlitter};
use crate::terrain::{meshgen::TerrainMeshGenerator, renderer::TerrainRenderer, Terrain};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidePanelTab {
    Graph,
    Biomes
}

pub struct App {
    project: Project,

    actions: ActionManager,

    god_center: glam::Vec3,
    god_size: f32,
    cam_yaw: f32,
    cam_pitch: f32,

    biome_preview_depth: f32,

    tri_counter: Arc<Mutex<u64>>,

    side_panel_tab: SidePanelTab,
    viewport_tab: ViewportTab,

    project_path: PathBuf,

    regenerate_on_update: bool,
    prev_sdf_code: String,
    prev_biome_preview_code: String,

    add_biome_parameter_dialog_open: bool,
    add_biome_parameter_name: String,

    texture_loader: TextureLoader,
    blitter: TextureBlitter,
    
    prev_unreal_hlsl: String
}

const UNDO_SHORTCUT: egui::KeyboardShortcut = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Z);
const REDO_SHORTCUT: egui::KeyboardShortcut = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Y);

impl App {

    pub fn new(cc: &eframe::CreationContext, project_path: PathBuf, texture_path: PathBuf) -> Self {

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        let device = &wgpu_render_state.device;
        let queue = &wgpu_render_state.queue;

        // create default graph
        let mut terrain_graph = TerrainGraph::new();
        let heightmap = terrain_graph.add_node(egui::Pos2::ZERO, HeightmapTerrain {
            height: Value::scalar(0.0).into(),
        });
        terrain_graph.add_node(egui::pos2(300.0, 25.0), TerrainOutput {
            terrain: NodeInput {
                val: Value::terrain(),
                connection: Some((heightmap, 0)),
            },
        });
        terrain_graph.transform = egui::emath::TSTransform::from_translation(egui::vec2(0.0, 300.0));

        // create default biomes
        let biomes = Biomes::new();
        
        // init GPU stuff
        let terrain_rndr = TerrainRenderer::new(device, wgpu_render_state.target_format);

        let texture_loader = TextureLoader::new(texture_path.clone());

        let sdf_code = compile(&terrain_graph, &biomes, &texture_loader, 
            CompilationTarget::WGSL,project_path.file_name().unwrap().to_str()); 

        let mesh_generator = TerrainMeshGenerator::new(device, &sdf_code);

        let biome_preview_code = compile_biome_preview(&biomes);

        let mut blitter = TextureBlitter::new(device, queue); 
        let atlas = TextureAtlas::new(device, queue, &mut blitter);
        blitter.blit_error(device, queue, &atlas, 0);

        let mut app = Self {
            project: Project {
                terrain_graph, 
                biomes
            },
            actions: ActionManager::new(),
            god_center: glam::Vec3::splat(0.0),
            god_size: 128.0,
            cam_yaw: 0.0,
            cam_pitch: 0.5,
            biome_preview_depth: 0.0,
            tri_counter: Arc::new(Mutex::new(0)),
            side_panel_tab: SidePanelTab::Graph,
            viewport_tab: ViewportTab::Terrain,
            project_path: project_path.clone(),
            regenerate_on_update: true,
            prev_sdf_code: String::new(),
            prev_biome_preview_code: biome_preview_code.clone(),
            add_biome_parameter_dialog_open: false,
            add_biome_parameter_name: String::new(),
            texture_loader,
            blitter,
            prev_unreal_hlsl: "".to_owned()
        };

        app.load_project();

        let biome_preview_renderer = BiomePreviewRenderer::new(device, wgpu_render_state.target_format, biome_preview_code);

        wgpu_render_state.renderer.write().callback_resources.insert(TerrainRenderResources {
            terrain: Terrain::new(),
            renderer: terrain_rndr,
            mesh_generator,
            texture_atlas: atlas,
            request_redraw: false,
            biome_preview_renderer
        });

        app
    }

    fn save_project(&mut self) {
        let data = self.project.to_json();
        
        let _ = std::fs::write(self.project_path.join("project.terrain"), data.to_string());
    }

    fn load_project(&mut self) {
        let Ok(file) = std::fs::File::open(self.project_path.join("project.terrain")) else { return; };
        let Ok(data) = serde_json::from_reader::<_, serde_json::Value>(file) else { return; };
        self.project.load_from_json(data);
    }

}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let device = &frame.wgpu_render_state().unwrap().device;
        let queue = &frame.wgpu_render_state().unwrap().queue;

        ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });

        egui::TopBottomPanel::top("menu_bar")
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button(format!("Project Name: {}", self.project_path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or(String::new())), |ui| {
                        ui.label(format!("Path: {}", self.project_path.to_string_lossy().to_string()));
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.add_enabled(
                            self.actions.can_undo(),
                            egui::Button::new("Undo").shortcut_text(ui.ctx().format_shortcut(&UNDO_SHORTCUT))).clicked() {
                                self.actions.undo(&mut self.project, device, queue);
                        }
                        if ui.add_enabled(
                            self.actions.can_redo(),
                            egui::Button::new("Redo").shortcut_text(ui.ctx().format_shortcut(&REDO_SHORTCUT))).clicked() {
                                self.actions.redo(&mut self.project, device, queue);
                        }
                    });
                });
            });

        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(400.0)
            .frame(egui::Frame::none().fill(ctx.style().visuals.window_fill))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("side_panel_menu")
                    .frame(egui::Frame::none().inner_margin(4.0))
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui.selectable_label(self.side_panel_tab == SidePanelTab::Graph, "Terrain Graph").clicked() {
                                    self.side_panel_tab = SidePanelTab::Graph;
                                }
                                if ui.selectable_label(self.side_panel_tab == SidePanelTab::Biomes, "Biomes").clicked() {
                                    self.side_panel_tab = SidePanelTab::Biomes;
                                }
                            });
                        });
                });
                egui::CentralPanel::default()
                    .show_inside(ui, |ui| {
                        match self.side_panel_tab {
                            SidePanelTab::Graph => self.render_graph(ui),
                            SidePanelTab::Biomes => self.render_biomes(ui),
                        }
                });
        });

        let mut renderer = frame.wgpu_render_state().unwrap().renderer.write();
        let resources = renderer.callback_resources.get_mut::<TerrainRenderResources>().unwrap();

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            self.render_viewport(ui, device, frame.wgpu_render_state().unwrap().target_format, resources);
        });

        if ctx.input_mut(|i| i.consume_shortcut(&UNDO_SHORTCUT)) {
            self.actions.undo(&mut self.project, device, queue);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&REDO_SHORTCUT)) {
            self.actions.redo(&mut self.project, device, queue);
        }

        if resources.request_redraw {
            ctx.request_repaint();
            resources.request_redraw = false;
        }

        self.save_project();
        self.texture_loader.tick(&device, &queue, &mut self.blitter, &mut renderer);

        let hlsl_code = compile(&self.project.terrain_graph, &self.project.biomes, &self.texture_loader, CompilationTarget::UnrealHLSL, self.project_path.file_name().unwrap().to_str());

        if hlsl_code != self.prev_unreal_hlsl {
            std::fs::write(self.project_path.join("unreal.ush"), &hlsl_code).unwrap();
            self.prev_unreal_hlsl = hlsl_code;
        }
    }

}
