use std::path::PathBuf;
use crate::app::tabs::{Tab, TabKey};
use egui::{Button, TextEdit, Ui, WidgetText};
use egui_flex::{item, Flex, FlexAlignContent};

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

        let mut text = "text".to_owned();

        Flex::horizontal()
            .w_full()
            .show(ui, |mut flex| {
            flex.add_ui(
                item()
                    .grow(1.0),
                |ui: &mut Ui | {
                ui.add(
                    TextEdit::singleline(&mut text)
                        .desired_width(10.0),
                );
            });
            flex.add_ui(
                item(),
                |ui| {
                ui.add(egui::Button::new("..."));
            });
        });

        Flex::horizontal()
            .align_content(FlexAlignContent::Stretch)
            .w_full()
            .show(ui, |flex| {
                flex.add(
                    item().grow(1.0),
                    TextEdit::singleline(&mut text).desired_width(10.0),
                );
                flex.add(item(), Button::new("Click me!"));
            });
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }
}
