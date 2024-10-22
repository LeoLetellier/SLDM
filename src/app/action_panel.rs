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
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.vertical(|ui| {
                match self.current_panel {
                    Panel::Settings => {
                        ui.label("Settings");
                        ui.separator();
                    },
                    Panel::Explorer => {
                        ui.label("Explorer");
                        ui.separator();
                    },
                    Panel::Command => {
                        ui.label("Command");
                        ui.separator();
                    },
                    Panel::Documentation => {
                        ui.label("Docs");
                        ui.separator();
                    },
                };
            });
        });
    }
}

impl ActionPanel {
    pub(crate) fn ui_panel(&mut self, ui: &mut egui::Ui) {
        let icon_settings = egui::RichText::new(Phosphor::GEAR).size(32.).strong();
        let icon_explorer = egui::RichText::new(Phosphor::TREE_VIEW).size(32.).strong();
        let icon_command = egui::RichText::new(Phosphor::QUEUE).size(32.).strong();
        let icon_documentation = egui::RichText::new(Phosphor::FILE_DOC).size(32.).strong();

        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center) ,|ui| {
                let button_explorer = ui.button(icon_explorer);
                ui.separator();
                let button_command = ui.button(icon_command);
                ui.separator();
                let button_documentation = ui.button(icon_documentation);

                if button_explorer.clicked() {
                    self.current_panel = Panel::Explorer;
                }
                if button_command.clicked() {
                    self.current_panel = Panel::Command;
                }
                if button_documentation.clicked() {
                    self.current_panel = Panel::Documentation;
                }

                match self.current_panel {
                    Panel::Explorer => {button_explorer.highlight();},
                    Panel::Command => {button_command.highlight();},
                    Panel::Documentation => {button_documentation.highlight();},
                    _ => (),
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                let button_settings = ui.button(icon_settings);
            
                if button_settings.clicked() {
                    self.current_panel = Panel::Settings;
                }
                
                match self.current_panel {
                    Panel::Settings => {button_settings.highlight();},
                    _ => (),
                }
            });
        });
    }
}