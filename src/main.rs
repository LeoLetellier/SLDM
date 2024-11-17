#![windows_subsystem = "windows"]

mod app;
mod components;
mod project;
use app::AppDM;

fn main() {
    let viewport = egui::ViewportBuilder::default()
        .with_title("Slow Landslide Displacement Model")
        .with_inner_size([1280., 720.])
        .with_min_inner_size([640., 360.])
        .with_icon(load_icon("./assets/favicon-32x32.png"));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "sldm_app", 
        native_options, 
        Box::new(|cc| Ok(Box::new(AppDM::new(cc))))
    );
}

fn load_icon(path: &str) -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
