use eframe::egui;
use egui::{DragValue, InnerResponse, ScrollArea};

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
    InvalidOrientation,
    EmptyName,
    MethodError,
    InputError,
    EmptySar,
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

#[derive(Debug, Clone)]
pub struct OpenSurface {
    status: CommandStatus,
    file_path: Option<String>,
    surface_name: String,
}

impl Default for OpenSurface {
    fn default() -> Self {
        OpenSurface {
            status: CommandStatus::default(),
            file_path: None,
            surface_name: String::from("New surface"),
        }
    }
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

#[derive(Debug, Clone)]
pub struct SlblRoutine {
    status: CommandStatus,
    first_pnt: usize,
    last_pnt: usize,
    tol: f32,
    n_it: usize,
    min_elev: f32,
    max_slope: f32,
}

impl Default for SlblRoutine {
    fn default() -> Self {
        SlblRoutine {
            status: CommandStatus::default(),
            first_pnt: 1,
            last_pnt: 2,
            tol: 2.,
            n_it: 300,
            min_elev: 0.,
            max_slope: 90.,
        }
    }
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
    file_path: Option<String>,
    name: String,
}

#[derive(Debug, Default, Clone)]
pub struct CalibrateModel {
    status: CommandStatus,
    model: usize,
    sar_geom: usize,
    sar_data: usize,
}

impl AppDM {
    // fn ui_command_basis<R>(&self, data: & impl CommandData, ui: &mut egui::Ui, title: String, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> () {
    //     let title = egui::RichText::new(title).heading();
    //     ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
    //         ui.vertical( |ui| {
    //             ui.label(title);
    //             ui.separator();
    //             ui.add_space(10.);
    //             add_contents(ui);
    //         });
    //     });

    //     // ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
    //     //     match &data.status {
    //     //         CommandStatus::Error(CommandError::InvalidFile) => {
    //     //             ui.label("Selected file is invalid");
    //     //         },
    //     //         CommandStatus::Error(CommandError::NoFile) => {
    //     //             ui.label("No file provided");
    //     //         },
    //     //         _ => (),
    //     //     }
    //     // });
    // }

    // fn ui_test(&self, ui: &mut egui::Ui) -> () {
    //     let note = Note::default();
    //     self.ui_command_basis(&note, ui, String::from("My title"), |ui| {
    //         ui.label("something");
    //     });
    // }

    fn ui_no_dem(&mut self, ui: &mut egui::Ui) -> () {
        ui.label(egui::RichText::new("No DEM loaded!").heading());
        ui.separator();
        ui.add_space(10.);
        ui.label("Please load a DEM to continue...");
        ui.add_space(10.);
        if ui.button(egui::RichText::new("Load DEM").size(16.)).clicked() {
            self.current_command = ProjectCommand::OpenDem(OpenDem::default());
        }
    }

    fn ui_no_command(&self, ui: &mut egui::Ui) -> () {
        ui.label(egui::RichText::new("No Command Selected").heading());
        ui.separator();
        ui.add_space(10.);
        ui.label("Please select a command to continue...");
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
        // self.ui_command_basis(&data, ui, String::from("Note on the current project"), |ui| {
        //     ui.horizontal(|ui| {
        //         ui.label("Project name: ");
        //         ui.text_edit_singleline(&mut self.project.name);
        //     });
        //     ui.add_space(10.);
        //     ui.label("You can add here notes that describe the current project.");
        //     ui.text_edit_multiline(&mut self.project.note);
        // });
    }

