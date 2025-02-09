use crate::app::tabs::{Tab, TabKey};
use egui::{TextEdit, Ui, WidgetText};
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
use egui_i18n::tr;
use garde::Validate;
use serde::{Deserialize, Serialize};
use crate::TemplateApp;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct NewTab {
    fields: NewTabForm,
}

// FIXME form errors do not use i18n
#[derive(Clone, Debug, Default, Validate, Deserialize, Serialize)]
struct NewTabForm {
    #[garde(length(min = 1))]
    name: String,

    #[garde(required)]
    kind: Option<NewDocumentKind>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
enum NewDocumentKind {
    Text,
    Image,
}

impl Tab<TemplateApp> for NewTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from("New")
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, app: &mut TemplateApp) {
        let mut form = Form::new().add_report(GardeReport::new(self.fields.validate()));

        FormField::new(&mut form, field_path!("name"))
            .label(tr!("form-new-name"))
            .ui(ui, TextEdit::singleline(&mut self.fields.name));

        let kind_id = ui.id();
        FormField::new(&mut form, field_path!("kind"))
            .label(tr!("form-new-kind"))
            .ui(ui, |ui: &mut egui::Ui| {
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
            });

        if let Some(Ok(())) = form.handle_submit(&ui.button(tr!("form-common-button-ok")), ui) {
            self.on_submit();
        }
    }
}

impl NewTab {
    fn on_submit(&mut self) {
        println!("Submitted: {:?}", self.fields);
    }
}
