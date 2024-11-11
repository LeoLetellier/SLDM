use eframe::egui;
use super::AppDM;
use egui_phosphor::regular as Phosphor;
use egui_plot::{Line, Plot, Points, Arrows};
use src_logic::prelude::*;

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
        let mut surface_lines: Vec<Line> = vec![];
        let mut arrows: Vec<Arrows> = vec![];
        let mut dem_line: Vec<Line> = vec![];
        let mut pillar_lines: Vec<Line> = vec![];
        let x_len = self.project.dem.dem.x.len();

        if !self.project.surfaces.is_empty() {
            for surf in &self.project.surfaces {
                // Plot surface elevation
                if !surf.surface.z.is_empty() & surf.section_surface {
                    let line = Line::new({
                        let mut data = Vec::with_capacity(x_len);
                        for (a, b) in self.project.dem.dem.x.iter().zip(surf.surface.z.iter()) {
                            data.push([*a as f64, *b as f64]);
                        }
                        data
                    });
                    surface_lines.push(line);
                }
                // Plot surface unit model vectors
                if !surf.profile.vecs.is_empty() & surf.section_arrow {
                    let mut base = Vec::with_capacity(surf.profile.vecs.len());
                    let mut tip = Vec::with_capacity(surf.profile.vecs.len());
                    (0..surf.profile.vecs.len())
                        .filter(|k| surf.profile.vecs[*k].amplitude() != 0.)
                        .for_each(|k| {
                            let coords = surf.profile.vecs[k].coords();
                            let origins = surf.profile.origins[k];
                            base.push([origins[0] as f64, origins[1] as f64]);
                            tip.push(
                                [(origins[0] + coords.0 * surf.arrow_scaling_factor) as f64,
                                (origins[1] + coords.1 * surf.arrow_scaling_factor) as f64]
                            );
                        });
                    arrows.push(Arrows::new(base, tip));
                }
                // Plot surface unit model pillars
                if !surf.profile.vecs.is_empty() & surf.section_pillar {
                    for k in 0..surf.profile.origins.len() {
                        let x = self.project.dem.dem.x[k];
                        let z = surf.surface.z[k];
                        let x_pillar = surf.profile.origins[k][0];
                        let z_pillar = surf.profile.origins[k][1];
                        pillar_lines.push(Line::new(vec![[x as f64, z as f64], [x_pillar as f64, z_pillar as f64]]));
                    }
                }
            }
        }
        // Plot combinated model vectors
        if !self.project.models.is_empty() {
            for model in &self.project.models {
                if model.section_arrow {
                    let mut base = Vec::with_capacity(x_len);
                    let mut tip = Vec::with_capacity(x_len);
                    (0..x_len)
                        .filter(|k| model.resulting_profile.vecs[*k].amplitude() != 0.)
                        .for_each(|k| {
                            let coords = model.resulting_profile.vecs[k].coords();
                            let origins = model.resulting_profile.origins[k];
                            base.push([origins[0] as f64, origins[1] as f64]);
                            tip.push(
                                [(origins[0] + coords.0 * model.arrow_scaling_factor) as f64, 
                                (origins[1] + coords.1 * model.arrow_scaling_factor) as f64]
                            );
                        });
                        arrows.push(Arrows::new(base, tip));
                }
            }
        }
        // Plot DEM elevation
        if !self.project.dem.dem.x.is_empty() & self.project.dem.section_surface {
            let line = Line::new(
                {
                    let mut data = Vec::with_capacity(x_len);
                    for (a, b) in self.project.dem.dem.x.iter().zip(self.project.dem.dem.surface.z.iter()) {
                        data.push([*a as f64, *b as f64]);
                    }
                    data
                }
            );
            dem_line.push(line);
        }

        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .show(ui, |plot_ui| {
                for line in pillar_lines {
                    plot_ui.line(line);
                }
                for arrow in arrows {
                    plot_ui.arrows(arrow);
                }
                for line in surface_lines {
                    plot_ui.line(line);
                }
                for line in dem_line {
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
