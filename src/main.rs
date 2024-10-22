mod app;
mod components;
mod project;
use app::AppDM;

fn main() {
    let viewport = egui::ViewportBuilder::default()
        .with_title("Slow Landslide Displacement Model")
        .with_inner_size([1280., 720.])
        .with_min_inner_size([640., 360.]);

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "sldm_app", 
        native_options, 
        Box::new(|cc| Ok(Box::new(AppDM::new(cc))))
    );
}
