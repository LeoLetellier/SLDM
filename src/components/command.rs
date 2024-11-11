use eframe::egui;
use egui::{DragValue, ScrollArea};

use crate::{app::AppDM, project::{self, BundleSar}};
use src_logic::prelude::*;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default, Clone)]
pub(crate) enum ProjectCommand {
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
    ModelNew(ModelNew),
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
                ProjectCommand::ModelNew(_) => {
                    if dem_loaded {self.ui_model_new(ui)} else {self.ui_no_dem(ui)}
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

#[derive(Debug, Default, Clone)]
pub struct ModelNew {
    status: CommandStatus,
    name: String,
    surface_params: Vec<SurfaceParams>,
}

#[derive(Debug, Default, Clone)]
pub struct SurfaceParams {
    pub(crate) index: usize,
    pub(crate) boundaries: (usize, usize),
    pub(crate) gradient_points: Vec<(usize, f32)>,
    pub(crate) weight: f32,
}

#[derive(Debug, Default, Clone)]
pub struct SatGeometry {
    status: CommandStatus,
    name: String,
    azimuth: f32,
    incidence: f32,
}

#[derive(Debug, Default, Clone)]
pub struct OpenDisp {
    status: CommandStatus,
    sar_index: usize,
    file_path: String,
    name: String,
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

        if !self.project.surfaces.is_empty() | !self.project.models.is_empty() | !self.project.models.is_empty() | !self.project.sars.is_empty() {
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
                ui.add(egui::Slider::new(&mut data.azimuth, 0.0..=359.99).text("Section azimuth"));

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
                    self.project.dem.section_geometry = Some(Orientation::from_deg(data.azimuth, 0.).unwrap());
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

    fn ui_model_new(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Create a New model from surfaces combination");
        let ProjectCommand::ModelNew(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();

                if self.project.surfaces.is_empty() {
                    ui.label("no surfaces to use");
                } else {
                    ui.text_edit_singleline(&mut data.name);
                    for k in 0..data.surface_params.len() {
                        ui.push_id(k, |ui| {
                            egui::ComboBox::from_label("Surface")
                                .selected_text(self.project.surfaces[data.surface_params[k].index].name.to_owned())
                                .show_ui(ui, |ui| {
                                    for s in 0..self.project.surfaces.len() {
                                        ui.selectable_value(&mut data.surface_params[s].index, k, self.project.surfaces[s].name.to_owned());
                                    }
                            });
                            ui.add(egui::DragValue::new(&mut data.surface_params[k].boundaries.0).range(0..=(self.project.dem.dem.x.len() - 1)));
                            ui.add(egui::DragValue::new(&mut data.surface_params[k].boundaries.1).range(0..=(self.project.dem.dem.x.len() - 1)));
                            ui.add(egui::DragValue::new(&mut data.surface_params[k].weight));
                            for i in 0..data.surface_params[k].gradient_points.len() {
                                ui.push_id(i, |ui| {
                                    ui.add(egui::DragValue::new(&mut data.surface_params[k].gradient_points[i].0).range(0..=self.project.dem.dem.x.len()));
                                    ui.add(egui::Slider::new(&mut data.surface_params[k].gradient_points[i].1, -1000.0..=1000.0).logarithmic(true));
                                });
                            }
                            if ui.button("+").clicked() {
                                data.surface_params[k].gradient_points.push((0, 1.));
                            }
                            if ui.button("-").clicked() {
                                data.surface_params[k].gradient_points.pop();
                            }
                        });
                    }
                    if ui.button("+").clicked() {
                        data.surface_params.push(SurfaceParams::default());
                    }
                    if ui.button("-").clicked() {
                        data.surface_params.pop();
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
                    self.project.combine_unit_models(&data.name, &data.surface_params);
                    // check input is valid
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
                ui.text_edit_singleline(&mut data.name);
                ui.add(egui::Slider::new(&mut data.azimuth, 0.0..=359.99).text("LOS azimuth"));
                ui.add(egui::Slider::new(&mut data.incidence, 0.0..=90.).text("LOS incidence"));
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
                    let mut new_bundle = BundleSar::default();
                    let orientation = Orientation::from_deg(data.azimuth, data.incidence);
                    match orientation {
                        Err(_) => data.status = CommandStatus::Error(CommandError::MiscError),
                        Ok(orientation) => {
                            new_bundle.sar_geometry = orientation;
                            new_bundle.name = data.name.to_owned();
                            self.project.sars.push(new_bundle);

                            data.status = CommandStatus::Complete;
                        },
                    }                    
                }
            }
        });
    }

    fn ui_open_disp(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Load Satellite Displacement Data");
        let ProjectCommand::OpenDisp(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        if self.project.sars.is_empty() {
            ui.label("No geometry defined");
        } else {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    ui.text_edit_singleline(&mut data.name);
                    egui::ComboBox::from_label("With geometry")
                        .selected_text(self.project.sars[data.sar_index].name.to_owned())
                        .show_ui(ui, |ui| {
                            for k in (0..self.project.sars.len()) {
                                ui.selectable_value(&mut data.sar_index, k, self.project.sars[k].name.to_owned());
                            }
                    });
                    if ui.button("Select file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            data.file_path = path.display().to_string();
                        }
                    }
                });
            });
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
                    data.status = match self.project.new_sar_data(&data.name, data.sar_index, data.file_path.to_owned()) {
                        Err(e) => CommandStatus::Error(CommandError::MiscError),
                        Ok(_) => CommandStatus::Complete,
                    }
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
