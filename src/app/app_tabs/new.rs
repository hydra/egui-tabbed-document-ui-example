use std::path::PathBuf;
use eframe::emath::Align2;
use eframe::epaint::FontFamily;
use eframe::glow::RED;
use crate::app::tabs::{Tab, TabKey};
use egui::{Button, Color32, Frame, Label, Response, RichText, TextEdit, Ui, Vec2, Widget, WidgetText};
// use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexDirection, FlexItem, FlexJustify};
// use egui_form::{EguiValidationReport, Form, FormField};
// use egui_form::garde::{field_path, GardeReport};
use egui_i18n::tr;
use egui_material_icons::icons::ICON_HOME;
use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent, span};
use egui_taffy::taffy::{AlignContent, AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, JustifyItems, JustifySelf, Size, Style};
use egui_taffy::{taffy, tui, Tui, TuiBuilderLogic, TuiContainerResponse};
use garde::{Path, Report, Validate};
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
#[derive(Clone, Debug, Default, Validate, Deserialize, Serialize)]
struct NewTabForm {
    #[garde(length(min = 1))]
    name: String,

    #[garde(required)]
    kind: Option<NewDocumentKind>,

    #[garde(required)]
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

        let validation_result = self.fields.validate();

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

                                Self::field_error(&validation_result, default_style, tui, "name");

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

                                Self::field_error(&validation_result, default_style, tui, "directory");

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

                                Self::field_error(&validation_result, default_style, tui, "kind");
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


        /*
        let frame = Frame::group(ui.style());

        Flex::vertical()
            .align_content(FlexAlignContent::Stretch)
            .w_full()
            .show(ui, |form_flex| {

                let report = GardeReport::new(self.fields.validate());

                //
                // name field
                //
                form_flex.add_flex(
                    item().align_self(FlexAlign::Stretch).frame(frame).grow(1.0),
                    Flex::horizontal()
                        .justify(FlexJustify::SpaceBetween)
                        .w_full(),
                    |field_flex|{
                        // label
                        field_flex.add(
                            item().frame(frame).grow(0.2),
                            Label::new(tr!("form-new-name"))
                        );
                        // control
                        field_flex.add(
                            item().frame(frame).grow(0.8),
                             TextEdit::singleline(&mut self.fields.name)
                        );
                    }
                );
                if let Some(error) = report.get_field_error(field_path!("name")) {
                    form_flex.add_flex(
                        item().frame(frame).grow(1.0),
                        Flex::horizontal(),
                        |error_flex|{
                            error_flex.add(item().grow(1.0), Label::new(error));
                        }
                    );
                }

                //
                // kind field
                //
                form_flex.add_flex(
                    item().frame(frame).grow(1.0),
                    Flex::horizontal()
                        .w_full(),
                    |field_flex|{
                        // label
                        field_flex.add(
                            item().frame(frame).grow(0.2),
                            Label::new(tr!("form-new-kind"))
                        );
                        // control
                        field_flex.add_ui(
                            item().frame(frame).grow(0.8),
                            |ui: &mut Ui| {
                                let kind_id = ui.id();
                                egui::ComboBox::from_id_salt(kind_id)
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
                                    })
                                    .response
                            }
                        );
                    }
                );
                if let Some(error) = report.get_field_error(field_path!("kind")) {
                    form_flex.add_flex(
                        item().frame(frame).grow(1.0),
                        Flex::horizontal(),
                        |error_flex| {
                            error_flex.add(item().grow(1.0), Label::new(error));
                        }
                    );
                }



                // form_flex.add_ui(FlexItem::new(), |ui| {
                //     let kind_id = ui.id();
                //     FormField::new(&mut form, field_path!("kind"))
                //         .label(tr!("form-new-kind"))
                //         .ui(ui, |ui: &mut egui::Ui| {
                //             egui::ComboBox::from_id_salt(kind_id)
                //                 .selected_text(match self.fields.kind {
                //                     None => tr!("form-common-combo-default"),
                //                     Some(NewDocumentKind::Text) => tr!("form-new-kind-text"),
                //                     Some(NewDocumentKind::Image) => tr!("form-new-kind-image"),
                //                 })
                //                 .show_ui(ui, |ui| {
                //                     if ui
                //                         .add(egui::SelectableLabel::new(
                //                             self.fields.kind == Some(NewDocumentKind::Image),
                //                             tr!("form-new-kind-image"),
                //                         ))
                //                         .clicked()
                //                     {
                //                         self.fields.kind = Some(NewDocumentKind::Image)
                //                     }
                //                     if ui
                //                         .add(egui::SelectableLabel::new(
                //                             self.fields.kind == Some(NewDocumentKind::Text),
                //                             tr!("form-new-kind-text"),
                //                         ))
                //                         .clicked()
                //                     {
                //                         self.fields.kind = Some(NewDocumentKind::Text)
                //                     }
                //                 })
                //                 .response
                //     });
                // });
                //
                // form_flex.add_ui(FlexItem::new(), |ui| {
                //     FormField::new(&mut form, field_path!("directory"))
                //         .label(tr!("form-new-directory"))
                //         .ui(ui, |ui: &mut egui::Ui| {
                //             let mut selected_directory = self.fields.directory.clone().map_or("choose!".to_string(), |directory|{
                //                 directory.display().to_string()
                //             });
                //
                //             Flex::horizontal()
                //                 //.align_content(FlexAlignContent::Stretch)
                //                 .w_full()
                //                 .show(ui, |flex| {
                //                     flex.add_ui(FlexItem::new().grow(9.0), |ui|{
                //                         egui::TextEdit::singleline(&mut selected_directory).interactive(false).ui(ui)
                //                     });
                //                     flex.add_ui(FlexItem::new().grow(1.0), |ui|{
                //                         if egui::Button::new("...").ui(ui).clicked() {
                //                             self.file_picker.pick_folder()
                //                         }
                //                     })
                //                 }).response
                //         });
                // });
                //
                // form_flex.add_ui(FlexItem::new(), |ui| {
                //     let button = ui.button(tr!("form-common-button-ok"));
                //
                //     if let Some(Ok(())) = form.handle_submit(&button, ui) {
                //         self.on_submit();
                //     }
                // });
            });


        let mut text = "text".to_owned();

        Flex::horizontal()
            .align_content(FlexAlignContent::Stretch)
            .w_full()
            .show(ui, |flex| {
                flex.add(
                    item().grow(1.0),
                    TextEdit::singleline(&mut text)
                );
                flex.add(item(), Button::new("..."));
            });

         */
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }

    fn field_error(validation_result: &Result<(), Report>, default_style: fn() -> Style, tui: &mut Tui, field_path: &str) {
        if let Err(errors) = validation_result {
            if let Some((_path, error)) = errors.iter().find(|(path, error)| path.eq(&Path::new(field_path))) {
                tui
                    .style(Style {
                        grid_column: span(2),
                        ..default_style()
                    })
                    .add(|tui| {
                        tui.label(RichText::new(error.message()).color(colors::ERROR))
                    });
            }
        }
    }
}

fn no_transform(value: TuiContainerResponse<Response>, _ui: &Ui) -> TuiContainerResponse<Response> {
    value
}
