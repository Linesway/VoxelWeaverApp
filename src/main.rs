
mod terrain;
mod graph;
mod app;
mod project;
mod biome;
mod compiler; 
pub mod util;

use app::App;

fn main() {

    let args: Vec<_> = std::env::args().collect();
    let proj_path = args[1].clone();
    let texture_path = args.get(2).unwrap_or(&proj_path).clone();

    let icon_data = eframe::icon_data::from_png_bytes(&include_bytes!("../res/icon.png")[..]).expect("failed to load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default() 
            .with_title("Voxel Weaver")
            .with_maximized(true)
            .with_icon(icon_data),
        renderer: eframe::Renderer::Wgpu,
        depth_buffer: 24,
        ..Default::default()
    };
    eframe::run_native(
        "voxelweaver",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc, proj_path.into(), texture_path.into())))),
    ).unwrap();
}
