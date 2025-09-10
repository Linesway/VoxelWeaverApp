

use std::ops::RangeInclusive;

use egui::Response;

use crate::app::action::{Action, ActionManager};

pub fn get_init_numeric_val<N: egui::emath::Numeric + Send + Sync>(ui: &mut egui::Ui, resp: &Response, prev_val: N, curr_val: N) -> Option<N> {

    if resp.drag_started() || resp.gained_focus() {
        ui.memory_mut(|mem| mem.data.insert_temp(resp.id, prev_val));
    }

    if resp.lost_focus() || resp.drag_stopped() {
        if let Some(init_val) = ui.memory(|mem| mem.data.get_temp(resp.id)) {
            if init_val != curr_val {
                return Some(init_val);
            }
        }
    }

    None
}

pub fn slider_with_undo(ui: &mut egui::Ui, val: &mut f32, range: RangeInclusive<f32>, create_undo: fn(f32) -> Action, actions: &mut ActionManager) {

    let prev_val = *val;
    let resp = ui.add(egui::Slider::new(val, range));
    if let Some(init_val) = get_init_numeric_val(ui, &resp, prev_val, *val) {
        actions.push_undo_action(create_undo(init_val));    
    }

}

pub fn drag_value_with_undo<N: egui::emath::Numeric + Send + Sync, F: FnOnce(N) -> Action>(ui: &mut egui::Ui, val: &mut N, create_undo: F, actions: &mut ActionManager) {
    let prev_val = *val;
    let resp = ui.add(egui::DragValue::new(val));
    if let Some(init_val) = get_init_numeric_val(ui, &resp, prev_val, *val) {
        actions.push_undo_action(create_undo(init_val));    
    }
}

pub fn ranged_drag_value_with_undo<N: egui::emath::Numeric + Send + Sync, F: FnOnce(N) -> Action>(ui: &mut egui::Ui, val: &mut N, range: RangeInclusive<N>, create_undo: F, actions: &mut ActionManager) {
    let prev_val = *val;
    let resp = ui.add(egui::DragValue::new(val).range(range));
    if let Some(init_val) = get_init_numeric_val(ui, &resp, prev_val, *val) {
        actions.push_undo_action(create_undo(init_val));    
    }
}

pub fn textedit_with_undo<F: FnOnce(String) -> Action>(ui: &mut egui::Ui, val: &mut String, create_undo: F, actions: &mut ActionManager) {

    let prev_val = val.clone();

    let resp = ui.text_edit_singleline(val);

    if resp.gained_focus() {
        ui.memory_mut(|mem| mem.data.insert_temp(resp.id, prev_val));
    }

    if resp.lost_focus() { 
        if let Some(init_val) = ui.memory(|mem| mem.data.get_temp(resp.id)) {
            actions.push_undo_action(create_undo(init_val));
        }
    }

}