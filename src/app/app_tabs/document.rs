use std::path::PathBuf;
use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::documents::DocumentKey;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentTab {
    title: String,
    pub path: PathBuf,
    pub document_key: DocumentKey,
}

impl<'a> Tab<Context<'a>> for DocumentTab {
    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(self.title.clone())
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, _context: &mut Context<'a>) {
        ui.label(format!("path: {:?}, key: {:?}", self.path, self.document_key));

        // get the document, this will fail if the document has not been restored on application startup.

        let documents_guard = _context.documents.lock().unwrap();
        let document = documents_guard.get(self.document_key);

        match document {
            Some(document) => {
                ui.label("loaded");
            },
            None => {
                ui.label("unknown document key");
            }
        }
    }
}

impl DocumentTab {
    pub fn new(title: String, path: PathBuf, document_key: DocumentKey) -> Self {
        Self {
            title,
            path,
            document_key,
        }
    }
}
