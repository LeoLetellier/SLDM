// #![windows_subsystem = "window"]
#![windows_subsystem = "console"]

mod app;
mod components;
mod project;
use app::AppDM;

fn main() {
    std::env::set_var("RUST_LOG", "SLDM=trace,src_logic=trace");
    // std::env::set_var("RUST_LOG", "trace,wgpu_hal=off,wgpu_core=off,eframe=off,naga=off");
    env_logger::init();
    log::info!("Logger initialized");
    log::trace!("at trace level");

    let viewport = egui::ViewportBuilder::default()
        .with_title("Slow Landslide Displacement Model")
        .with_inner_size([1280., 720.])
        .with_min_inner_size([640., 360.])
        .with_icon(egui::IconData::default());

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "sldm_app",
        native_options,
        Box::new(|cc| Ok(Box::new(AppDM::new(cc)))),
    );
}
