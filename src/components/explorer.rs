use egui::CollapsingHeader;

use crate::{app::AppDM, project::BundleSurface};
use egui_phosphor::regular as Phosphor;

impl AppDM {
    pub(crate) fn ui_explorer(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("DEM")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                        ui.label("Elevation");
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.set_width(ui.available_width());
                        if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, self.project.dem.section_surface)).clicked() {
                            self.project.dem.section_surface = !self.project.dem.section_surface;
                        };
                    });
                });
            });

        ui.separator();

        CollapsingHeader::new("Surfaces")
            .default_open(true)
            .show(ui, |ui| {
                (0..self.project.surfaces.len()).for_each(|k| {
                    let bundle = &mut self.project.surfaces[k];
                    CollapsingHeader::new(bundle.name.clone())
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{
                                    ui.label("Elevation");
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.set_width(ui.available_width());
                                    if ui.button(Self::get_display_icon(true, !self.is_viewer_properties, bundle.section_surface)).clicked() {
                                        bundle.section_surface = !bundle.section_surface;
                                    };
                                });
                            });
                        });
                });
            });

        ui.separator();

        CollapsingHeader::new("Models")
            .default_open(true)
            .show(ui, |ui| {

            });

        ui.separator();

        CollapsingHeader::new("Calibration data")
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