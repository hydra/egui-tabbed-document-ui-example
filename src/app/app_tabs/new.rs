use std::path::PathBuf;
use eframe::emath::Align2;
use crate::app::tabs::{Tab, TabKey};
use egui::{Button, Frame, Label, TextEdit, Ui, WidgetText};
use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexDirection, FlexItem, FlexJustify};
use egui_form::{EguiValidationReport, Form, FormField};
use egui_form::garde::{field_path, GardeReport};
use egui_i18n::tr;
use garde::Validate;
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::file_picker::Picker;

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
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }
}
