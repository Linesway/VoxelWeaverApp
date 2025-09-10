
use std::u32;

use egui::{emath::TSTransform, epaint::{CubicBezierShape, RectShape}, pos2, vec2, Align, Color32, Id, LayerId, Layout, Order, Pos2, Rect, Rounding, Sense, Shape, Stroke, TextureId, Vec2};

use crate::{app::action::{Action, ActionManager}, biome::Biomes, graph::{node_types::NODE_TYPES, Connection, GraphProjectInfo, Node, NodeId, TerrainGraph, Type, Value}, util::ui::{drag_value_with_undo, get_init_numeric_val}};

impl Type {

    fn color(&self) -> Color32 {
        match self {
            Type::Scalar => Color32::from_rgb(114, 198, 247),
            Type::Vector => Color32::from_rgb(250, 115, 255),
            Type::Terrain => Color32::from_rgb(158, 235, 91),
        } 
    }

}

impl Value {

    fn render_editor(&mut self, ui: &mut egui::Ui, actions: &mut ActionManager, label: &'static str, node: NodeId, in_idx: u32) {
        match self {
            Value::Scalar(val) => {
                let prev_val = *val;
                let drag_val = egui::DragValue::new(val)
                    .speed(0.05)
                    .update_while_editing(false)
                    .prefix(format!("{}: ", label));
                let rect = Rect::from_center_size(ui.available_rect_before_wrap().center(), Vec2::X * ui.available_width()).translate(Vec2::UP * 9.0);
                let resp = ui.put(rect, drag_val);

                if let Some(init_val) = get_init_numeric_val(ui, &resp, prev_val, *val) {
                    actions.push_undo_action(Action::GraphSetInput { node, in_idx, val: Value::Scalar(init_val) });
                }

            },
            Value::Vector(vec) => {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::X * 2.0;
                    let old_vec = *vec;
                    ui.label(format!("{}: ", label));
                    drag_value_with_undo(ui, &mut vec.x, |_old_x| Action::GraphSetInput { node, in_idx, val: Value::Vector(old_vec) }, actions);
                    drag_value_with_undo(ui, &mut vec.y, |_old_y| Action::GraphSetInput { node, in_idx, val: Value::Vector(old_vec) }, actions);
                    drag_value_with_undo(ui, &mut vec.z, |_old_z| Action::GraphSetInput { node, in_idx, val: Value::Vector(old_vec) }, actions);
                });
            },
            Value::Terrain => {
                ui.label(label);
            },
        }
    }

}

pub const TOPBAR_H: f32 = 20.0;
pub const PARAM_SIZE: Vec2 = vec2(150.0, 25.0);
pub const PARAM_H_MARGIN: f32 = 10.0;
pub const PARAM_V_MARGIN: f32 = 10.0;
pub const CONN_RADIUS: f32 = 5.0;
pub const NODE_ROUNDING: f32 = 5.0;

impl Node {

    fn calc_input_conn_pos(&self, inp_idx: u32) -> Pos2 {
        let n_outs = self.ty.outputs().len() as u32;
        self.pos + (((n_outs + inp_idx) as f32 + 0.5) * PARAM_SIZE.y + TOPBAR_H) * Vec2::DOWN
    } 

    fn calc_output_conn_pos(&self, out_idx: u32) -> Pos2 {
        self.pos + ((out_idx as f32 + 0.5) * PARAM_SIZE.y + TOPBAR_H) * Vec2::DOWN + (PARAM_SIZE.x + 2.0 * PARAM_H_MARGIN) * Vec2::RIGHT
    }

}

impl TerrainGraph {

    pub fn search_context_menu(&mut self, ui: &mut egui::Ui, actions: &mut ActionManager, new_node_pos: Pos2) {
        let resp = egui::TextEdit::singleline(&mut self.editor_search_query).min_size(vec2(200.0, 0.0)).show(ui).response;
        resp.request_focus();
        ui.separator();

        let mut matcher = code_fuzzy_match::FuzzyMatcher::new();
        let mut node_types = Vec::new();
        for (_, nodes) in NODE_TYPES {
            for node in *nodes {
                node_types.push((matcher.fuzzy_match(&node.label, &self.editor_search_query).map(|score| score as f64).unwrap_or(-2.0), node));
            }
        }

        node_types.sort_by(|(a_score, _a), (b_score, _b)| {
            if a_score < b_score {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        let avg_score = node_types.iter().map(|(score, _)| score).sum::<f64>() / (node_types.len() as f64);
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (score, node) in &node_types {
                if *score > avg_score - 0.5 && *score > -1.0 {
                    if ui.button(node.label).clicked() {
                        self.add_node_from_box_ty_with_action(new_node_pos, (node.make)(), actions);
                        ui.close_menu();
                    }
                }
            }
        });

        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
            if let Some((_, node)) = node_types.first() {
                self.add_node_from_box_ty_with_action(new_node_pos, (node.make)(), actions);
                ui.close_menu();
            }
        }
    }

