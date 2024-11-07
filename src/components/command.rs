use std::default;

use eframe::egui;
use egui::{Button, ScrollArea, Separator};

use crate::{app::AppDM, project};
use src_logic::types::*;
use src_logic::slide::slbl_matrix2;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default, Clone)]
pub(crate) enum ProjectCommand { // Make it own the data with box
    #[default]
    NoCommand,
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
        let dem_loaded = !self.project.dem.dem.x.is_empty();
        ui.vertical(|ui|{
            match &mut self.current_command {
                ProjectCommand::NoCommand => self.ui_no_command(ui),
                ProjectCommand::Note(_) => {self.ui_note(ui)},
                ProjectCommand::OpenDem(_) => {self.ui_open_dem(ui)},
                ProjectCommand::DemGeometry(_) => {
                    if dem_loaded {self.ui_dem_geometry(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::OpenSurface(_) => {
                    if dem_loaded {self.ui_open_surface(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::SlblExact(_) => {
                    if dem_loaded {self.ui_slbl_exact(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::SlblRoutine(_) => {
                    if dem_loaded {self.ui_slbl_routine(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::SurfaceMin(_) => {
                    if dem_loaded {self.ui_surface_min(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::SurfaceMax(_) => {
                    if dem_loaded {self.ui_surface_max(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::ModelSurface(_) => {
                    if dem_loaded {self.ui_model_surface(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::ModelGradient(_) => {
                    if dem_loaded {self.ui_model_gradient(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::ModelCombine(_) => {
                    if dem_loaded {self.ui_model_combine(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::SatGeometry(_) => {
                    if dem_loaded {self.ui_sat_geometry(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::OpenDisp(_) => {
                    if dem_loaded {self.ui_open_disp(ui)} else {self.ui_no_dem(ui)}
                },
                ProjectCommand::CalibrateModel(_) => {
                    if dem_loaded {self.ui_calibrate_model(ui)} else {self.ui_no_dem(ui)}
                },
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

#[derive(Debug, Default, Clone)]
pub struct Note {
    status: CommandStatus,
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
    surface_name: String,
}

#[derive(Debug, Clone)]
pub struct SlblExact {
    status: CommandStatus,
    first_pnt: usize,
    last_pnt: usize,
    tol: f32,
    pub(crate) temp_surface: Surface1D,
}

impl Default for SlblExact {
    fn default() -> Self {
        SlblExact {
            status: CommandStatus::default(),
            first_pnt: 0,
            last_pnt: 1,
            tol: 1.,
            temp_surface: Surface1D::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SlblRoutine {
    status: CommandStatus,
    first_x: f32,
    last_x: f32,
    tol: f32,
    n_it: usize,
}

#[derive(Debug, Clone)]
pub struct SurfaceMin {
    status: CommandStatus,
    first_surface_index: usize,
    second_surface_index: usize,
}

impl Default for SurfaceMin {
    fn default() -> Self {
        SurfaceMin {
            status: CommandStatus::default(),
            first_surface_index: 0,
            second_surface_index: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceMax {
    status: CommandStatus,
    first_surface_index: usize,
    second_surface_index: usize,
}

impl Default for SurfaceMax {
    fn default() -> Self {
        SurfaceMax {
            status: CommandStatus::default(),
            first_surface_index: 0,
            second_surface_index: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelSurface {
    status: CommandStatus,
    surface_index: usize,
    first_pnt: usize,
    last_pnt: usize,
}

impl Default for ModelSurface {
    fn default() -> Self {
        ModelSurface {
            status: CommandStatus::default(),
            surface_index: 0,
            first_pnt: 0,
            last_pnt: 1,
        }
    }
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
    fn ui_no_dem(&self, ui: &mut egui::Ui) -> () {
        ui.label("No DEM loaded!");
        ui.separator();
        ui.label("Please load a DEM to start working on a project");
    }

    fn ui_no_command(&self, ui: &mut egui::Ui) -> () {
        ui.label("No command");
        ui.label("Please select a command to begin...");
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
                ui.add_space(10.);
                ui.horizontal(|ui| {
                    ui.label("Project name: ");
                    ui.text_edit_singleline(&mut self.project.name);
                });
                ui.add_space(10.);
                ui.label("You can add here notes that describe the current project.");
                ui.text_edit_multiline(&mut self.project.note);
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
                ui.add_space(10.);
                ui.horizontal(|ui| {
                    ui.label("name");
                    ui.text_edit_singleline(&mut data.surface_name);
                });
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
                            match self.project.open_surface_from_file(f.to_string(), data.surface_name.to_string()) {
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
                let text_first = "First point at ".to_string() + self.project.dem.dem.x[data.first_pnt].to_string().as_str() + "m";
                let text_last = "Last point at ".to_string() + self.project.dem.dem.x[data.last_pnt].to_string().as_str() + "m";
                ui.add(egui::Slider::new(&mut data.first_pnt, 0..=(data.last_pnt - 1)).text(text_first));
                ui.add(egui::Slider::new(&mut data.last_pnt, (data.first_pnt + 1)..=(self.project.dem.dem.x.len() - 1)).text(text_last));
                ui.add(egui::Slider::new(&mut data.tol, 0.0..=2.0).text("Tolerance"));
            });
        });

        // data.temp_surface.z = slbl_matrix2(&self.project.dem.dem, data.first_pnt, data.last_pnt, data.tol);

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
                    self.project.surface_from_exact_slbl(data.first_pnt, data.last_pnt, data.tol);
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

        let nb_surf = self.project.surfaces.len();

        if nb_surf >= 2 {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    egui::ComboBox::from_label("First surface")
                        .selected_text(self.project.surfaces[data.first_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.second_surface_index) {
                            ui.selectable_value(&mut data.first_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });

                    egui::ComboBox::from_label("Second surface")
                        .selected_text(self.project.surfaces[data.second_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.first_surface_index) {
                            ui.selectable_value(&mut data.second_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });
                });
            });
        } else {
            ui.label("Not enough surfaces. Need at least 2.");
        }

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
                    self.project.surface_from_min(data.first_surface_index, data.second_surface_index);
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

        let nb_surf = self.project.surfaces.len();

        if nb_surf >= 2 {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    egui::ComboBox::from_label("First surface")
                        .selected_text(self.project.surfaces[data.first_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.second_surface_index) {
                            ui.selectable_value(&mut data.first_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });

                    egui::ComboBox::from_label("Second surface")
                        .selected_text(self.project.surfaces[data.second_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.first_surface_index) {
                            ui.selectable_value(&mut data.second_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });
                });
            });
        } else {
            ui.label("Not enough surfaces. Need at least 2.");
        }
        

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
                    self.project.surface_from_max(data.first_surface_index, data.second_surface_index);
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

        let nb_surf = self.project.surfaces.len();

        if nb_surf >= 1 {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    ui.add(egui::Slider::new(&mut data.first_pnt, 0..=(data.last_pnt - 1)).text("First point"));
                    ui.add(egui::Slider::new(&mut data.last_pnt, (data.first_pnt + 1)..=(self.project.dem.dem.x.len() - 1)).text("Last point"));
                    egui::ComboBox::from_label("From surface")
                        .selected_text(self.project.surfaces[data.surface_index].name.clone())
                        .show_ui(ui, |ui| {
                            for k in 0..nb_surf {
                                ui.selectable_value(&mut data.surface_index, k, self.project.surfaces[k].name.clone());
                            }
                        })
                });
            });
        } else {
            ui.label("Need a surface to perform");
        }

        

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
                    self.project.disp_from_surf(data.surface_index, data.first_pnt, data.last_pnt);
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
