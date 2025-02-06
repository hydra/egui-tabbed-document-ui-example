use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {}

impl Tab for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("Home")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        ui.label("Home");
    }
}
