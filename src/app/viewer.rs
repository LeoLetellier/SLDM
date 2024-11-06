use std::borrow::Borrow;

use eframe::egui;
use super::AppDM;
use egui_phosphor::regular::{self as Phosphor, HEAD_CIRCUIT};
use egui_plot::{Line, Plot, Points, Arrows};
use src_logic::types::*;

impl AppDM {
    pub(super) fn ui_viewer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.is_viewer_properties {
                self.ui_viewer_properties(ui);
            } else {
                self.ui_viewer_section(ui);
            }
            ui.separator();
            
            ui.vertical(|ui| {
                let section_button = egui::RichText::new(Phosphor::CHART_LINE).size(32.);
                let prop_button = egui::RichText::new(Phosphor::DOTS_SIX).size(32.);
                let button_section = ui.button(section_button);
                ui.separator();
                let button_properties = ui.button(prop_button);

                if self.is_viewer_properties {
                    button_properties.to_owned().highlight();
                } else {
                    button_section.to_owned().highlight();
                }

                if button_section.on_hover_text("Section").clicked() {
                    self.is_viewer_properties = false;
                }
                if button_properties.on_hover_text("Properties").clicked() {
                    self.is_viewer_properties = true;
                }
            });
        });
    }

    fn ui_viewer_section(&mut self, ui: &mut egui::Ui) {
        let mut lines: Vec<Line> = vec![];

        if !self.project.surfaces.is_empty() {
            for surf in &self.project.surfaces {
                if !surf.surface.z.is_empty() & surf.section_surface {
                    let line = Line::new({
                        let mut data = Vec::with_capacity(self.project.dem.dem.surface.z.len());
                        for (a, b) in self.project.dem.dem.x.iter().zip(surf.surface.z.iter()) {
                            data.push([*a as f64, *b as f64]);
                        }
                        data
                    });
                    lines.push(line);
                }
            }
        }
        
        if !self.project.dem.dem.x.is_empty() & self.project.dem.section_surface {
            let line = Line::new(
                {
                    let mut data = Vec::with_capacity(self.project.dem.dem.surface.z.len());
                    for (a, b) in self.project.dem.dem.x.iter().zip(self.project.dem.dem.surface.z.iter()) {
                        data.push([*a as f64, *b as f64]);
                    }
                    data
                }
            );
            lines.push(line);
        }

        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
        });
    }

    fn ui_viewer_properties(&mut self, ui: &mut egui::Ui) {
        let data = vec![[0., 2.], [1., 1.], [2., 1.], [3., 2.]];
        let tips = vec![[1., 3.], [2., 2.], [3., 2.], [4., 3.]];
        let line = Line::new(data.to_owned());
        let arrows = Arrows::new(data.to_owned(), tips);
        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.arrows(arrows);
            });
    }
}
