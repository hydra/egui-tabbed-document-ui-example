use eframe::App;
use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};
use crate::TemplateApp;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct DocumentTab {
    path: String,
}

impl Tab<TemplateApp> for DocumentTab {
    fn label(&self) -> WidgetText {
        let title = format!("{:?}", self.path);
        egui::widget_text::WidgetText::from(title)
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, app: &mut TemplateApp) {
        ui.label(format!("path: {:?}", self.path));
    }
}
