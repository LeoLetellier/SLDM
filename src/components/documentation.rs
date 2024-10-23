use egui::ScrollArea;
use egui_commonmark::{commonmark_str, CommonMarkCache, CommonMarkViewer};

use crate::app::AppDM;

impl AppDM {
    pub(crate) fn ui_documentation(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let mut cache = CommonMarkCache::default();
            commonmark_str!(ui, &mut cache, "./src/documentation.md");
            CommonMarkViewer::new().show(ui, &mut cache, "");
        });
    }
}