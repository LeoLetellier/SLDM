use egui::CollapsingHeader;

use crate::{app::AppDM, project};
use egui_phosphor::regular as Phosphor;

impl AppDM {
    pub(crate) fn ui_explorer(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("DEM")
            .default_open(true)
            .show(ui, |ui| {
                if !self.project.dem.dem.x.is_empty() {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                            ui.label("Elevation");
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.set_width(ui.available_width());
                            if self.is_viewer_properties {
                                ui.disable();
                            }
                            if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, self.project.dem.section_surface)).clicked() {
                                self.project.dem.section_surface = !self.project.dem.section_surface;
                            };
                        });
                    });
                }
            });

        ui.separator();
        
        let nb_surfaces = self.project.surfaces.len();
        CollapsingHeader::new("Surfaces (".to_string() + nb_surfaces.to_string().as_str() + ")")
            .default_open(true)
            .show(ui, |ui| {
                (0..self.project.surfaces.len()).for_each(|k| {
                    let bundle = &mut self.project.surfaces[k];
                    ui.push_id(k, |ui|{
                        CollapsingHeader::new(bundle.name.clone())
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                                        ui.label("Elevation");
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.set_width(ui.available_width());
                                        if self.is_viewer_properties {
                                            ui.disable();
                                        }
                                        if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, bundle.section_surface)).clicked() {
                                            bundle.section_surface = !bundle.section_surface;
                                        };
                                    });
                                });
                            });
                    });
                });
            });

        ui.separator();

        let nb_models = self.project.unit_models.len() + self.project.composition_models.len();
        CollapsingHeader::new("Models (".to_string() + nb_models.to_string().as_str() + ")")
            .default_open(true)
            .show(ui, |ui| {
                (0..self.project.unit_models.len()).for_each(|k| {
                    let bundle = &mut self.project.unit_models[k];
                    ui.push_id(k, |ui|{
                        CollapsingHeader::new(bundle.name.clone())
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                                        ui.label("Displacement Vectors");
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.set_width(ui.available_width());
                                        if self.is_viewer_properties {
                                            ui.disable();
                                        }
                                        if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, bundle.section_arrow)).clicked() {
                                            bundle.section_arrow = !bundle.section_arrow;
                                        };
                                    });
                                });
                                if bundle.section_arrow {
                                    ui.add(egui::Slider::new(&mut bundle.arrow_scaling_factor, 0.001..=1000.0).text("Scaling factor").logarithmic(true));
                                }
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                                        ui.label("Migration Pillars");
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.set_width(ui.available_width());
                                        if self.is_viewer_properties {
                                            ui.disable();
                                        }
                                        if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, bundle.section_pillar)).clicked() {
                                            bundle.section_pillar = !bundle.section_pillar;
                                        };
                                    });
                                });
                            });
                    });
                });
                (0..self.project.composition_models.len()).for_each(|k| {
                    let bundle = &mut self.project.composition_models[k];
                    ui.push_id(k, |ui|{
                        CollapsingHeader::new(bundle.name.clone())
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                                        ui.label("Displacement Vectors");
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.set_width(ui.available_width());
                                        if self.is_viewer_properties {
                                            ui.disable();
                                        }
                                        if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, bundle.section_arrow)).clicked() {
                                            bundle.section_arrow = !bundle.section_arrow;
                                        };
                                    });
                                });
                            });
                    });
                });
            });

        ui.separator();
        
        let nb_sat = self.project.sars.len();
        CollapsingHeader::new("Calibration data (".to_string() + nb_sat.to_string().as_str() + ")")
            .default_open(true)
            .show(ui, |ui| {

            });
    }

    fn get_display_icon(is_section: bool, is_focus_section: bool, is_displayed: bool) -> egui::RichText {
        let icon_display = egui::RichText::new(Phosphor::EYE);
        let icon_not_display = egui::RichText::new(Phosphor::EYE_SLASH);
        let icon_no_display = egui::RichText::new(Phosphor::EYE_CLOSED);

        let icon;
        if is_section ^ is_focus_section {
            icon = icon_no_display.to_owned();
        } else if is_displayed {
            icon = icon_display.to_owned();
        } else {
            icon = icon_not_display.to_owned();
        }

        icon
    }
}