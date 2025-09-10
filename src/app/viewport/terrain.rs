
use core::f32;
use std::sync::{Arc, Mutex};
use eframe::wgpu;
use egui::Pos2;

use crate::{app::{viewport::TerrainRenderResources, App}, compiler::{compile, CompilationTarget}};

fn god_view_cam_pos(center: glam::Vec3, yaw: f32, pitch: f32, r: f32) -> glam::Vec3 {
    let yaw_vec = glam::vec3(yaw.cos(), 0.0, yaw.sin());
    r * (yaw_vec * pitch.cos() + glam::Vec3::Y * pitch.sin()) + center
} 

struct RenderCallback {
    aspect: f32,

    god_center: glam::Vec3,
    god_size: f32,
    cam_yaw: f32,
    cam_pitch: f32,

    tri_counter: Arc<Mutex<u64>>
}

impl eframe::egui_wgpu::CallbackTrait for RenderCallback {

    fn prepare(
            &self,
            device: &eframe::wgpu::Device,
            queue: &eframe::wgpu::Queue,
            _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut eframe::wgpu::CommandEncoder,
            callback_resources: &mut eframe::egui_wgpu::CallbackResources,
        ) -> Vec<eframe::wgpu::CommandBuffer> {

            let resources = callback_resources.get_mut::<TerrainRenderResources>().unwrap();
            let TerrainRenderResources { renderer, mesh_generator, texture_atlas, request_redraw, .. } = resources;

            *request_redraw |= mesh_generator.generate(device, queue, &mut resources.terrain, self.god_center, self.god_size, &self.tri_counter);

            let cam_pos = god_view_cam_pos(self.god_center, self.cam_yaw, self.cam_pitch, self.god_size * 1.15);
            let view = glam::Mat4::look_at_rh(cam_pos, self.god_center, glam::Vec3::Y);

            let proj = glam::Mat4::perspective_rh(f32::consts::PI / 2.0, self.aspect, 0.01, 1000.0); 
            let cam_trans = proj * view; 
            renderer.prepare(device, queue, cam_trans, self.god_center - glam::Vec3::splat(self.god_size / 2.0), self.god_center + glam::Vec3::splat(self.god_size / 2.0), &texture_atlas);

            vec![]
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'a>,
        callback_resources: &'a eframe::egui_wgpu::CallbackResources,
    ) {
        let resources = callback_resources.get::<TerrainRenderResources>().unwrap();
        let resources: &TerrainRenderResources = &resources;
        resources.renderer.render(&resources.terrain, render_pass);
    }

}


impl App {

    pub fn render_terrain_viewport(&mut self, ui: &mut egui::Ui, device: &wgpu::Device, resources: &mut TerrainRenderResources) {

        let mut regenerate_terrain = self.regenerate_on_update;

        egui::TopBottomPanel::bottom(ui.next_auto_id())
            .show_inside(ui, |ui| {
                ui.add_space(15.0);
                if ui.button("Regenerate Terrain").clicked() {
                    regenerate_terrain = true;
                }
                ui.horizontal(|ui| {
                    ui.label("Regenerate on change: ");
                    ui.checkbox(&mut self.regenerate_on_update, "");
                });
                ui.label(format!("Triangles: {}", *self.tri_counter.lock().unwrap()));
            });

        let sdf_code = compile(&self.project.terrain_graph, &self.project.biomes, &self.texture_loader, 
            CompilationTarget::WGSL, self.project_path.file_name().unwrap().to_str());

        if regenerate_terrain && self.prev_sdf_code != sdf_code {
            resources.terrain.clear();
            resources.mesh_generator.update_shaders(&device, &sdf_code);
            self.prev_sdf_code = sdf_code;
        }

        let (rect, _resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let contains_pointer = rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::new(-10.0, -10.0))));

        if ui.input(|i| !i.modifiers.shift) {
            if ui.input(|i| i.pointer.primary_down()) && contains_pointer {
                let drag_delta = ui.input(|i| i.pointer.delta());
                self.cam_yaw += drag_delta.x * 0.03;
                self.cam_pitch += drag_delta.y * 0.03;
                self.cam_pitch = self.cam_pitch.clamp(-f32::consts::FRAC_PI_2 + 0.005, f32::consts::FRAC_PI_2 - 0.005);
            }
        } else {
            let cam_pos = god_view_cam_pos(self.god_center, self.cam_yaw, self.cam_pitch, 128.0); 
            let view_dir = self.god_center - cam_pos;
            let horiz = view_dir.cross(glam::Vec3::Y).normalize();
            let vert = horiz.cross(view_dir).normalize();
            if ui.input(|i| i.pointer.primary_down()) && contains_pointer {
                let drag_delta = ui.input(|i| i.pointer.delta());
                self.god_center += drag_delta.x * horiz * 0.5;
                self.god_center += drag_delta.y * vert * 0.5;
            }
        }

        if contains_pointer {
            self.god_size *= (ui.input(|i| i.smooth_scroll_delta.y) * 0.05).exp();
            self.god_size = self.god_size.clamp(32.0, 640.0);
        }

        let cb = eframe::egui_wgpu::Callback::new_paint_callback(
            rect,
            RenderCallback {
                aspect: rect.aspect_ratio(),
                god_center: self.god_center,
                god_size: self.god_size,
                cam_yaw: self.cam_yaw,
                cam_pitch: self.cam_pitch,
                tri_counter: self.tri_counter.clone()
            }
        );
        ui.painter().add(cb);

    }

}
