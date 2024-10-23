use eframe::egui;
use super::AppDM;
use egui_phosphor::regular::{self as Phosphor, HEAD_CIRCUIT};
use egui_plot::{Line, Plot, Points, Arrows};
use src_logic::types::*;

impl AppDM {
    pub(super) fn ui_viewer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.viewer_handler.plot_section {
                self.ui_viewer_section(ui);
            } else {
                self.ui_viewer_properties(ui);
            }
            ui.separator();
            
            ui.vertical(|ui| {
                let section_button = egui::RichText::new(Phosphor::CHART_LINE).size(32.);
                let prop_button = egui::RichText::new(Phosphor::DOTS_SIX).size(32.);
                if ui.button(section_button).on_hover_text("Section").clicked() {
                    self.viewer_handler.plot_section = true;
                }
                if ui.button(prop_button).on_hover_text("Properties").clicked() {
                    self.viewer_handler.plot_section = false;
                }
            });
        });
    }

    fn ui_viewer_section(&mut self, ui: &mut egui::Ui) {
        let data = vec![[0., 2.], [1., 1.], [2., 3.]];
        let line = Line::new(data);
        Plot::new("Section plot")
            .width(ui.available_width() - 64.)
            .height(ui.available_height())
            .show(ui, |plot_ui| plot_ui.line(line));
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

#[derive(Debug)]
pub(crate) struct ViewerHandler {
    pub(crate) plot_section: bool,
}

impl Default for ViewerHandler {
    fn default() -> Self {
        Self { plot_section: true }
    }
}