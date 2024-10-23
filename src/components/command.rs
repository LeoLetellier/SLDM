use eframe::egui;

use crate::app::AppDM;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum ProjectCommand {
    #[default]
    NoCommand,
    OpenDem,
}

impl AppDM {
    pub(crate) fn ui_command(&mut self, ui: &mut egui::Ui) {
        match self.current_command {
            ProjectCommand::NoCommand => {
                ui.label("No command");
                ui.label("Please select a command to begin...");
            },
            ProjectCommand::OpenDem => {
                let title = egui::RichText::new("Open DEM from file");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true), |ui| {
                    ui.label(title);
                    ui.separator();
                    if ui.button("Open file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let dem_path = Some(path.display().to_string());
                            command_open_dem(self, dem_path.unwrap());
                        }
                    }
                });
            }
        }
    }
}

fn command_open_dem(app: &mut AppDM, dem_path: String) {
    app.project.dem.open_from_file(dem_path);
}
