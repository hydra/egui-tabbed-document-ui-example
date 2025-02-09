use crate::app::tabs::{Tab, TabKey};
use egui::{FontFamily, Label, RichText, Ui, Widget, WidgetText};
use egui_flex::{Flex, FlexAlign, FlexDirection, FlexItem, FlexJustify};
use egui_i18n::tr;
use egui_material_icons::icons::ICON_HOME;
use serde::{Deserialize, Serialize};
use crate::app::Config;
use crate::context::Context;
use crate::TemplateApp;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {
    show_on_startup: bool,
}

impl<'a> Tab<Context<'a>> for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(tr!("home-tab-label"))
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, context: &mut Context) {
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
                            ui.checkbox(&mut context.config.show_home_tab_on_startup, tr!("home-tab-show-on-startup"));
                        });
                    });
            });


    }
}