    pub fn context_menu(&mut self, ui: &mut egui::Ui, actions: &mut ActionManager, rect: Rect) {
        let new_node_pos = self.transform.inverse() * ui.input(|i| i.pointer.hover_pos()).map(|pos| pos - rect.min.to_vec2()).unwrap_or(rect.min);

        if self.editor_searching {
            self.search_context_menu(ui, actions, new_node_pos);
            return;
        }

        let key_pressed = ui.input(|i| i.events.iter().filter(|ev| matches!(ev, egui::Event::Key {pressed: true, ..})).next().is_some());
        if key_pressed {
            self.editor_searching = true;
            ui.input(|i| {
                for ev in &i.events {
                    if let egui::Event::Text(text) = ev {
                        self.editor_search_query = text.clone();
                    }
                }
            });
        }

        if ui.button("Search...").clicked() {
            self.editor_searching = true;
        }

        ui.separator();

        for (category, nodes) in NODE_TYPES {
            ui.menu_button(*category, |ui| {
                for node in *nodes {
                    if ui.button(node.label).clicked() {
                        self.add_node_from_box_ty_with_action(new_node_pos, (node.make)(), actions);
                        ui.close_menu();
                    }
                }
            }); 
        }
    }

    fn node_area_contents(id: NodeId, actions: &mut ActionManager, ui_layer_id: LayerId, node_ui: &mut egui::Ui, rect: Rect, node_rect: Rect, node: &mut Node, delete_node: &mut bool, to_connect: &mut Vec<Connection>, to_disconnect: &mut Vec<u32>, connections_to_draw: &mut Vec<(Pos2, Pos2, Color32)>, reconnection_idx: &mut u32, reconnect: &mut bool, transform: TSTransform, biomes: &Biomes) {

        let label = node.ty.label(); 
        let outputs = node.ty.outputs();
        let inputs = node.ty.inputs_mut(); 
        
        node_ui.ctx().set_transform_layer(node_ui.layer_id(), transform);
        node_ui.ctx().set_sublayer(ui_layer_id, node_ui.layer_id());
        node_ui.set_clip_rect(transform.inverse() * rect);
        let painter = node_ui.painter(); 

        let topbar_rect = Rect::from_min_size(node_rect.min, vec2(node_rect.width(), TOPBAR_H));

        // paint node bg
        painter.add(Shape::Rect(RectShape {
            rect: node_rect,
            rounding: Rounding::ZERO,
            fill: Color32::from_black_alpha(80),
            stroke: Stroke::NONE,
            blur_width: 20.0 * transform.scaling,
            fill_texture_id: TextureId::User(0), 
            uv: Rect::ZERO,
        }));
        painter.rect(node_rect, Rounding::same(NODE_ROUNDING), node_ui.visuals().window_fill, Stroke::NONE);
        let topbar_color = Color32::from_gray(40);
        painter.rect(topbar_rect, Rounding { nw: NODE_ROUNDING, ne: NODE_ROUNDING, sw: 0.0, se: 0.0 }, topbar_color, Stroke::NONE);

        // node label
        node_ui.put(topbar_rect, egui::Label::new(label).selectable(false).truncate());

        // deletion button                
        let deletion_rect = Rect::from_two_pos(topbar_rect.right_top(), topbar_rect.right_bottom() + TOPBAR_H * Vec2::LEFT); 
        if node_ui.put(deletion_rect, egui::Label::new(egui_phosphor::regular::X).sense(egui::Sense::click())).clicked() {
            *delete_node = true;
        }

        node_ui.advance_cursor_after_rect(topbar_rect);

        let mut param_rect = Rect::from_min_size(
            topbar_rect.left_bottom() + vec2(PARAM_H_MARGIN, 0.0),
            PARAM_SIZE
        );
        for (out_idx, (label, ty)) in outputs.into_iter().enumerate() {
            let mut out_ui = node_ui.child_ui(param_rect, Layout::default(), None);
            out_ui.horizontal_centered(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(label);
                });
            });

            let painter = node_ui.painter(); 
            let conn_center = param_rect.right_center() + Vec2::X * PARAM_H_MARGIN;
            painter.circle(conn_center, CONN_RADIUS, ty.color(), Stroke::NONE);

            let conn_resp = node_ui.allocate_rect(Rect::from_center_size(conn_center, Vec2::splat(2.5 * CONN_RADIUS)), Sense::drag());
            conn_resp.dnd_set_drag_payload((ty, id, out_idx as u32));
            if conn_resp.drag_started() {
                conn_resp.request_focus();
            }
            if conn_resp.has_focus() {
                if let Some(mouse_pos) = node_ui.input(|i| i.pointer.hover_pos()) {
                    connections_to_draw.push((conn_center, transform.inverse() * mouse_pos, ty.color()));
                }
            }
            if conn_resp.drag_stopped() {
                conn_resp.surrender_focus();
            }

            param_rect = param_rect.translate(Vec2::Y * PARAM_SIZE.y);
        }
        for (inp_idx, (label, ty, inp)) in inputs.into_iter().enumerate() {
            let mut inp_ui = node_ui.child_ui(param_rect, Layout::default(), None);
            inp_ui.horizontal_centered(|ui| {
                if let Some(_) = &inp.connection {
                    ui.label(label);
                } else {
                    inp.val.render_editor(ui, actions, label, id, inp_idx as u32);
                }
            });

            let painter = node_ui.painter(); 
            let conn_center = param_rect.left_center() - Vec2::X * PARAM_H_MARGIN;
            painter.circle(conn_center, 5.0, ty.color(), Stroke::NONE);

            let conn_resp = node_ui.allocate_rect(Rect::from_center_size(conn_center, Vec2::splat(2.5 * CONN_RADIUS)), Sense::click_and_drag());

            if let Some(payload) = conn_resp.dnd_release_payload::<(Type, NodeId, u32)>() {
                let (out_ty, from_node_id, out_idx) = *payload;
                if out_ty == ty {
                    to_connect.push(Connection {
                        from: from_node_id,
                        out_idx,
                        to: id,
                        inp_idx: inp_idx as u32,
                    });
                }
            }

            if conn_resp.secondary_clicked() {
                to_disconnect.push(inp_idx as u32);
            }

            if conn_resp.drag_started() {
                conn_resp.request_focus();
            }
            if conn_resp.has_focus() {
                *reconnection_idx = inp_idx as u32;
            }
            if conn_resp.drag_stopped() {
                conn_resp.surrender_focus();
                *reconnect = true;
            }

            param_rect = param_rect.translate(Vec2::Y * PARAM_SIZE.y);
        }

        node.ty.custom_ui(node_ui, &GraphProjectInfo {
            biomes
        }); 

        node_ui.advance_cursor_after_rect(node_rect);
    }

    pub fn render_node(&mut self, id: NodeId, ui: &mut egui::Ui, actions: &mut ActionManager, rect: Rect, connections_to_draw: &mut Vec<(Pos2, Pos2, Color32)>, biomes: &Biomes, use_mouse: bool) {
        let node = self.nodes.get_mut(&id).unwrap();
        let outputs = node.ty.outputs();
        let custom_ui_height = node.ty.custom_ui_height();
        let inputs = node.ty.inputs_mut();

        // "commands"
        let mut delete_node = false;
        let mut to_connect = Vec::new();
        let mut to_disconnect = Vec::new();

        let node_rect = Rect::from_min_size(
            node.pos + rect.min.to_vec2(),
            vec2(
                PARAM_SIZE.x + 2.0 * PARAM_H_MARGIN,
                TOPBAR_H + PARAM_SIZE.y * ((outputs.len() + inputs.len()) as f32) + custom_ui_height + PARAM_V_MARGIN
            )
        );

        let mut reconnection_idx = u32::MAX;
        let mut reconnect = false;
        let resp = egui::Area::new(ui.id().with(id))
            .current_pos(node_rect.min)
            .default_size(node_rect.size())
            .constrain(false)
            .order(egui::Order::Foreground)
            .sense(egui::Sense::click_and_drag())
            .show(ui.ctx(), |node_ui| {
                Self::node_area_contents(id, actions, ui.layer_id(), node_ui, rect, node_rect, node, &mut delete_node, &mut to_connect, &mut to_disconnect, connections_to_draw, &mut reconnection_idx, &mut reconnect, self.transform, biomes); 
            }).response;

        ui.ctx().set_transform_layer(resp.layer_id, self.transform);
        ui.ctx().set_sublayer(ui.layer_id(), resp.layer_id);

        let node = self.nodes.get_mut(&id).unwrap();
        
        // dragging
        if resp.drag_started() && use_mouse {
            resp.request_focus();
            self.curr_drag_delta = Vec2::ZERO;
        }
        if resp.dragged() && resp.has_focus() {
            node.pos += resp.drag_delta();
            self.curr_drag_delta += resp.drag_delta();
        }
        if resp.drag_stopped() {
            resp.surrender_focus();
            actions.push_undo_action(Action::GraphMoveNode(id, -self.curr_drag_delta));
        }

        let inputs = node.ty.inputs_mut();
        let input_conns: Vec<_> = inputs.into_iter().enumerate().filter_map(|(inp_idx, (_name, ty, inp))| 
            if let Some((node_id, out_idx)) = &inp.connection {
                Some((inp_idx as u32, *node_id, *out_idx, ty))
            } else {
                None
            }
        ).collect();
        for (inp_idx, other_node, out_idx, ty) in input_conns {
            let out_pos = self.nodes[&other_node].calc_output_conn_pos(out_idx) + rect.min.to_vec2();
            let inp_pos = if reconnection_idx != inp_idx {
                self.nodes[&id].calc_input_conn_pos(inp_idx) + rect.min.to_vec2()
            } else {
                let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) else { continue; };
                self.transform.inverse() * mouse_pos
            };
            connections_to_draw.push((out_pos, inp_pos, ty.color()));
        }

        if delete_node {
            self.delete_node_with_action(id, actions);
            return;
        }
        for conn in to_connect {
            self.connect_with_action(conn, actions);
        }
        for inp_idx in to_disconnect {
            self.disconnect_with_action(id, inp_idx, actions);
        }
        let node = self.nodes.get(&id).unwrap();
        let inputs = node.ty.inputs();
        if reconnect && reconnection_idx < inputs.len() as u32 {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                let mouse_pos = self.transform.inverse() * (mouse_pos - rect.min.to_vec2());
                let mut found = false;
                'node_search: for (to_id, to) in &self.nodes {
                    for i in 0..to.ty.inputs().len() {
                        let i = i as u32;
                        let inp_pos = to.calc_input_conn_pos(i);
                        if inp_pos.distance(mouse_pos) < 1.25 * CONN_RADIUS {
                            let to_id = *to_id;
                            if let Some(broken_connection) = self.disconnect(id, reconnection_idx) {
                                if self.connect(Connection {
                                    from: broken_connection.from,
                                    out_idx: broken_connection.out_idx,
                                    to: to_id,
                                    inp_idx: i,
                                }).0 {
                                    actions.push_undo_action(Action::Compound(vec![
                                        Action::GraphDisconnect { to_node: to_id, in_idx: i },
                                        Action::GraphConnect(broken_connection)
                                    ]));
                                }
                            }
                            found = true;
                            break 'node_search;
                        }
                    }
                }
                if !found {
                    self.disconnect_with_action(id, reconnection_idx, actions);
                }
            }
        }

    }

    pub fn render(&mut self, ui: &mut egui::Ui, actions: &mut ActionManager, biomes: &Biomes) {
        let (rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

        let transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;
        if let Some(pointer) = ui.input(|i| i.pointer.hover_pos()) {
            if rect.contains(pointer) {
                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = if self.editor_searching { Vec2::ZERO } else { ui.ctx().input(|i| i.smooth_scroll_delta) };

                self.transform = self.transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                self.transform = TSTransform::from_translation(pan_delta) * self.transform;
            }
        }

        let painter = ui.painter_at(rect);

        let bg_color = Color32::from_gray(17);
        let bg_grid_color = Color32::from_gray(13);
        painter.rect(rect, Rounding::ZERO, bg_color, Stroke::NONE);

        let inv_rect = self.transform.inverse() * rect;
        let grid_spacing = 60.0;
        let left = (inv_rect.left() / grid_spacing).floor() as i32 - 1;
        let right = (inv_rect.right() / grid_spacing).ceil() as i32 + 1;
        for x in left..=right {
            let x = (self.transform * pos2((x as f32) * grid_spacing, 0.0)).x;
            painter.vline(x, rect.y_range(), Stroke::new(self.transform.scaling * 3.0, bg_grid_color));
        }
        let top = (inv_rect.top() / grid_spacing).floor() as i32 - 1;
        let bottom = (inv_rect.bottom() / grid_spacing).ceil() as i32 + 1;
        for y in top..=bottom {
            let y = (self.transform * pos2(0.0, (y as f32) * grid_spacing)).y;
            painter.hline(rect.x_range(), y, Stroke::new(self.transform.scaling * 3.0, bg_grid_color));
        }


        let mut connections = Vec::new();
        
        let use_mouse = resp.rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::new(-10.0, -10.0))));
        for id in self.nodes.keys().map(|id| *id).collect::<Vec<_>>() {
            self.render_node(id, ui, actions, rect, &mut connections, biomes, use_mouse); 
        }

        egui::Area::new(Id::from("connections")).order(Order::Middle).show(ui.ctx(), |ui| {
            let painter = ui.painter().with_clip_rect(rect);
            for (a, b, color) in connections {
                let a = self.transform * a;
                let b = self.transform * b;
                painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                    [
                        a,
                        a + vec2(100.0, 0.0) * self.transform.scaling,
                        b - vec2(100.0, 0.0) * self.transform.scaling,
                        b
                    ],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(CONN_RADIUS * self.transform.scaling, color)
                )));
            }
        });

        resp.context_menu(|ui| {
            self.context_menu(ui, actions, rect);
        });

        if !resp.context_menu_opened() {
            self.editor_searching = false;
            self.editor_search_query = String::new();
        }
        
    }

}
