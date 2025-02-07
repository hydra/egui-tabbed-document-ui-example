use crate::app::tabs::{Tab, TabKey};
use egui::{FontFamily, Label, RichText, Ui, Widget, WidgetText};
use egui_material_icons::icons::ICON_HOME;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {}

impl Tab for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("Home")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        ui.horizontal(|ui| {
            Label::new(
                RichText::new(ICON_HOME)
                    .size(16.0)
                    .family(FontFamily::Proportional),
            )
                .ui(ui);

            ui.label("Home");
        });
    }
}
