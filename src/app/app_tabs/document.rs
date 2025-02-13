use std::path::PathBuf;
use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use log::debug;
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::documents::{DocumentContext, DocumentKey, DocumentKind};

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

        let mut documents_guard = _context.documents.lock().unwrap();
        let document = documents_guard.get_mut(self.document_key);

        // FAIL here we have a catch-22 situation. we need to delegate to the right document implementation
        //      but cannot pass the context because we need to borrow the documents from the context to find
        //      out what type of document it is before we can delegate to it.

        let mut document_context = DocumentContext {
            config: _context.config,
            sender: _context.sender.clone(),
        };

        match document {
            Some(document_kind) => {
                ui.label("loaded");
                match document_kind {
                    DocumentKind::TextDocument(document) => document.ui(ui, &mut document_context),
                    DocumentKind::ImageDocument(document) => document.ui(ui, &mut document_context),
                }
            }
            None => {
                ui.label("unknown document key");
            }
        }
    }

    fn on_close(&mut self, _tab_key: &mut TabKey, app: &mut Context<'a>) -> bool {
        debug!("removing document. key: {:?}", self.document_key);
        app.documents.lock().unwrap().remove(self.document_key);

        true
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
