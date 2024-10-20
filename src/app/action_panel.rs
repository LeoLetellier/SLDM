use eframe::egui;
use super::View;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default)]
enum Panel {
    Settings,
    #[default]
    Explorer,
    Command,
    Documentation,
}

#[derive(Debug, Default)]
pub(crate) struct ActionPanel {
    current_panel: Panel,
}

impl View for ActionPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.ui_panel(ui);
            match self.current_panel {
                Panel::Settings => {
                    ui.label("Settings");
                },
                Panel::Explorer => {
                    ui.label("Explorer");
                },
                Panel::Command => {
                    ui.label("Command");
                },
                Panel::Documentation => {
                    ui.label("Docs");
                },
            }
        });
    }
}

impl ActionPanel {
    fn ui_panel(&mut self, ui: &mut egui::Ui) {
        let icon_settings = egui::RichText::new(Phosphor::GEAR).size(32.).strong();
        let icon_explorer = egui::RichText::new(Phosphor::TREE_VIEW).size(32.).strong();
        let icon_command = egui::RichText::new(Phosphor::QUEUE).size(32.).strong();
        let icon_documentation = egui::RichText::new(Phosphor::FILE_DOC).size(32.).strong();
        
        ui.vertical(|ui| {
            if ui.button(icon_settings).clicked() {
                self.current_panel = Panel::Settings;
            }
            ui.separator();
            if ui.button(icon_explorer).clicked() {
                self.current_panel = Panel::Explorer;
            }
            ui.separator();
            if ui.button(icon_command).clicked() {
                self.current_panel = Panel::Command;
            }
            ui.separator();
            if ui.button(icon_documentation).clicked() {
                self.current_panel = Panel::Documentation;
            }
        });
    }
}