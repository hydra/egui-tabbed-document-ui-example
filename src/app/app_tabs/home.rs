use crate::app::tabs::{Tab, TabKey};
use egui::{Checkbox, FontFamily, Frame, Label, RichText, Ui, Widget, WidgetText};
use egui_flex::{item, Flex, FlexAlign, FlexDirection, FlexItem, FlexJustify};
use egui_i18n::tr;
use egui_material_icons::icons::ICON_HOME;
use serde::{Deserialize, Serialize};
use crate::context::Context;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {
    show_on_startup: bool,
}

impl<'a> Tab<Context<'a>> for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(tr!("home-tab-label"))
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, context: &mut Context<'a>) {
        let frame = egui::frame::Frame::group(ui.style());

        Flex::new()
            .justify(FlexJustify::Center)
            .direction(FlexDirection::Vertical)
            .align_items(FlexAlign::Center)
            .h_full()
            .w_full()
            .show(ui, |outer_flex| {
                outer_flex.add_flex(item().frame(frame), Flex::horizontal(), |flex| {
                    flex.add(
                        item(),
                        Label::new(
                            RichText::new(ICON_HOME)
                                .size(48.0)
                                .family(FontFamily::Proportional),
                        ),
                    );
                    flex.add(
                        item(),
                        Label::new(
                            RichText::new(tr!("home-heading"))
                                .size(48.0)
                                .family(FontFamily::Proportional),
                        ),
                    );
                });

                outer_flex.add(item(), Checkbox::new(&mut context.config.show_home_tab_on_startup, tr!("home-tab-show-on-startup")));
            });
    }
}
