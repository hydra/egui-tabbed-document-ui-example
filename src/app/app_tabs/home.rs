use crate::app::tabs::{Tab, TabKey};
use egui::{Checkbox, FontFamily, RichText, Ui, WidgetText};
//use egui_flex::{item, Flex, FlexAlign, FlexDirection, FlexItem, FlexJustify};
use crate::context::Context;
use egui_i18n::tr;
use egui_material_icons::icons::ICON_HOME;
use egui_taffy::taffy::prelude::{length, percent};
use egui_taffy::taffy::Style;
use egui_taffy::{taffy, tui, TuiBuilderLogic};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct HomeTab {
    show_on_startup: bool,
}

impl<'a> Tab<Context<'a>> for HomeTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(tr!("home-tab-label"))
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, context: &mut Context<'a>) {
        ui.ctx().style_mut(|style| {
            // if this is not done, text in labels/checkboxes/etc wraps
            style.wrap_mode = Some(egui::TextWrapMode::Extend);
        });

        let default_style = || Style {
            padding: length(8.),
            gap: length(8.),
            ..Default::default()
        };

        tui(ui, ui.id().with("home"))
            .reserve_available_space()
            .style(Style {
                justify_content: Some(taffy::JustifyContent::Center),
                align_items: Some(taffy::AlignItems::Center),
                flex_direction: taffy::FlexDirection::Column,
                size: taffy::Size {
                    width: percent(1.),
                    height: percent(1.),
                },
                ..default_style()
            })
            .show(|tui| {
                tui.style(Style {
                    flex_direction: taffy::FlexDirection::Row,
                    //align_self: Some(taffy::AlignItems::Center),
                    ..default_style()
                })
                .add_with_border(|tui| {
                    tui.label(
                        RichText::new(ICON_HOME)
                            .size(48.0)
                            .family(FontFamily::Proportional),
                    );
                    tui.label(
                        RichText::new(tr!("home-heading"))
                            .size(48.0)
                            .family(FontFamily::Proportional),
                    );
                });

                tui.ui(|ui| {
                    ui.add(Checkbox::new(
                        &mut context.config.show_home_tab_on_startup,
                        tr!("home-tab-show-on-startup"),
                    ));
                });
            });
    }
}
