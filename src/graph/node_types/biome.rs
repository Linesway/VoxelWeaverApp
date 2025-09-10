
use std::collections::HashMap;
use crate::{app::graph::ui::{PARAM_H_MARGIN, PARAM_SIZE}, graph::{GraphProjectInfo, NodeInput, NodeType, Type}};
use std::fmt::Write;

pub struct BiomeParameter {
    param: String 
}

impl NodeType for BiomeParameter {
    const LABEL: &'static str = "Biome Parameter";

    fn make() -> Self {
        Self {
            param: String::new()
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![("x", Type::Scalar)]
    }

    fn compile_wgsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        if let Some(idx) = info.biomes.biome_params.iter().position(|param| param == &self.param) {
            let _ = writeln!(out, "\tlet {} = b_{};", out_varnames["x"], idx); 
        } else {
            let _ = writeln!(out, "\tlet {} = 0.0;", out_varnames["x"]);
        }
    }

    fn compile_hlsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        if let Some(idx) = info.biomes.biome_params.iter().position(|param| param == &self.param) {
            let _ = writeln!(out, "\tfloat {} = b_{};", out_varnames["x"], idx); 
        } else {
            let _ = writeln!(out, "\tfloat {} = 0.0;", out_varnames["x"]);
        }
    }

    fn custom_ui_height() -> f32 {
        13.0
    }

    fn custom_ui(&mut self, ui: &mut egui::Ui, info: &GraphProjectInfo) {
        ui.horizontal_centered(|ui| {
            ui.add_space((PARAM_SIZE.x - 100.0) / 2.0 + PARAM_H_MARGIN);

            let idx = info.biomes.biome_params.iter().position(|param| param == &self.param);
            egui::ComboBox::new("biome_parameter", "")
                .selected_text(idx.map(|_| self.param.as_str()).unwrap_or("Select..."))
                .width(100.0)
                .truncate()
                .show_ui(ui, |ui| {
                    if info.biomes.biome_params.len() == 0 {
                        ui.label("No parameters available.");
                    } else {
                        for param in &info.biomes.biome_params {
                            if ui.selectable_label(&self.param == param, param).clicked() {
                                self.param = param.clone();
                            }
                        }
                    } 
                });
        });
    }

    fn custom_serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "param": self.param
        })
    }

    fn custom_deserialize(&mut self, data: &serde_json::Value) {
        if let Some(param) = data.as_object().map(|data| data.get("param")).flatten().map(|idx| idx.as_str()).flatten() {
            self.param = param.to_owned(); 
        } 
    }

}

pub struct BiomeWeight {
    biome_idx: usize
}

impl NodeType for BiomeWeight {
    const LABEL: &'static str = "Biome Weight";

    fn make() -> Self {
        Self {
            biome_idx: 0
        }
    }

    fn inputs(&self) -> Vec<(&'static str, Type, &NodeInput)> {
        vec![]
    }

    fn inputs_mut(&mut self) -> Vec<(&'static str, Type, &mut NodeInput)> {
        vec![]
    }

    fn outputs() -> Vec<(&'static str, Type)> {
        vec![
            ("weight", Type::Scalar)
        ]
    }

    fn compile_wgsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        if self.biome_idx < info.biomes.biomes.len() {
            let _ = writeln!(out, "\tlet {} = biome_w[{}];", out_varnames["weight"], self.biome_idx);
        } else {
            let _ = writeln!(out, "\tlet {} = 0.0;", out_varnames["weight"]);
        }
    }

    fn compile_hlsl(&self, _args: HashMap<&'static str, String>, out_varnames: HashMap<&'static str, String>, out: &mut String, info: &GraphProjectInfo) {
        if self.biome_idx < info.biomes.biomes.len() {
            let _ = writeln!(out, "\tfloat {} = biome_w.w[{}];", out_varnames["weight"], self.biome_idx);
        } else {
            let _ = writeln!(out, "\tfloat {} = 0.0;", out_varnames["weight"]);
        }
    }

    fn custom_ui_height() -> f32 {
        13.0
    }

    fn custom_ui(&mut self, ui: &mut egui::Ui, info: &GraphProjectInfo) {
        ui.horizontal_centered(|ui| {
            ui.add_space((PARAM_SIZE.x - 100.0) / 2.0 + PARAM_H_MARGIN);

            egui::ComboBox::new("biome", "")
                .selected_text(info.biomes.biomes.get(self.biome_idx).map(|s| s.name.as_str()).unwrap_or("NONE"))
                .width(100.0)
                .truncate()
                .show_ui(ui, |ui| {
                    if info.biomes.biomes.len() == 0 {
                        ui.label("No biomes available.");
                    } else {
                        for (idx, biome) in info.biomes.biomes.iter().enumerate() {
                            if ui.selectable_label(self.biome_idx == idx, &biome.name).clicked() {
                                self.biome_idx = idx;
                            }
                        }
                    } 
                });
        });
    }

    fn custom_serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "biome_idx": self.biome_idx
        })
    }

    fn custom_deserialize(&mut self, data: &serde_json::Value) {
        if let Some(idx) = data.as_object().map(|data| data.get("biome_idx")).flatten().map(|idx| idx.as_u64()).flatten() {
            self.biome_idx = idx as usize;
        } 
    }

}
