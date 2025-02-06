use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct DocumentTab {
    path: String,
}

impl Tab for DocumentTab {
    fn label(&self) -> WidgetText {
        let title = format!("{:?}", self.path);
        egui::widget_text::WidgetText::from(title)
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        ui.label(format!("path: {:?}", self.path));
    }
}
