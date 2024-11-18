use super::AppDM;
use crate::app::action_panel::Panel;
use crate::components::command::*;
use eframe::egui;
use egui_phosphor::regular as Phosphor;

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
        let header_calibration = Self::header_main(Phosphor::PLANET.to_string() + " Satellite");
        let header_about = Self::header_main(Phosphor::INFO.to_string() + " About");

        ui.horizontal(|ui| {
            // Menu File
            ui.menu_button(header_file.strong(), |ui| {
                ui.set_max_width(200.);
                let header_project = Self::header(Phosphor::FOLDER.to_string() + " Project");
                let header_dem = Self::header(Phosphor::LINE_SEGMENTS.to_string() + " DEM");

                ui.menu_button(header_project, |ui| {
                    if ui
                        .button(Self::header(Phosphor::FOLDER_OPEN.to_string() + " Open"))
                        .clicked()
                    {
                        self.load_project();
                        ui.close_menu();
                    }
                    if ui
                        .button(Self::header(Phosphor::FLOPPY_DISK.to_string() + " Save"))
                        .clicked()
                    {
                        self.save_project();
                        ui.close_menu();
                    }
                    if ui
                        .button(Self::header(Phosphor::NOTE.to_string() + " Note"))
                        .clicked()
                    {
                        self.open_command(ProjectCommand::Note(Note::default()));
                        ui.close_menu();
                    }
                });

                ui.menu_button(header_dem, |ui| {
                    if ui.button(Self::header("From file")).clicked() {
                        self.open_command(ProjectCommand::OpenDem(OpenDem::default()));
                        ui.close_menu();
                    }
                    if ui.button(Self::header("Define Geometry")).clicked() {
                        self.open_command(ProjectCommand::DemGeometry(DemGeometry::default()));
                        ui.close_menu();
                    }
                });
            });

            // Menu Surface
            ui.menu_button(header_surface.strong(), |ui| {
                ui.set_max_width(200.);
                let header_from_file =
                    Self::header(Phosphor::FILE_ARROW_DOWN.to_string() + " From file");
                let header_from_geometry =
                    Self::header(Phosphor::LINE_SEGMENT.to_string() + " From geometry");
                let header_from_surfaces =
                    Self::header(Phosphor::STACK_PLUS.to_string() + " From surfaces");

                if ui.button(header_from_file).clicked() {
                    self.open_command(ProjectCommand::OpenSurface(OpenSurface::default()));
                    ui.close_menu();
                }
                ui.menu_button(header_from_geometry, |ui| {
                    if ui.button(Self::header("SLBL exact")).clicked() {
                        self.open_command(ProjectCommand::SlblExact(SlblExact::default()));
                        ui.close_menu();
                    }
                    if ui.button(Self::header("SLBL routine")).clicked() {
                        self.open_command(ProjectCommand::SlblRoutine(SlblRoutine::default()));
                        ui.close_menu();
                    }
                });
                ui.menu_button(header_from_surfaces, |ui| {
                    if ui.button(Self::header("Minimum")).clicked() {
                        self.open_command(ProjectCommand::SurfaceMin(SurfaceMin::default()));
                        ui.close_menu();
                    }
                    if ui.button(Self::header("Maximum")).clicked() {
                        self.open_command(ProjectCommand::SurfaceMax(SurfaceMax::default()));
                        ui.close_menu();
                    }
                });
            });

            // Menu Model
            ui.menu_button(header_model.strong(), |ui| {
                ui.set_max_width(200.);
                let header_new_model =
                    Self::header(Phosphor::ROWS_PLUS_TOP.to_string() + " New model");
                let header_calibrate_model =
                    Self::header(Phosphor::TRAY_ARROW_DOWN.to_string() + " Calibrate model");

                if ui.button(header_new_model).clicked() {
                    self.open_command(ProjectCommand::ModelNew(ModelNew::default()));
                    ui.close_menu();
                }
                if ui.button(header_calibrate_model).clicked() {
                    self.open_command(ProjectCommand::CalibrateModel(CalibrateModel::default()));
                    ui.close_menu();
                }
            });

            // Menu Calibration
            ui.menu_button(header_calibration.strong(), |ui| {
                ui.set_max_width(200.);
                let header_new_satellite_geometry =
                    Self::header(Phosphor::COMPASS_TOOL.to_string() + " New acquisition geometry");
                let header_displacement_data =
                    Self::header(Phosphor::ARROWS_OUT_CARDINAL.to_string() + " Displacement data");

                if ui.button(header_new_satellite_geometry).clicked() {
                    self.open_command(ProjectCommand::SatGeometry(SatGeometry::default()));
                    ui.close_menu();
                }
                ui.menu_button(header_displacement_data, |ui| {
                    if ui.button(Self::header("From file")).clicked() {
                        self.open_command(ProjectCommand::OpenDisp(OpenDisp::default()));
                        ui.close_menu();
                    }
                });
            });

            if ui.button(header_about.strong()).clicked() {
                self.is_about_open = true;
            }

            ui.separator();
            ui.label(egui::RichText::new(&self.project.name).size(20.));
        });
    }

    fn open_command(&mut self, command: ProjectCommand) {
        self.current_panel = Panel::Command;
        self.current_command = command;
    }

    fn save_project(&mut self) {
        if self.project.path.is_none() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.project.path = Some(path.display().to_string());
            }
        }
        match &self.project.path {
            Some(_) => {
                let _ = self.project.save();
            }
            None => (),
        }
    }

    fn load_project(&mut self) {
        let project_path: String;
        match rfd::FileDialog::new()
            .add_filter("TOML", &["toml"])
            .pick_file()
        {
            Some(path) => {
                project_path = path.display().to_string();
                match self.project.load(&project_path) {
                    Ok(project_loading) => {
                        match &self.project.path {
                            Some(_) => {
                                let _ = self.project.save();
                            }
                            None => (),
                        };
                        self.reset_with_project(project_loading);
                    }
                    Err(_) => (),
                }
            }
            None => (),
        };
    }
}
