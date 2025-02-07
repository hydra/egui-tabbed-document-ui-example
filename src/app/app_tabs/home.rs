use crate::app::tabs::{Tab, TabKey};
use egui::{FontFamily, Label, RichText, Ui, Widget, WidgetText};
use egui_flex::{Flex, FlexAlign, FlexItem, FlexJustify};
use egui_material_icons::icons::ICON_HOME;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {}

impl Tab for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("Home")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        let frame = egui::frame::Frame::group(ui.style());

        Flex::new()
            .justify(FlexJustify::Center)
            .h_full()
            .w_full()
            .show(ui, |flex| {

                flex.add_ui(
                    FlexItem::new()
                        // causes the box of the frame to shrink to the content
                        .align_self(FlexAlign::Center)
                        .frame(frame), |ui|
                {
                    Label::new(
                        RichText::new(ICON_HOME)
                            .size(48.0)
                            .family(FontFamily::Proportional),
                    )
                        .ui(ui);

                    ui.label(RichText::new("Home").size(48.0));
                });
            });
    }
}
