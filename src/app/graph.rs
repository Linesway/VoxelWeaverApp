
use super::App;

pub mod ui;

impl App {

    pub fn render_graph(&mut self, ui: &mut egui::Ui) {
        self.project.terrain_graph.render(ui, &mut self.actions, &self.project.biomes);
    }

}
