use crate::app::tabs::{Tab, TabKey};
use egui::{FontFamily, Label, RichText, Ui, Widget, WidgetText};
use egui_flex::{Flex, FlexAlign, FlexDirection, FlexItem, FlexJustify};
use egui_i18n::tr;
use egui_material_icons::icons::ICON_HOME;
use serde::{Deserialize, Serialize};
use crate::app::Config;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {
    show_on_startup: bool,
}

impl Tab for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(tr!("home-tab-label"))
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey) {
        let frame = egui::frame::Frame::group(ui.style());

        Flex::new()
            .justify(FlexJustify::Center)
            .direction(FlexDirection::Vertical)
            .h_full()
            .w_full()
            .show(ui, |outer_flex| {
                Flex::new()
                    .justify(FlexJustify::Center)
                    .w_full()
                    .show_in(outer_flex, FlexItem::new(), |flex| {
                        flex.add_ui(
                            FlexItem::new()
                                // causes the box of the frame to shrink to the content
                                .align_self(FlexAlign::Center)
                                .frame(frame),
                            |ui| {
                                Label::new(
                                    RichText::new(ICON_HOME)
                                        .size(48.0)
                                        .family(FontFamily::Proportional),
                                )
                                    .ui(ui);

                                ui.label(RichText::new(tr!("home-heading")).size(48.0));
                            },
                        );
                    });

                Flex::new()
                    .justify(FlexJustify::Center)
                    .w_full()
                    .show_in(outer_flex, FlexItem::new()
                        .align_self(FlexAlign::Center), |flex| {
                        flex.add_ui(FlexItem::new(), |ui|{
                            // FIXME need to access config from application state here
                            let config: &mut Config = todo!();
                            ui.checkbox(&mut config.show_home_tab_on_startup, tr!("home-tab-show-on-startup"));
                        });
                    });
            });


    }
}
