use eframe::egui;
use egui::Visuals;
use super::AppDM;
use egui_phosphor::regular as Phosphor;
use crate::components::documentation;

#[derive(Debug, Default, PartialEq)]
pub(crate) enum Panel {
    #[default]
    Explorer,
    Command,
    Documentation,
}

impl AppDM {
    pub(super) fn ui_panel_content(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                match self.current_panel {
                    Panel::Explorer => {
                        self.ui_explorer(ui);
                    },
                    Panel::Command => {
                        self.ui_command(ui);
                    },
                    Panel::Documentation => {
                        self.ui_documentation(ui);
                    },
                };
            });
        });
    }

    pub(super) fn ui_panel_header(&mut self, ui: &mut egui::Ui) {
        let icon_light = egui::RichText::new(Phosphor::SUN).size(32.).strong();
        let icon_light2 = egui::RichText::new(Phosphor::SUN_DIM).size(32.).strong();
        let icon_dark = egui::RichText::new(Phosphor::MOON).size(32.).strong();
        let icon_dark2 = egui::RichText::new(Phosphor::MOON_STARS).size(32.).strong();
        let icon_explorer = egui::RichText::new(Phosphor::TREE_VIEW).size(32.).strong();
        let icon_command = egui::RichText::new(Phosphor::QUEUE).size(32.).strong();
        let icon_documentation = egui::RichText::new(Phosphor::FILE_DOC).size(32.).strong();

        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center) ,|ui| {
                let button_explorer = ui.button(icon_explorer).on_hover_text("Explorer");
                ui.separator();
                let button_command = ui.button(icon_command).on_hover_text("Command");
                ui.separator();
                let button_documentation = ui.button(icon_documentation).on_hover_text("Documentation");

                if button_explorer.clicked() {
                    if self.current_panel == Panel::Explorer {
                        self.show_panel = !self.show_panel;
                    } else {
                        self.current_panel = Panel::Explorer;
                        self.show_panel = true;
                    }
                }
                if button_command.clicked() {
                    if self.current_panel == Panel::Command {
                        self.show_panel = !self.show_panel;
                    } else {
                        self.current_panel = Panel::Command;
                        self.show_panel = true;
                    }
                }
                if button_documentation.clicked() {
                    if self.current_panel == Panel::Documentation {
                        self.show_panel = !self.show_panel;
                    } else {
                        self.current_panel = Panel::Documentation;
                        self.show_panel = true;
                    }
                }

                match self.current_panel {
                    Panel::Explorer => {button_explorer.highlight();},
                    Panel::Command => {button_command.highlight();},
                    Panel::Documentation => {button_documentation.highlight();},
                    _ => (),
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                let icon_theme;
                if self.is_light_mode {
                    ui.ctx().set_visuals(Visuals::light());
                    icon_theme = icon_dark.to_owned();
                } else {
                    ui.ctx().set_visuals(Visuals::dark());
                    icon_theme = icon_light.to_owned();
                }

                let button_theme = ui.button(icon_theme);
                if button_theme.clicked() {
                    self.is_light_mode = !self.is_light_mode;
                }

                button_theme.on_hover_text("Light/Dark Theme");
            });
        });
    }
}
