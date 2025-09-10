
use std::{collections::HashMap, path::PathBuf};

use crate::{biome::{Biome, BIOME_DEPTH_LIMIT}, util::ui::{drag_value_with_undo, ranged_drag_value_with_undo, slider_with_undo, textedit_with_undo}};

use super::{action::Action, App};

impl App {

    pub fn render_biomes(&mut self, ui: &mut egui::Ui) {
        if ui.button(format!("{} Biome", egui_phosphor::regular::PLUS)).clicked() {
            self.project.biomes.biomes.push(Biome {
                name: "Biome".into(),
                frequency: 1.0,
                texture: PathBuf::new(),
                params: HashMap::new(),
                min_depth: -BIOME_DEPTH_LIMIT,
                max_depth: BIOME_DEPTH_LIMIT,
                color: [1.0, 0.0, 0.0] 
            });
            self.actions.push_undo_action(Action::BiomeDelete(self.project.biomes.biomes.len() - 1));
        }

        ui.horizontal(|ui| {
            ui.label("Biome Size:");
            slider_with_undo(ui, &mut self.project.biomes.biome_size, 50.0..=2000.0, Action::BiomeSetSize, &mut self.actions);
        });
        ui.horizontal(|ui| {
            ui.label("Biome Blending:");
            slider_with_undo(ui, &mut self.project.biomes.biome_blending, 0.01..=0.9, Action::BiomeSetBlending, &mut self.actions);
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_exact_size(egui::Vec2::X * ui.available_width(), egui::Sense::click());
            let mut biome_to_delete = None; 
            for (idx, biome) in self.project.biomes.biomes.iter_mut().enumerate() {
                let header_resp = egui::CollapsingHeader::new(format!("{} (#{})", biome.name, idx))
                    .id_source(idx)
                    .show(ui, |ui| { 

                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            textedit_with_undo(ui, &mut biome.name, |name| Action::BiomeSetName(idx, name), &mut self.actions);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Frequency:");
                            drag_value_with_undo(ui, &mut biome.frequency, |freq| Action::BiomeSetFrequency(idx, freq), &mut self.actions);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Depth Range:");
                            ranged_drag_value_with_undo(ui, &mut biome.min_depth, -BIOME_DEPTH_LIMIT..=biome.max_depth, |depth| Action::BiomeSetMinDepth(idx, depth), &mut self.actions);
                            ranged_drag_value_with_undo(ui, &mut biome.max_depth, biome.min_depth..=BIOME_DEPTH_LIMIT, |depth| Action::BiomeSetMaxDepth(idx, depth), &mut self.actions);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Texture:");
                            egui::ComboBox::new(idx, "")
                                .truncate()
                                .selected_text(biome.texture.to_string_lossy())
                                .show_ui(ui, |ui| {
                                    for path in self.texture_loader.textures.keys() {
                                        if ui.selectable_label(path == &biome.texture, path.to_string_lossy()).clicked() {
                                            let old_texture = std::mem::replace(&mut biome.texture, path.clone());
                                            self.actions.push_undo_action(Action::BiomeSetTexture(idx, old_texture));
                                        }
                                    }
                                });
                        });
                        if let Some(texture) = self.texture_loader.textures.get(&biome.texture) {
                            ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                                id: texture.egui_texture,
                                size: egui::Vec2::splat(100.0),
                            }));
                        }

                        ui.add_space(12.0);
                        let mut to_delete = None;
                        for (idx, param) in self.project.biomes.biome_params.iter_mut().enumerate() { 
                            if !biome.params.contains_key(param) {
                                biome.params.insert(param.clone(), 0.0);
                            }
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(format!("{}:", param)).sense(egui::Sense::click())).context_menu(|ui| {
                                    if ui.button("Delete").clicked() {
                                        to_delete = Some(idx);          
                                    }
                                });
                                drag_value_with_undo(ui, biome.params.get_mut(param).unwrap(), |val| Action::BiomeSetParameter { biome: idx, param: param.clone(), val }, &mut self.actions);
                            });
                        }

                        if let Some(to_delete) = to_delete {
                            let param = self.project.biomes.biome_params.remove(to_delete);
                            self.actions.push_undo_action(Action::BiomeCreateParameter { param });
                        }
                        if ui.button(format!("{} Parameter", egui_phosphor::regular::PLUS)).clicked() {
                            self.add_biome_parameter_dialog_open = true;
                        }

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            ui.label("Debug Color:");
                            ui.color_edit_button_rgb(&mut biome.color);
                        });

                }).header_response;

                header_resp.context_menu(|ui| {
                    if ui.button("Delete").clicked() {
                        biome_to_delete = Some(idx);
                    }
                });
            }

            if let Some(biome_to_delete) = biome_to_delete {
                let biome = self.project.biomes.biomes.remove(biome_to_delete);
                self.actions.push_undo_action(Action::BiomeCreate(biome_to_delete, biome));
            }

        });

        let mut close_dialog = false;
        if self.add_biome_parameter_dialog_open {
            egui::Window::new("Add Biome Parameter")
                .open(&mut self.add_biome_parameter_dialog_open)
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .pivot(egui::Align2::CENTER_CENTER)
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.text_edit_singleline(&mut self.add_biome_parameter_name);
                        if ui.button("Add").clicked() {
                            self.actions.push_undo_action(Action::BiomeDeleteParameter { param: self.add_biome_parameter_name.clone() });
                            self.project.biomes.add_parameter(std::mem::replace(&mut self.add_biome_parameter_name, String::new()));
                            close_dialog = true;
                        }
                    });
                });
        }
        if close_dialog { 
            self.add_biome_parameter_dialog_open = false;
        }
    }

}
