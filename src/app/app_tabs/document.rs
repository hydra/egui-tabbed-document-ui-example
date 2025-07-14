use crate::app::tabs::{Tab, TabKey};
use crate::context::TabContext;
use crate::documents::{DocumentContext, DocumentKey, DocumentKind};
use egui::{Ui, WidgetText};
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use egui_dock::tab_viewer::OnCloseResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentTab {
    title: String,
    pub path: PathBuf,
    pub document_key: DocumentKey,
}

impl Tab for DocumentTab {
    type Context = TabContext;

    fn label(&self) -> WidgetText {
        egui::widget_text::WidgetText::from(self.title.clone())
    }

    fn ui(&mut self, ui: &mut Ui, _tab_key: &TabKey, context: &mut Self::Context) {
        // get the document, this will fail if the document has not been restored on application startup.
        let mut documents_guard = context.documents.lock().unwrap();
        let document_kind = documents_guard.get_mut(self.document_key).unwrap();

        // delegate to the right document implementation, passing a `DocumentContext`.
        // Note: we can't pass the context, as it's already mutably borrowed.

        let mut document_context = DocumentContext {
            config: context.config.clone(),
            sender: context.sender.clone(),
        };

        // Note: we specifically do NOT pass a `TabKey` to the document as the document should NOT know that it lives in a tab.

        match document_kind {
            DocumentKind::TextDocument(document) => document.ui(ui, &mut document_context),
            DocumentKind::ImageDocument(document) => document.ui(ui, &mut document_context),
        }
    }

    fn on_close(&mut self, _tab_key: &TabKey, app: &mut TabContext) -> OnCloseResponse {
        debug!("removing document. key: {:?}", self.document_key);
        app.documents.lock().unwrap().remove(self.document_key);

        OnCloseResponse::Close
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
