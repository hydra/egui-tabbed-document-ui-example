use std::path::PathBuf;
use crate::app::tabs::{Tab, TabKey};
use egui::{Frame, TextEdit, Ui, Widget, WidgetText};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexDirection, FlexItem};
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
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

        Flex::new()
            .w_full()
            //.align_items(egui_flex::FlexAlign::Stretch)
            //.align_items_content(egui::Align2::CENTER_CENTER)
            // .direction(FlexDirection::Vertical)
            // .align_content(FlexAlignContent::Stretch)
            // .h_full()
            .show(ui, |outer_flex| {
                let mut form = Form::new().add_report(GardeReport::new(self.fields.validate()));
                //
                // outer_flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                //     FormField::new(&mut form, field_path!("name"))
                //         .label(tr!("form-new-name"))
                //         .ui(ui, |ui: &mut egui::Ui |
                //             TextEdit::singleline(&mut self.fields.name)
                //                 .desired_width(ui.available_width())
                //                 .ui(ui)
                //         );
                // });
                //
                // outer_flex.add_ui(FlexItem::new(), |ui| {
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
                //         });
                // });

                let mut selected_directory = self.fields.directory.clone().map_or("choose!".to_string(), |directory|{
                    directory.display().to_string()
                });

                outer_flex.add_flex(
                    FlexItem::new()
                        //.grow(1.0)
                        .frame(egui::Frame::group(outer_flex.ui().style())),
                    Flex::horizontal()
                        //.align_content(egui_flex::FlexAlignContent::Stretch)
                        .grow_items(1.0),
                    |field_flex| {

                    field_flex.add_flex(
                        FlexItem::new()
                            //.grow(1.0)
                            .frame(egui::Frame::group(field_flex.ui().style())),
                        Flex::horizontal()
                            //.align_content(egui_flex::FlexAlignContent::Stretch)
                            //.grow_items(1.0),
                            ,
                        |field_inner_flex| {
                            field_inner_flex.add_ui(
                                FlexItem::default()
                                    //.grow(1.0)
                                    //.basis(200.0)
                                    .frame(Frame::group(field_inner_flex.ui().style())),
                                |ui| {
                                    TextEdit::singleline(&mut selected_directory)
                                        // FIXME FAIL FAIL FAIL FAIL FAIL
                                        //       ALWAYS TAKES UP THE ENTIRE WINDOW WIDTH
                                        .desired_width(ui.available_width())
                                        .ui(ui);
                                });
                        }
                    );
                    field_flex.add_flex(
                        FlexItem::new()
                            //.grow(1.0)
                            .frame(egui::Frame::group(field_flex.ui().style())),
                        Flex::horizontal()
                            //.align_content(egui_flex::FlexAlignContent::Stretch)
                            //.grow_items(1.0),
                            ,
                        |field_inner_flex| {
                            field_inner_flex.add_ui(
                                FlexItem::new()
                                    //.grow(1.0),
                                    .frame(Frame::group(field_inner_flex.ui().style())),
                                |ui| {
                                if egui::Button::new("...")
                                    .ui(ui).clicked() {
                                    self.file_picker.pick_folder()
                                }
                            });
                        }
                    );

                    // field_flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                    //     egui::TextEdit::singleline(&mut selected_directory)
                    //         .desired_width(ui.available_width())
                    //         .interactive(false)
                    //         .ui(ui)
                    // });


                });

                // FormField::new(&mut form, field_path!("directory"))
                //     .label(tr!("form-new-directory"))
                //     .ui(ui, |ui: &mut egui::Ui| {
                //     });
                //
                // outer_flex.add_ui(FlexItem::new(), |ui| {
                //     let button = ui.button(tr!("form-common-button-ok"));
                //
                //     if let Some(Ok(())) = form.handle_submit(&button, ui) {
                //         self.on_submit();
                //     }
                // });
            });
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }
}
