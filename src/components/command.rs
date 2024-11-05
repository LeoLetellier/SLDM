use std::default;

use eframe::egui;
use egui::ScrollArea;

use crate::app::AppDM;
use src_logic::types::*;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum ProjectCommand {
    #[default]
    NoCommand,
    NewProject,
    OpenProject,
    CheckProject,
    OpenDem,
    DemGeometry,
    OpenSurface,
    SlblExact,
    SlblRoutine,
    SurfaceMin,
    SurfaceMax,
    ModelSurface,
    ModelGradient,
    ModelCombine,
    SatGeometry,
    OpenDisp,
    CalibrateModel,
}

impl AppDM {
    pub(crate) fn ui_command(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui|{
            match self.current_command {
                ProjectCommand::NoCommand => self.ui_no_command(ui),
                ProjectCommand::NewProject => self.ui_new_project(ui),
                ProjectCommand::OpenProject => self.ui_open_project(ui),
                ProjectCommand::CheckProject => self.ui_check_project(ui),
                ProjectCommand::OpenDem => self.ui_open_dem(ui),
                ProjectCommand::DemGeometry => self.ui_dem_geometry(ui),
                ProjectCommand::OpenSurface => self.ui_open_surface(ui),
                ProjectCommand::SlblExact => self.ui_slbl_exact(ui),
                ProjectCommand::SlblRoutine => self.ui_slbl_routine(ui),
                ProjectCommand::SurfaceMin => self.ui_surface_min(ui),
                ProjectCommand::SurfaceMax => self.ui_surface_max(ui),
                ProjectCommand::ModelSurface => self.ui_model_surface(ui),
                ProjectCommand::ModelGradient => self.ui_model_gradient(ui),
                ProjectCommand::ModelCombine => self.ui_model_combine(ui),
                ProjectCommand::SatGeometry => self.ui_sat_geometry(ui),
                ProjectCommand::OpenDisp => self.ui_open_disp(ui),
                ProjectCommand::CalibrateModel => self.ui_calibrate_model(ui),
            }
        });
        
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum CommandError {
    #[default]
    MiscError,
    InvalidFile,
    NoFile,
}

#[derive(Debug, Default, PartialEq)]
pub enum CommandStatus {
    #[default]
    Clean,
    Complete,
    Error(CommandError),
}

#[derive(Debug, Default)]
pub struct Commands {
    open_dem: OpenDem,
}

#[derive(Debug, Default)]
pub struct NewProject {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct OpenProject {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct CheckProject {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct OpenDem {
    status: CommandStatus,
    file_path: Option<String>,
}

#[derive(Debug, Default)]
pub struct DemGeometry {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct OpenSurface {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct SlblExact {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct SlblRoutine {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct SurfaceMin {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct SurfaceMax {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct ModelSurface {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct ModelGradient {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct ModelCombine {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct SatGeometry {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct OpenDisp {
    status: CommandStatus,
}

#[derive(Debug, Default)]
pub struct CalibrateModel {
    status: CommandStatus,
}

impl AppDM {
    fn ui_no_command(&mut self, ui: &mut egui::Ui) -> () {
        ui.label("No command");
        ui.label("Please select a command to begin...");
    }

    fn ui_new_project(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Create a New Project");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_open_project(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open an Existing Project");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_check_project(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Check the Consistency of the Current Project");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_open_dem(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open DEM from File");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.vertical(|ui| {
                    if ui.button("Select file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.command_data.open_dem.file_path = Some(path.display().to_string());
                        }
                    }
                    if let Some(f) = &self.command_data.open_dem.file_path {
                        ScrollArea::horizontal().show(ui, |ui| {
                            ui.label(f);
                        });
                    }
                });
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                match &self.command_data.open_dem.status {
                    CommandStatus::Error(CommandError::InvalidFile) => {
                        ui.label("Selected file is invalid");
                    },
                    CommandStatus::Error(CommandError::NoFile) => {
                        ui.label("No file provided");
                    },
                    _ => (),
                }
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                let apply_text= match self.command_data.open_dem.status {
                    CommandStatus::Clean => egui::RichText::new("Apply"),
                    CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                    CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
                };
                let apply_button= ui.button(apply_text);
                
                if apply_button.clicked() {
                    if self.command_data.open_dem.status != CommandStatus::Clean {
                        self.command_data.open_dem.status = CommandStatus::Clean;
                    } else {
                        match &self.command_data.open_dem.file_path {
                            Some(f) => {
                                match self.project.dem.open_from_file(f.to_string()) {
                                    Err(_) => self.command_data.open_dem.status = CommandStatus::Error(CommandError::InvalidFile),
                                    _ => self.command_data.open_dem.status = CommandStatus::Complete,
                                }
                            },
                            None => self.command_data.open_dem.status = CommandStatus::Error(CommandError::NoFile),
                        }
                    }
                }
            });
        });
    }

    fn ui_dem_geometry(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Set the Section Geometry");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_open_surface(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open an Existing Surface from File");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_slbl_exact(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Surface from an Exact SLBL");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_slbl_routine(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Surface from a Routine SLBL");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_surface_min(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Minimum Values");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_surface_max(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Maximum Values");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_model_surface(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Unit Model fro a Reference Surface");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_model_gradient(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Add an Amplitude Gradient to an Existing Model");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_model_combine(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Combine Multiple Unit Model");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_sat_geometry(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Set the Satellite Geometry");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_open_disp(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Load Satellite Displacement Data");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

    fn ui_calibrate_model(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Calibrate a Model using Displacement Data");
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
            });
        });
        ui.separator();
    }

}
