pub(crate) mod context_menu;
mod action_panel;
mod viewer;

use std::env;

use egui_phosphor::regular as Phosphor;
use crate::project::Project;
use action_panel::Panel;
use viewer::ViewerHandler;
use crate::components::command::ProjectCommand;

#[derive(Debug, Default)]
pub(crate) struct AppDM {
    pub(crate) project: Project,
    pub(crate) settings: Settings,
    pub(crate) viewer_handler: ViewerHandler,
    pub(crate) is_about_open: bool,
    pub(crate) current_panel: Panel,
    pub(crate) current_command: ProjectCommand,
}

impl eframe::App for AppDM {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("context_menu")
            .exact_height(40.)
            .show_separator_line(false)
            .show(ctx, |ui| {
                self.ui_context_menu(ui);
            });
        egui::SidePanel::left("header_panel")
            .exact_width(57.)
            .resizable(false)
            .show(ctx, |ui| {
                self.ui_panel_header(ui);
            });
        egui::SidePanel::left("action_panel")
            .resizable(true)
            .min_width(57.)
            .max_width(660.)
            .default_width(220.)
            .show(ctx, |ui| {
                self.ui_panel_content(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true), |ui| {
                self.ui_viewer(ui);
            });
        });
        egui::Window::new(egui::RichText::new(Phosphor::INFO.to_string() + " About"))
            .fixed_size([200., 500.])
            .open(&mut self.is_about_open)
            .show(ctx, |ui| {
                Self::ui_about(ui);
            });
    }
}

impl AppDM {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }

    fn ui_about(ui: &mut egui::Ui) {
        ui.label("Slow Landslide Displacement Model");
                ui.separator();
                ui.label("Version: ".to_owned() + Self::VERSION);
                ui.label("Author: ".to_owned() + Self::AUTHORS);
                ui.horizontal(|ui| {
                    ui.label("Repository: ");
                    ui.hyperlink_to("GitHub", Self::REPOSITORY);
                });
    }
}

#[derive(Debug, Default)]
pub(crate) struct Settings {}