    fn ui_open_dem(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open DEM from File").heading();
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
                ui.add_space(10.);
                ui.label("Use this command to load a Digital Elevation Model (DEM) from an existing file.");
                ui.label("The file should be a csv file with the headers 'x' for the sampling values and 'z' for the elevation values.");
                ui.label("Note that the sampling values should be equally spaced.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(15.);
                if data.status == CommandStatus::Error(CommandError::ProjectInitialized) {
                    ui.disable();
                }
                if ui.button(egui::RichText::new("Select file").size(18.)).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        data.file_path = Some(path.display().to_string());
                    }
                }
                ui.add_space(5.);
                if let Some(f) = &data.file_path {
                    ui.horizontal(|ui| {
                        ui.label("Selected file: ");
                        ScrollArea::horizontal().show(ui, |ui| {
                            ui.label(f);
                        });
                    });
                }
            });

            ui.add_space(10.);

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                match &data.status {
                    CommandStatus::Error(CommandError::InvalidFile) => {
                        ui.label("Error:");
                        ui.label("Selected file is invalid");
                    },
                    CommandStatus::Error(CommandError::NoFile) => {
                        ui.label("Error: ");
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
                let apply_button= ui.button(apply_text.size(22.));
                
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
        let title = egui::RichText::new("Set the Section Geometry").heading();
        let ProjectCommand::DemGeometry(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add_space(10.);
                ui.label("Use this command to configure the geometry of the 2D cross section.");
                ui.label("The azimuth angle corresponds to the clockwise angle between the North and the increasing x-axis direction.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.azimuth, 0.0..=360.).text("Section azimuth"));
            });
        });

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(CommandError::InvalidOrientation) => {ui.label("Invalid orientation");},
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    let azimuth = if data.azimuth == 360. {0.} else {data.azimuth};
                    match Orientation::from_deg(azimuth, 90.) {
                        Err(_) => data.status = CommandStatus::Error(CommandError::InvalidOrientation), // Should never reach
                        Ok(o) => {
                            self.project.dem.section_geometry = Some(o);
                            data.status = CommandStatus::Complete;
                        },
                    }
                }
            }
        });
    }

    fn ui_open_surface(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Open an Existing Surface from File").heading();
        let ProjectCommand::OpenSurface(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add_space(10.);
                ui.label("Use this command to load a surface from file.");
                ui.label("The file should be a csv file with the headers 'x' for the sampling values and 'z' for the elevation values.");
                ui.label("Note that the sampling values should be the save as the project's DEM.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(15.);
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut data.surface_name);
                });
                ui.add_space(5.);
                if ui.button(egui::RichText::new("Select file").size(18.)).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        data.file_path = Some(path.display().to_string());
                    }
                }
                ui.add_space(5.);
                if let Some(f) = &data.file_path {
                    ui.horizontal(|ui| {
                        ui.label("Selected file: ");
                        ScrollArea::horizontal().show(ui, |ui| {
                            ui.label(f);
                        });
                    });
                }
            });
        });

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => {
                    match e {
                        CommandError::InvalidFile => ui.label("Invalid file"),
                        CommandError::EmptyName => ui.label("Empty name"),
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
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match &data.file_path {
                        Some(f) => {
                            if f.is_empty() {
                                data.status = CommandStatus::Error(CommandError::EmptyName);
                            } else {
                                match self.project.open_surface_from_file(f.to_string(), data.surface_name.to_string()) {
                                    Err(_) => data.status = CommandStatus::Error(CommandError::InvalidFile),
                                    _ => data.status = CommandStatus::Complete,
                                }
                            }
                        },
                        None => data.status = CommandStatus::Error(CommandError::NoFile),
                    }
                }
            }
        });
    }

    fn ui_slbl_exact(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Generate a Surface from an Exact SLBL").heading();
        let ProjectCommand::SlblExact(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add_space(10.);
                ui.label("Use this command to construct a new using using the SLBL matrix method.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(15.);
                let text_first = "First point at ".to_string() + self.project.dem.dem.x[data.first_pnt].to_string().as_str() + "m";
                let text_last = "Last point at ".to_string() + self.project.dem.dem.x[data.last_pnt].to_string().as_str() + "m";
                ui.add(egui::Slider::new(&mut data.first_pnt, 0..=(data.last_pnt - 1)).text(text_first));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.last_pnt, (data.first_pnt + 1)..=(self.project.dem.dem.x.len() - 1)).text(text_last));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.tol, 0.0..=100.0).text("Tolerance"));
            });
        });

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => {
                    match e {
                        CommandError::MethodError => {ui.label("The method cannot perform with the given parameters.");},
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match self.project.surface_from_exact_slbl(data.first_pnt, data.last_pnt, data.tol) {
                        Err(_) => data.status = CommandStatus::Error(CommandError::MethodError),
                        Ok(_) => data.status = CommandStatus::Complete,
                    }
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
                ui.add_space(10.);
                ui.label("Use this command to create a surface from the SLBL routine.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);
                let text_first = "First point at ".to_string() + self.project.dem.dem.x[data.first_pnt].to_string().as_str() + "m";
                let text_last = "Last point at ".to_string() + self.project.dem.dem.x[data.last_pnt].to_string().as_str() + "m";
                ui.add(egui::Slider::new(&mut data.first_pnt, 0..=(data.last_pnt - 1)).text(text_first));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.last_pnt, (data.first_pnt + 1)..=(self.project.dem.dem.x.len() - 1)).text(text_last));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.tol, 0.0..=100.0).text("Tolerance"));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.n_it, 10..=2000).text("Iterations number"));
                ui.add_space(5.);

                ui.horizontal(|ui| {
                    ui.label("Minimum elevation (m): ");
                    ui.add(egui::DragValue::new(&mut data.min_elev));
                });

                ui.add_space(5.);

                ui.horizontal(|ui| {
                    ui.label("Maximum slope (°)");
                    ui.add(egui::DragValue::new(&mut data.max_slope));
                });
            });
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => {
                    match e {
                        CommandError::MethodError => {ui.label("The method cannot perform with the given parameters.");},
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match self.project.surface_from_routine_slbl(data.first_pnt, data.last_pnt, data.tol, data.n_it, data.min_elev, data.max_slope) {
                        Err(e) => data.status = CommandStatus::Error(CommandError::MethodError),
                        Ok(_) => data.status = CommandStatus::Complete,
                    }
                }
            }
        });
    }

    fn ui_surface_min(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Minimum Values").heading();
        let ProjectCommand::SurfaceMin(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        let nb_surf = self.project.surfaces.len();

        if nb_surf >= 2 {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    ui.add_space(10.);
                    ui.label("Use this command to construct a surface by taking the minimum elevation at each point of two surfaces.");
                    ui.add_space(5.);
                    ui.separator();
                    ui.add_space(5.);
                    egui::ComboBox::from_label("First surface")
                        .selected_text(self.project.surfaces[data.first_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.second_surface_index) {
                            ui.selectable_value(&mut data.first_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });
                    ui.add_space(5.);
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

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => match e {
                    CommandError::MethodError => {ui.label("Method error");},
                    _ => (),
                }
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match self.project.surface_from_min(data.first_surface_index, data.second_surface_index) {
                        Err(_) => data.status = CommandStatus::Error(CommandError::MethodError),
                        Ok(_) => data.status = CommandStatus::Complete,
                    }
                }
            }
        });
    }

    fn ui_surface_max(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Define a Surface using Maximum Values").heading();
        let ProjectCommand::SurfaceMax(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        let nb_surf = self.project.surfaces.len();

        if nb_surf >= 2 {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.separator();
                    ui.add_space(10.);
                    ui.label("Use this command to construct a surface by taking the maximum elevation at each point of two surfaces.");
                    ui.add_space(5.);
                    ui.separator();
                    ui.add_space(5.);
                    egui::ComboBox::from_label("First surface")
                        .selected_text(self.project.surfaces[data.first_surface_index].name.clone())
                        .show_ui(ui, |ui| {
                        for k in (0..nb_surf).filter(|i| *i != data.second_surface_index) {
                            ui.selectable_value(&mut data.first_surface_index, k, self.project.surfaces[k].name.clone());
                        }
                    });
                    ui.add_space(5.);
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

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => match e {
                    CommandError::MethodError => {ui.label("Method error");},
                    _ => (),
                }
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
            if apply_button.clicked() {
                if data.status != CommandStatus::Clean {
                    data.status = CommandStatus::Clean;
                } else {
                    match self.project.surface_from_max(data.first_surface_index, data.second_surface_index) {
                        Err(_) => data.status = CommandStatus::Error(CommandError::MethodError),
                        Ok(_) => data.status = CommandStatus::Complete,
                    }
                }
            }
        });
    }

    fn ui_model_new(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Create a New model from surfaces combination").heading();
        let ProjectCommand::ModelNew(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add_space(10.);
                ui.label("Use this command to create a displacement model using one or multiple surfaces.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                if self.project.surfaces.is_empty() {
                    ui.label("No surfaces to use");
                } else {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut data.name);
                    });
                    ui.add_space(5.);
                    for k in 0..data.surface_params.len() {
                        ui.separator();
                        ui.push_id(k, |ui| {
                            egui::ComboBox::from_label("Surface")
                                .selected_text(self.project.surfaces[data.surface_params[k].index].name.to_owned())
                                .show_ui(ui, |ui| {
                                    for s in 0..self.project.surfaces.len() {
                                        ui.selectable_value(&mut data.surface_params[k].index, s, self.project.surfaces[s].name.to_owned());
                                    }
                            });
                            ui.add_space(5.);
                            ui.horizontal(|ui| {
                                ui.label("First point: ");
                                ui.add(egui::DragValue::new(&mut data.surface_params[k].boundaries.0).range(1..=(self.project.dem.dem.x.len() - 3)));
                                ui.label(self.project.dem.dem.x[data.surface_params[k].boundaries.0].to_string() + "m");
                            });
                            ui.add_space(2.);
                            ui.horizontal(|ui| {
                                ui.label("Last point: ");
                                ui.add(egui::DragValue::new(&mut data.surface_params[k].boundaries.1).range(2..=(self.project.dem.dem.x.len() - 2)));
                                ui.label(self.project.dem.dem.x[data.surface_params[k].boundaries.1].to_string() + "m");
                            });
                            ui.add_space(5.);
                            ui.horizontal(|ui| {
                                ui.label("Weight: ");
                                ui.add(egui::DragValue::new(&mut data.surface_params[k].weight));
                            });
                            ui.add_space(5.);
                            egui::CollapsingHeader::new("Gradient").show(ui, |ui| {
                                for i in 0..data.surface_params[k].gradient_points.len() {
                                    ui.push_id(i, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("point: ");
                                            ui.add(egui::DragValue::new(&mut data.surface_params[k].gradient_points[i].0).range(0..=self.project.dem.dem.x.len()));
                                            ui.label("factor: ");
                                            ui.add(egui::DragValue::new(&mut data.surface_params[k].gradient_points[i].1).range(-1000.0..=1000.0));
                                        });
                                    });
                                }
                                ui.horizontal(|ui| {
                                    if ui.button("+").clicked() {
                                        data.surface_params[k].gradient_points.push((0, 1.));
                                    }
                                    if ui.button("-").clicked() {
                                        data.surface_params[k].gradient_points.pop();
                                    }
                                });
                            });
                        });
                    }
                    ui.add_space(5.);
                    ui.separator();
                    ui.add_space(5.);
                    ui.horizontal(|ui| {
                        if ui.button("Add surface").clicked() {
                            data.surface_params.push(SurfaceParams::default());
                        }
                        if ui.button("Remove surface").clicked() {
                            data.surface_params.pop();
                        }
                    });
                }
            });
        });

        ui.add_space(10.);

        if !self.project.surfaces.is_empty() {
            ui.add_space(10.);

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                match &data.status {
                    CommandStatus::Error(e) => match e {
                        CommandError::MethodError => {ui.label("Method error");},
                        CommandError::InputError => {ui.label("Please recheck your inputs");},
                        _ => (),
                    }
                    _ => (),
                }
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                let apply_text= match data.status {
                    CommandStatus::Clean => egui::RichText::new("Apply"),
                    CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                    CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
                };
                let apply_button= ui.button(apply_text.size(22.));
                
                if apply_button.clicked() {
                    if data.status != CommandStatus::Clean {
                        data.status = CommandStatus::Clean;
                    } else {
                        let mut all_surf_diff = true;
                        let mut all_index_ordered = true;
                        let mut all_no_grad_dupli = true;
                        for s in data.surface_params.clone() {
                            if s.boundaries.0 >= s.boundaries.1 {
                                all_index_ordered = false;
                            }
                            if !is_all_diff(&s.gradient_points.iter().map(|g| g.0).collect()) {
                                all_no_grad_dupli = false;
                            }
                        }
                        if !is_all_diff(&data.surface_params.iter().map(|s| s.index).collect()) {
                            all_surf_diff = false;
                        }

                        if !all_surf_diff | !all_index_ordered | !all_no_grad_dupli {
                            data.status = CommandStatus::Error(CommandError::InputError);
                        } else {
                            match self.project.combine_unit_models(&data.name, &data.surface_params) {
                                Err(_) => data.status = CommandStatus::Error(CommandError::MethodError),
                                _ => data.status = CommandStatus::Complete,
                            };
                        }
                    }
                }
            });
        }
    }

    fn ui_sat_geometry(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Set the Satellite Geometry").heading();
        let ProjectCommand::SatGeometry(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
            ui.vertical(|ui| {
                ui.label(title);
                ui.separator();
                ui.add_space(10.);
                ui.label("Use this command to define a new satellite geometry.");
                ui.label("The azimuth angle is the azimuth of the LOS, i.e. 90°+heading of the satellite.");
                ui.add_space(5.);
                ui.separator();
                ui.add_space(15.);
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut data.name);
                });
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.azimuth, 0.0..=359.99).text("LOS azimuth"));
                ui.add_space(5.);
                ui.add(egui::Slider::new(&mut data.incidence, 0.0..=90.).text("LOS incidence"));
            });
        });

        ui.add_space(10.);

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
            match &data.status {
                CommandStatus::Error(e) => match e {
                    CommandError::MiscError => {ui.label("An error occured");},
                    _ => (),
                }
                _ => (),
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            let apply_text= match data.status {
                CommandStatus::Clean => egui::RichText::new("Apply"),
                CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
            };
            let apply_button= ui.button(apply_text.size(22.));
            
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
        let title = egui::RichText::new("Load Satellite Displacement Data").heading();
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
                    ui.add_space(10.);
                    ui.label("Use this command to load displacement data from file.");
                    ui.label("The file should be a csv file with the header 'x' for the sampling values and 'disp' for the displacement values");
                    ui.add_space(5.);
                    ui.separator();
                    ui.add_space(15.);
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut data.name);
                    });
                    egui::ComboBox::from_label("With geometry")
                        .selected_text(self.project.sars[data.sar_index].name.to_owned())
                        .show_ui(ui, |ui| {
                            for k in (0..self.project.sars.len()) {
                                ui.selectable_value(&mut data.sar_index, k, self.project.sars[k].name.to_owned());
                            }
                    });
                    ui.add_space(5.);
                    if ui.button("Select file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            data.file_path = Some(path.display().to_string());
                        }
                    }
                    ui.add_space(5.);
                    if let Some(f) = &data.file_path {
                        ui.horizontal(|ui| {
                            ui.label("Selected file: ");
                            ScrollArea::horizontal().show(ui, |ui| {
                                ui.label(f);
                            });
                        });
                    }
                });
            });

            ui.add_space(10.);

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                match &data.status {
                    CommandStatus::Error(e) => match e {
                        CommandError::MethodError => {ui.label("An error occured");},
                        _ => (),
                    }
                    _ => (),
                }
            });
    
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                let apply_text= match data.status {
                    CommandStatus::Clean => egui::RichText::new("Apply"),
                    CommandStatus::Complete => egui::RichText::new(Phosphor::CHECK),
                    CommandStatus::Error(_) => egui::RichText::new(Phosphor::WARNING),
                };
                let apply_button= ui.button(apply_text.size(22.));
                
                if apply_button.clicked() {
                    if data.status != CommandStatus::Clean {
                        data.status = CommandStatus::Clean;
                    } else {
                        match &data.file_path {
                            Some(f) => {
                                match self.project.new_sar_data(&data.name, data.sar_index, f.to_string()) {
                                    Err(_) => data.status = CommandStatus::Error(CommandError::MethodError),
                                    _ => data.status = CommandStatus::Complete,
                                }
                            },
                            None => data.status = CommandStatus::Error(CommandError::NoFile),
                        }
                    }
                }
            });
        }        
    }

    fn ui_calibrate_model(&mut self, ui: &mut egui::Ui) {
        let title = egui::RichText::new("Calibrate a Model using Displacement Data").heading();
        let ProjectCommand::CalibrateModel(data) = &mut self.current_command else {
            panic!("Wrong intern command assignation. Please report it if raised.") // Should never reach
        };

        if !self.project.sars.is_empty() & !self.project.models.is_empty() & !self.project.dem.section_geometry.is_none() {
            let sars: Vec<&BundleSar> = self.project.sars.iter().filter(|sar| !sar.disp_data.is_empty()).collect();
            if !sars.is_empty() {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(true), |ui| {
                    ui.vertical(|ui| {
                        ui.label(title);
                        ui.separator();
                        ui.add_space(10.);
                        ui.label("Use this command to calibrate a model using InSAR data.");
                        ui.add_space(5.);
                        ui.separator();
                        ui.add_space(15.);
                        egui::ComboBox::from_label("On model")
                            .selected_text(self.project.models[data.model].name.to_string())
                            .show_ui(ui, |ui| {
                                for k in 0..self.project.models.len() {
                                    ui.selectable_value(&mut data.model, k, self.project.models[k].name.to_string());
                                }
                            });
                        ui.add_space(10.);
                        egui::ComboBox::from_label("With geometry")
                            .selected_text(self.project.sars[data.sar_geom].name.to_string())
                            .show_ui(ui, |ui| {
                                for k in 0..self.project.sars.len() {
                                    ui.selectable_value(&mut data.sar_geom, k, self.project.sars[k].name.to_string());
                                }
                            });
                        if !self.project.sars[data.sar_geom].disp_data.is_empty() {
                            egui::ComboBox::from_label("With data")
                                .selected_text(self.project.sars[data.sar_geom].disp_data[data.sar_data].name.to_string())
                                .show_ui(ui, |ui| {
                                    for k in 0..self.project.sars[data.sar_geom].disp_data.len() {
                                        ui.selectable_value(&mut data.sar_data, k, self.project.sars[data.sar_geom].disp_data[k].name.to_string());
                                    }
                                });
                        }
                    });
                });
        
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                    match &data.status {
                        CommandStatus::Error(e) => match e {
                            CommandError::EmptySar => {ui.label("No data in selected sar geometry.");},
                            CommandError::MethodError => {ui.label("An error occured with the solver.");},
                            _ => (),
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
                    let apply_button= ui.button(apply_text.size(22.));
                    
                    if apply_button.clicked() {
                        if data.status != CommandStatus::Clean {
                            data.status = CommandStatus::Clean;
                        } else {
                            if self.project.sars[data.sar_geom].disp_data.is_empty() {
                                data.status = CommandStatus::Error(CommandError::EmptySar);
                            } else {
                                match self.project.calibrate_model(data.model, data.sar_geom, data.sar_data) {
                                    Err(e) => data.status = CommandStatus::Error(CommandError::MethodError),
                                    Ok(_) => data.status = CommandStatus::Complete,
                                };
                            }
                        }
                    }
                });
            } else {
                ui.label("No available data in sar geometry.");
            }
        } else {
            ui.label("No model or sar geometry or dem geometry available available.");
        }
    }

    fn ui_view_surface(&mut self, ui: &mut egui::Ui) {
        // Surface data
        // color
    }

    fn ui_view_model(&mut self, ui: &mut egui::Ui) {
        // Model data
        // metric compared to sar data (RMSE)
        // color
    }
}

/// from https://sts10.github.io/2019/06/06/is-all-equal-function.html
fn is_all_diff(vec: &Vec<usize>) -> bool {
    vec.iter()
        .fold((true, None), {
            |acc, elem| {
                if let Some(prev) = acc.1 {
                    (acc.0 && (prev != elem), Some(elem))
                } else {
                    (true, Some(elem))
                }
            }
        })
        .0
}