use super::{AppDM, ProjectCommand};
use eframe::egui;
use egui_phosphor::regular as Phosphor;
use egui_plot::{Arrows, Line, Plot, PlotBounds, Points};

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
                    if !self.is_viewer_properties {
                        self.graph_bound = true;
                    } else {
                        self.is_viewer_properties = false;
                    }
                }
                if button_properties.on_hover_text("Properties").clicked() {
                    match self.current_command {
                        ProjectCommand::ModelAnalysis(_) => self.is_viewer_properties = true,
                        _ => ()
                    }
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
                    let mut line = line.name("Surface Elevation");
                    if let Some(c) = surf.color_surface {
                        line = line.color(egui::Color32::from_rgb(c[0], c[1], c[2]));
                    }
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
                            tip.push([
                                (origins[0] + coords.0 * surf.arrow_scaling_factor) as f64,
                                (origins[1] + coords.1 * surf.arrow_scaling_factor) as f64,
                            ]);
                        });
                    let mut arrow = Arrows::new(base, tip).name("Displacement Vectors");
                    if let Some(c) = surf.color_arrow {
                        arrow = arrow.color(egui::Color32::from_rgb(c[0], c[1], c[2]));
                    }
                    arrows.push(arrow);
                }
                // Plot surface unit model pillars
                if !surf.profile.vecs.is_empty() & surf.section_pillar {
                    for k in 0..surf.profile.origins.len() {
                        let x = self.project.dem.dem.x[k];
                        let z = surf.surface.z[k];
                        let x_pillar = surf.profile.origins[k][0];
                        let z_pillar = surf.profile.origins[k][1];
                        let line = Line::new(vec![
                            [x as f64, z as f64],
                            [x_pillar as f64, z_pillar as f64],
                        ]);
                        pillar_lines.push(line.color(egui::Color32::DARK_GRAY).name("Pillars"));
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
                            tip.push([
                                (origins[0] + coords.0 * model.arrow_scaling_factor) as f64,
                                (origins[1] + coords.1 * model.arrow_scaling_factor) as f64,
                            ]);
                        });
                    let mut arrow = Arrows::new(base, tip).name("Displacement Vectors");
                    if let Some(c) = model.color_arrow {
                        arrow = arrow.color(egui::Color32::from_rgb(c[0], c[1], c[2]));
                    }
                    arrows.push(arrow);
                }
            }
        }
        // Plot DEM elevation
        if !self.project.dem.dem.x.is_empty() & self.project.dem.section_surface {
            let line = Line::new({
                let mut data = Vec::with_capacity(x_len);
                for (a, b) in self
                    .project
                    .dem
                    .dem
                    .x
                    .iter()
                    .zip(self.project.dem.dem.surface.z.iter())
                {
                    data.push([*a as f64, *b as f64]);
                }
                data
            });
            dem_line.push(line.color(egui::Color32::ORANGE).width(2.).name("DEM"));
        }

        if !self.project.dem.dem.x.is_empty() & self.graph_bound {
            let min_x = self.project.dem.dem.x.first().unwrap();
            let max_x = self.project.dem.dem.x.last().unwrap();
            let amp_x = (max_x - min_x).abs();
            let mean_y = self
                .project
                .dem
                .dem
                .surface
                .z
                .iter()
                .fold(0.0, |acc, k| acc + k)
                / (self.project.dem.dem.surface.z.len() as f32);

            let bound_x_min = min_x - 0.05 * amp_x;
            let bound_x_max = max_x + 0.05 * amp_x;
            let bound_y_min =
                mean_y - (1.1 * amp_x * ui.available_height() / (ui.available_width() - 64.)) / 2.;
            let bound_y_max =
                mean_y + (1.1 * amp_x * ui.available_height() / (ui.available_width() - 64.)) / 2.;
            self.project.dem.min_bound = [bound_x_min as f64, bound_y_min as f64];
            self.project.dem.max_bound = [bound_x_max as f64, bound_y_max as f64];
        }

        let legend = match &self.project.dem.dem.surface.z {
            z if z.is_empty() => egui_plot::Legend::default(),
            z if z.last().unwrap_or(&f32::MIN) > z.first().unwrap_or(&f32::MAX) => {
                egui_plot::Legend::default().position(egui_plot::Corner::LeftTop)
            }
            _z => egui_plot::Legend::default().position(egui_plot::Corner::RightTop),
        };

        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .x_axis_label("Section (m)")
            .y_axis_label("Elevation (m)")
            .legend(legend)
            .show(ui, |plot_ui| {
                if self.graph_bound {
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                        self.project.dem.min_bound,
                        self.project.dem.max_bound,
                    ));
                    self.graph_bound = false;
                }
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
        let mut lines = vec![];
        let mut points = vec![];

        match &self.current_command {
            ProjectCommand::ModelAnalysis(data) => {
                if !data.amp_in_los.is_empty() {
                    let full_x = self.project.dem.dem.x.clone();
                    let data_x = data.data_x.to_owned();

                    let full_amp_line = Line::new(
                        full_x.iter().zip(data.full_amp.to_owned().iter())
                        .map(|(a, b)| [*a as f64, *b as f64]).collect::<Vec<[f64; 2]>>());
                    let full_amp_los_line = Line::new(
                        full_x.iter().zip(data.full_amp_in_los.to_owned().iter())
                        .map(|(a, b)| [*a as f64, *b as f64]).collect::<Vec<[f64; 2]>>());
                    lines.push(full_amp_line.name("Model Amplitude").width(2.));
                    lines.push(full_amp_los_line.name("Model in LOS").width(2.));

                    let amp_points = Points::new(
                        data_x.iter().zip(data.amp.to_owned().iter())
                        .map(|(a, b)| [*a as f64, *b as f64]).collect::<Vec<[f64; 2]>>());
                    let amp_los_points = Points::new(
                        data_x.iter().zip(data.amp_in_los.to_owned().iter())
                        .map(|(a, b)| [*a as f64, *b as f64]).collect::<Vec<[f64; 2]>>());
                    let data_points = Points::new(
                        data_x.iter().zip(data.amp_data.to_owned().iter())
                        .map(|(a, b)| [*a as f64, *b as f64]).collect::<Vec<[f64; 2]>>());
                    points.push(amp_points.name("Model Amplitude").radius(4.));
                    points.push(amp_los_points.name("Model in LOS").radius(4.));
                    points.push(data_points.name("Displacement Data").radius(5.));
                }
            },
            _ => (),
        }

        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
                for point in points {
                    plot_ui.points(point);
                }
            });
    }
}
