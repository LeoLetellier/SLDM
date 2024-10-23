use eframe::egui;

use egui_phosphor::regular as Phosphor;
use super::AppDM;
use crate::components::command::ProjectCommand;

impl AppDM {
    fn header(text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text).size(18.)
    }

    fn header_main(text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text).size(22.)
    }

    pub(super) fn ui_context_menu(&mut self, ui: &mut egui::Ui) {
        let header_file = Self::header_main(Phosphor::FILE.to_string() + " File");
        let header_surface = Self::header_main(Phosphor::LINE_SEGMENTS.to_string() + " Surface");
        let header_model = Self::header_main(Phosphor::VECTOR_TWO.to_string() + " Model");
        let header_calibration = Self::header_main(Phosphor::TRAY_ARROW_DOWN.to_string() + " Calibration");
        let header_about = Self::header_main(Phosphor::INFO.to_string() + " About");

        ui.horizontal(|ui| {
            // Menu File
            ui.menu_button(header_file.strong(), |ui| {
                ui.set_max_width(200.);
                let header_project = Self::header(Phosphor::FOLDER.to_string() + " Project");
                let header_dem = Self::header(Phosphor::LINE_SEGMENTS.to_string() + " DEM");

                ui.menu_button(header_project, |ui| {
                    if ui.button(Self::header("New")).clicked() {
                        // TODO
                    }
                    if ui.button(Self::header("Open")).clicked() {
                        // TODO
                    }
                    if ui.button(Self::header("Check consistency")).clicked() {
                        // TODO
                    }
                });

                ui.menu_button(header_dem, |ui| {
                    if ui.button(Self::header("From file")).clicked() {
                        self.current_command = ProjectCommand::OpenDem;
                    }
                    if ui.button(Self::header("Define Geometry")).clicked() {
                        // TODO
                    }
                });
            });

            // Menu Surface
            ui.menu_button(header_surface.strong(), |ui| {
                ui.set_max_width(200.);
                let header_from_file = Self::header(Phosphor::FILE_ARROW_DOWN.to_string() + " From file");
                let header_from_geometry = Self::header(Phosphor::LINE_SEGMENT.to_string() + " From geometry");
                let header_from_surfaces = Self::header(Phosphor::STACK_PLUS.to_string() + " From surfaces");

                if ui.button(header_from_file).clicked() {
                    // TODO
                }
                ui.menu_button(header_from_geometry, |ui| {
                    if ui.button(Self::header("SLBL exact")).clicked() {
                        // TODO
                    }
                    if ui.button(Self::header("SLBL routine")).clicked() {
                        // TODO
                    }
                });
                ui.menu_button(header_from_surfaces, |ui| {
                    if ui.button(Self::header("Minimum")).clicked() {
                        // TODO
                    }
                    if ui.button(Self::header("Maximum")).clicked() {
                        // TODO
                    }
                });
            });

            // Menu Model
            ui.menu_button(header_model.strong(), |ui| {
                ui.set_max_width(200.);
                let header_from_surface = Self::header(Phosphor::STACK.to_string() + " From surface");
                let header_add_gradient = Self::header(Phosphor::GRADIENT.to_string() + " Add gradient");
                let header_create_combined_model = Self::header(Phosphor::ROWS_PLUS_TOP.to_string() + " Create combined model");

                if ui.button(header_from_surface).clicked() {
                    // TODO
                }
                if ui.button(header_add_gradient).clicked() {
                    // TODO
                }
                if ui.button(header_create_combined_model).clicked() {
                    // TODO
                }
            });

            // Menu Calibration
            ui.menu_button(header_calibration.strong(), |ui| {
                ui.set_max_width(200.);
                let header_new_satellite_geometry = Self::header(Phosphor::COMPASS_TOOL.to_string() + " New satellite geometry");
                let header_displacement_data = Self::header(Phosphor::ARROWS_OUT_CARDINAL.to_string() + " Displacement data");

                if ui.button(header_new_satellite_geometry).clicked() {
                    // TODO
                }
                ui.menu_button(header_displacement_data, |ui| {
                    if ui.button(Self::header("From file")).clicked() {
                        // TODO
                    }
                });
            });
            
            if ui.button(header_about.strong()).clicked() {
                self.is_about_open = true;
            }
        });
    }
}
