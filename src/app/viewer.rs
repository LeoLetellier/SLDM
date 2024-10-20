use eframe::egui;
use super::View;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default)]
pub(crate) struct Viewer {}

impl View for Viewer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello World!");
            ui.label("Some label");
            let mut string = String::new();
            ui.add(egui::TextEdit::singleline(&mut string).hint_text("it's a text edit"));
    }
}