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

    fn ui(&mut self, ui: &mut Ui, _tab_key: &mut TabKey, context: &mut Context<'a>) {
        ui.label(format!("path: {:?}, key: {:?}", self.path, self.document_key));
        
        // get the document, this will fail if the document has not been restored on application startup.
        let mut documents_guard = context.documents.lock().unwrap();
        let document_kind = documents_guard.get_mut(self.document_key).unwrap();

        // delegate to the right document implementation, passing a `DocumentContext`.
        // Note: we can't pass the context, as it's already mutably borrowed.

        let mut document_context = DocumentContext {
            config: context.config,
            sender: context.sender.clone(),
        };

        // Note: we specifically do NOT pass a `TabKey` to the document as the document should NOT know that it lives in a tab.
        
        match document_kind {
            DocumentKind::TextDocument(document) => document.ui(ui, &mut document_context),
            DocumentKind::ImageDocument(document) => document.ui(ui, &mut document_context),
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
