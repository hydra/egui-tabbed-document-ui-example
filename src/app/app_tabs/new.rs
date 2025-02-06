use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct NewTab {
    // TODO a form
}

impl Tab for NewTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("New")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        ui.label("New");
    }
}
