use std::default;

use eframe::egui;
use egui::{Button, ScrollArea};

use crate::{app::AppDM, project};
use src_logic::types::*;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default, Clone)]
pub(crate) enum ProjectCommand { // Make it own the data with box
    #[default]
    NoCommand,
    NewProject(NewProject),
    OpenProject(OpenProject),
    Note(Note),
    OpenDem(OpenDem),
    DemGeometry(DemGeometry),
    OpenSurface(OpenSurface),
    SlblExact(SlblExact),
    SlblRoutine(SlblRoutine),
    SurfaceMin(SurfaceMin),
    SurfaceMax(SurfaceMax),
    ModelSurface(ModelSurface),
    ModelGradient(ModelGradient),
    ModelCombine(ModelCombine),
    SatGeometry(SatGeometry),
    OpenDisp(OpenDisp),
    CalibrateModel(CalibrateModel),
}

impl AppDM {
    pub(crate) fn ui_command(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui|{
            match &mut self.current_command {
                ProjectCommand::NoCommand => self.ui_no_command(ui),
                ProjectCommand::NewProject(_) => self.ui_new_project(ui),
                ProjectCommand::OpenProject(_) => self.ui_open_project(ui),
                ProjectCommand::Note(_) => self.ui_note(ui),
                ProjectCommand::OpenDem(_) => self.ui_open_dem(ui),
                ProjectCommand::DemGeometry(_) => self.ui_dem_geometry(ui),
                ProjectCommand::OpenSurface(_) => self.ui_open_surface(ui),
                ProjectCommand::SlblExact(_) => self.ui_slbl_exact(ui),
                ProjectCommand::SlblRoutine(_) => self.ui_slbl_routine(ui),
                ProjectCommand::SurfaceMin(_) => self.ui_surface_min(ui),
                ProjectCommand::SurfaceMax(_) => self.ui_surface_max(ui),
                ProjectCommand::ModelSurface(_) => self.ui_model_surface(ui),
                ProjectCommand::ModelGradient(_) => self.ui_model_gradient(ui),
                ProjectCommand::ModelCombine(_) => self.ui_model_combine(ui),
                ProjectCommand::SatGeometry(_) => self.ui_sat_geometry(ui),
                ProjectCommand::OpenDisp(_) => self.ui_open_disp(ui),
                ProjectCommand::CalibrateModel(_) => self.ui_calibrate_model(ui),
            }
        });
        
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CommandError {
    #[default]
    MiscError,
    InvalidFile,
    NoFile,
    InvalidFolder,
    NoFolder,
    ProjectInitialized,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CommandStatus {
    #[default]
    Clean,
    Complete,
    Error(CommandError),
}

#[derive(Debug, Default)]
pub struct Commands {
    new_project: NewProject,
    open_project: OpenProject,
    open_dem: OpenDem,
    note: Note,
    dem_geometry: DemGeometry,
    open_surface: OpenSurface,
    slbl_exact: SlblExact,
    slbl_routine: SlblRoutine,
    surface_min: SurfaceMin,
    surface_max: SurfaceMax,
    model_surface: ModelSurface,
    model_gradient: ModelGradient,
    model_combine: ModelCombine,
    sat_geometry: SatGeometry,
    open_disp: OpenDisp,
    calibrate_model: CalibrateModel,
}

#[derive(Debug, Default, Clone)]
pub struct NewProject {
    status: CommandStatus,
    project_name: String,
    project_in_folder: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct OpenProject {
    status: CommandStatus,
    project_folder: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Note {
    status: CommandStatus,
    content: String,
}

#[derive(Debug, Default, Clone)]
pub struct OpenDem {
    status: CommandStatus,
    file_path: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct DemGeometry {
    status: CommandStatus,
    azimuth: f32,
}

#[derive(Debug, Default, Clone)]
pub struct OpenSurface {
    status: CommandStatus,
    file_path: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct SlblExact {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct SlblRoutine {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct SurfaceMin {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct SurfaceMax {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct ModelSurface {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct ModelGradient {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct ModelCombine {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct SatGeometry {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
pub struct OpenDisp {
    status: CommandStatus,
}

#[derive(Debug, Default, Clone)]
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
        let ProjectCommand::NewProject(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.text_edit_singleline(&mut data.project_name);
                if ui.button("Select Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        data.project_in_folder = Some(path.display().to_string());
                    }
                }
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_open_project(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open an Existing Project");
        let ProjectCommand::OpenProject(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                if ui.button("Select Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        data.project_folder = Some(path.display().to_string());
                    }
                }
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_note(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Note on the Current Project");
        let ProjectCommand::Note(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.label("You can add here notes that describe the current project.");
                ui.text_edit_multiline(&mut data.content);
            });
        });
    }

    fn ui_open_dem(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open DEM from File");
        let ProjectCommand::OpenDem(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        if !self.project.surfaces.is_empty() | !self.project.unit_models.is_empty() | !self.project.composition_models.is_empty() | !self.project.sars.is_empty() {
            data.status = CommandStatus::Error(CommandError::ProjectInitialized);
        }

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.vertical(|ui| {
                    if data.status == CommandStatus::Error(CommandError::ProjectInitialized) {
                        ui.disable();
                    }
                    if ui.button("Select file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            data.file_path = Some(path.display().to_string());
                        }
                    }
                    if let Some(f) = &data.file_path {
                        ScrollArea::horizontal().show(ui, |ui| {
                            ui.label(f);
                        });
                    }
                });
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                match &data.status {
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
                let apply_text= match data.status {
                    CommandStatus::Clean => egui::RichText::new("Apply"),
                    CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                    CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
                };
                let apply_button= ui.button(apply_text);
                
                if apply_button.clicked() {
                    if data.status != CommandStatus::Clean {
                        data.status = CommandStatus::Clean;
                    } else {
                        match &data.file_path {
                            Some(f) => {
                                match self.project.open_dem_from_file(f.to_string()) {
                                    Err(_) => data.status = CommandStatus::Error(CommandError::InvalidFile),
                                    _ => data.status = CommandStatus::Complete,
                                }
                            },
                            None => data.status = CommandStatus::Error(CommandError::NoFile),
                        }
                    }
                }
            });
        });
    }

    fn ui_dem_geometry(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Set the Section Geometry");
        let ProjectCommand::DemGeometry(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add(egui::Slider::new(&mut data.azimuth, 0.0..=360.0).text("Section azimuth"))

            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_open_surface(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open an Existing Surface from File");
        let ProjectCommand::OpenSurface(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                if ui.button("Select file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        data.file_path = Some(path.display().to_string());
                    }
                }
                if let Some(f) = &data.file_path {
                    ScrollArea::horizontal().show(ui, |ui| {
                        ui.label(f);
                    });
                }
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => {
                    match e {
                        CommandError::InvalidFile => ui.label("Invalid file"),
                        _ => ui.label(""),
                    }
                },
                _ => ui.label(""),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match &data.file_path {
                        Some(f) => {
                            match self.project.open_surface_from_file(f.to_string()) {
                                Err(_) => data.status = CommandStatus::Error(CommandError::InvalidFile),
                                _ => data.status = CommandStatus::Complete,
                            }
                        },
                        None => data.status = CommandStatus::Error(CommandError::NoFile),
                    }
                }
            }
        });
    }

    fn ui_slbl_exact(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Surface from an Exact SLBL");
        let ProjectCommand::SlblExact(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_slbl_routine(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Surface from a Routine SLBL");
        let ProjectCommand::SlblRoutine(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_surface_min(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Minimum Values");
        let ProjectCommand::SurfaceMin(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };
        
        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_surface_max(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Maximum Values");
        let ProjectCommand::SurfaceMax(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_model_surface(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Unit Model fro a Reference Surface");
        let ProjectCommand::ModelSurface(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_model_gradient(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Add an Amplitude Gradient to an Existing Model");
        let ProjectCommand::ModelGradient(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_model_combine(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Combine Multiple Unit Model");
        let ProjectCommand::ModelCombine(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_sat_geometry(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Set the Satellite Geometry");
        let ProjectCommand::SatGeometry(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_open_disp(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Load Satellite Displacement Data");
        let ProjectCommand::OpenDisp(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

    fn ui_calibrate_model(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Calibrate a Model using Displacement Data");
        let ProjectCommand::CalibrateModel(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text);
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    // Logic part
                    data.status = CommandStatus::Complete;
                }
            }
        });
    }

}
