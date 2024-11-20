use super::AppDM;
use eframe::egui;
use egui_phosphor::regular as Phosphor;

#[derive(Debug, Default, PartialEq)]
pub(crate) enum Panel {
    NoPanel,
    Explorer,
    Command,
    #[default]
    Documentation,
}

impl AppDM {
    pub(super) fn ui_panel_content(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                match self.current_panel {
                    Panel::Explorer => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            self.ui_explorer(ui);
                        });
                    }
                    Panel::Command => {
                        self.ui_command(ui);
                    }
                    Panel::Documentation => {
                        self.ui_documentation(ui);
                    }
                    Panel::NoPanel => (),
                };
            });
        });
    }

    pub(super) fn ui_panel_header(&mut self, ui: &mut egui::Ui) {
        let icon_light = egui::RichText::new(Phosphor::SUN).size(32.).strong();
        // let icon_light2 = egui::RichText::new(Phosphor::SUN_DIM).size(32.).strong();
        let icon_dark = egui::RichText::new(Phosphor::MOON).size(32.).strong();
        // let icon_dark2 = egui::RichText::new(Phosphor::MOON_STARS).size(32.).strong();
        let icon_explorer = egui::RichText::new(Phosphor::TREE_VIEW).size(32.).strong();
        let icon_command = egui::RichText::new(Phosphor::QUEUE).size(32.).strong();
        let icon_documentation = egui::RichText::new(Phosphor::FILE_DOC).size(32.).strong();

        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let button_explorer = ui.button(icon_explorer).on_hover_text("Explorer");
                ui.separator();
                let button_command = ui.button(icon_command).on_hover_text("Command");
                ui.separator();
                let button_documentation =
                    ui.button(icon_documentation).on_hover_text("Documentation");

                if button_explorer.clicked() {
                    if self.current_panel == Panel::Explorer {
                        self.current_panel = Panel::NoPanel;
                    } else {
                        self.current_panel = Panel::Explorer;
                    }
                }
                if button_command.clicked() {
                    if self.current_panel == Panel::Command {
                        self.current_panel = Panel::NoPanel;
                    } else {
                        self.current_panel = Panel::Command;
                    }
                }
                if button_documentation.clicked() {
                    if self.current_panel == Panel::Documentation {
                        self.current_panel = Panel::NoPanel;
                    } else {
                        self.current_panel = Panel::Documentation;
                    }
                }

                match self.current_panel {
                    Panel::Explorer => {
                        button_explorer.highlight();
                    }
                    Panel::Command => {
                        button_command.highlight();
                    }
                    Panel::Documentation => {
                        button_documentation.highlight();
                    }
                    _ => (),
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(6.);
                let icon_theme;
                if self.is_light_mode {
                    ui.ctx().set_visuals(egui::Visuals::light());
                    icon_theme = icon_dark.to_owned();
                } else {
                    ui.ctx().set_visuals(egui::Visuals::dark());
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
