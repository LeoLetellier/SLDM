use eframe::egui;
use super::View;
use crate::project::types;
use egui_phosphor::regular::{self as Phosphor, HEAD_CIRCUIT};
use egui_plot::{Line, Plot, Points, Arrows};

#[derive(Debug, Default)]
pub(crate) struct Viewer {
    is_plot_prop: bool,
    viewer_section: ViewerSection,
    viewer_properties: ViewerProperties,
}

impl View for Viewer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.is_plot_prop {
                self.viewer_properties.ui(ui);
            } else {
                self.viewer_section.ui(ui);
            }
            ui.separator();
            
            ui.vertical(|ui| {
                let section_button = egui::RichText::new(Phosphor::CHART_LINE).size(32.);
                let prop_button = egui::RichText::new(Phosphor::DOTS_SIX).size(32.);
                if ui.button(section_button).on_hover_text("Section").clicked() {
                    self.is_plot_prop = true;
                }
                if ui.button(prop_button).on_hover_text("Properties").clicked() {
                    self.is_plot_prop = false;
                }
            });
        });
    }
}

#[derive(Debug, Default)]
struct ViewerSection {
    dem: types::Dem1D,
    surfaces: Vec<types::Surface1D>,
    disp_model: Vec<types::DispProfile>,
    disp_data: Vec<types::DispProfile>,
}

impl ViewerSection {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let data = vec![[0., 2.], [1., 1.], [2., 3.]];
        let line = Line::new(data);
        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}

#[derive(Debug, Default)]
struct ViewerProperties {}

impl ViewerProperties {
    fn ui(&mut self, ui: &mut egui::Ui) {
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