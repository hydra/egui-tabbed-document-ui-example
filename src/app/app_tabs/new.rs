use std::path::PathBuf;
use crate::app::tabs::{Tab, TabKey};
use egui::{Button, Response, TextEdit, Ui, Widget, WidgetText};
use egui_i18n::tr;
use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent};
use egui_taffy::taffy::{AlignContent, AlignItems, AlignSelf, Display, FlexDirection, Style};
use egui_taffy::{taffy, tui, TuiBuilderLogic, TuiContainerResponse};
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::file_picker::Picker;

mod colors {
    use egui::Color32;

    pub const ERROR: Color32 = Color32::from_rgb(0xcb, 0x63, 0x5d);
}

#[derive(Default, Deserialize, Serialize)]
pub struct NewTab {
    fields: NewTabForm,

    #[serde(skip)]
    file_picker: Picker,
}

// FIXME form errors do not use i18n
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct NewTabForm {
    name: String,

    kind: Option<NewDocumentKind>,

    directory: Option<PathBuf>
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
enum NewDocumentKind {
    Text,
    Image,
}

impl<'a> Tab<Context<'a>> for NewTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("New")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, _context: &mut Context<'a>) {

        if let Ok(picked_directory) = self.file_picker.picked() {
            self.fields.directory = Some(picked_directory);
        }

        ui.ctx().style_mut(|style| {
            // if this is not done, text in labels/checkboxes/etc wraps
            style.wrap_mode = Some(egui::TextWrapMode::Extend);
        });

        let default_style = || Style {
            padding: length(2.),
            gap: length(2.),
            ..Default::default()
        };

        let no_padding_style = || Style {
            padding: length(0.),
            gap: length(0.),
            ..Default::default()
        };

        tui(ui, ui.id().with("new"))
            .reserve_available_width()
            .style(Style {
                align_items: Some(AlignItems::Center),
                flex_direction: FlexDirection::Column,
                size: taffy::Size {
                    width: percent(1.),
                    height: auto(),
                },
                padding: length(8.),
                gap: length(8.),
                ..default_style()
            })
            .show(|tui| {

                //
                // form fields container
                //

                tui
                    .style(Style {
                        flex_direction: FlexDirection::Row,
                        align_self: Some(AlignSelf::Stretch),
                        ..default_style()
                    })
                    .add(|tui|{

                        //
                        // Grid container
                        //
                        tui
                            .style(Style {
                                flex_grow: 1.0,
                                display: Display::Grid,
                                grid_template_columns: vec![fit_content(percent(1.)), fr(1.)],
                                grid_template_rows: vec![fr(1.), fr(1.)],

                                // ensure items are centered vertically on rows
                                align_items: Some(AlignItems::Center),
                                ..default_style()
                            })
                            .add(|tui|{

                                //
                                // Name field
                                //
                                tui
                                    .style(Style {
                                        ..default_style()
                                    })
                                    .add(|tui| {
                                        tui.label(tr!("form-new-name"));
                                    });

                                tui
                                    .style(Style {
                                        flex_grow: 1.0,
                                        ..default_style()
                                    })
                                    .add(|tui| {
                                        // NOTE text input does not resize with grid cell when using `.ui_add`, known issue - https://discord.com/channels/900275882684477440/904461220592119849/1338883750922293319
                                        //      as a workaround we use `ui_add_manual` for now, with `no_transform`.
                                        tui
                                            .style(Style {
                                                flex_grow: 1.0,
                                                ..default_style()
                                            })
                                            .ui_add_manual(|ui| {
                                                TextEdit::singleline(&mut self.fields.name).desired_width(ui.available_width()).ui(ui)
                                            }, no_transform);
                                    });

                                //
                                // Directory field
                                //

                                tui
                                    .style(Style {
                                        ..default_style()
                                    })
                                    .add(|tui| {
                                        tui.label(tr!("form-new-directory"))
                                    });

                                    tui
                                        .style(Style {
                                            display: Display::Flex,
                                            align_content: Some(AlignContent::Stretch),
                                            flex_grow: 1.0,
                                            ..no_padding_style()
                                        })
                                        .add(|tui| {

                                            // NOTE text input does not resize with grid cell when using `.ui_add`, known issue - https://discord.com/channels/900275882684477440/904461220592119849/1338883750922293319
                                            //      as a workaround we use `ui_add_manual` for now, with `no_transform`.
                                            tui
                                                .style(Style {
                                                    flex_grow: 1.0,
                                                    ..default_style()
                                                })
                                                .ui_add_manual(|ui| {
                                                    let mut chosen_directory = self.fields.directory.as_ref().map_or("".to_string(), |path|path.display().to_string());
                                                    TextEdit::singleline(&mut chosen_directory)
                                                        .desired_width(ui.available_width())
                                                        .interactive(false)
                                                        .ui(ui)
                                                }, no_transform);

                                            if tui
                                                .style(Style {
                                                    flex_grow: 0.0,
                                                    ..default_style()
                                                })
                                                .ui_add(Button::new("..."))
                                                .clicked() {
                                                self.file_picker.pick_folder()
                                            }
                                        });

                                //
                                // Kind field
                                //

                                tui
                                    .style(Style {
                                        ..default_style()
                                    })
                                    .add(|tui| {
                                        tui.label(tr!("form-new-kind"));
                                    });

                                tui
                                    .style(Style {
                                        flex_grow: 1.0,
                                        ..default_style()
                                    })
                                    .add(|tui| {
                                        tui.ui_add_manual(|ui| {
                                            // FIXME combo box does not scale with container

                                            let available_size = ui.available_size();

                                            ui.add_sized(available_size, |ui: &mut Ui|{

                                                let kind_id = ui.id();
                                                egui::ComboBox::from_id_salt(kind_id)
                                                    .width(ui.available_width())
                                                    .selected_text(match self.fields.kind {
                                                        None => tr!("form-common-combo-default"),
                                                        Some(NewDocumentKind::Text) => tr!("form-new-kind-text"),
                                                        Some(NewDocumentKind::Image) => tr!("form-new-kind-image"),
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        if ui
                                                            .add(egui::SelectableLabel::new(
                                                                self.fields.kind == Some(NewDocumentKind::Image),
                                                                tr!("form-new-kind-image"),
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.fields.kind = Some(NewDocumentKind::Image)
                                                        }
                                                        if ui
                                                            .add(egui::SelectableLabel::new(
                                                                self.fields.kind == Some(NewDocumentKind::Text),
                                                                tr!("form-new-kind-text"),
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.fields.kind = Some(NewDocumentKind::Text)
                                                        }
                                                    }).response
                                                })
                                        }, no_transform);
                                    });
                            });
                });

                if tui
                    .style(Style {
                        ..default_style()
                    })
                    .ui_add(Button::new("Submit"))
                    .clicked() {
                    self.on_submit();
                }

            });
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }
}

fn no_transform(value: TuiContainerResponse<Response>, _ui: &Ui) -> TuiContainerResponse<Response> {
    value
}
