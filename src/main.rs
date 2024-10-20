mod app;

use app::AppDM;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Slow Landslide Displacement Model", native_options, Box::new(|cc| Ok(Box::new(AppDM::new(cc)))));
}
