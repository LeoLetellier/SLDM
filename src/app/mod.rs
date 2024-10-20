mod context_menu;
mod action_panel;
mod viewer;

use context_menu::ContextMenus;
use action_panel::ActionPanel;
use egui::vec2;
use viewer::Viewer;

#[derive(Debug, Default)]
pub(crate) struct AppDM {
    context_menu: ContextMenus,
    action_panel: ActionPanel,
    viewer: Viewer,
}

impl eframe::App for AppDM {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("context_menu")
            .exact_height(40.)
            .show_separator_line(false)
            .show(ctx, |ui| {
                self.context_menu.ui(ui);
            });
        egui::SidePanel::left("header_panel")
            .default_width(57.)
            .resizable(false)
            .show(ctx, |ui| {
                self.action_panel.ui_panel(ui);
            });
        egui::SidePanel::left("action_panel")
            .resizable(true)
            .show(ctx, |ui| {
                self.action_panel.ui(ui);
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
                self.viewer.ui(ui);
        });
        
        // egui::Window::new("wild")
        //     .default_width(600.)
        //     .default_height(400.)
        //     .vscroll(true)
        //     .show(ctx, |ui| ui.label("some text"));
    }
}

impl AppDM {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }
}

trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
