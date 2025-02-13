use std::path::PathBuf;
use egui::Ui;
use crate::context::Context;
use crate::documents::DocumentContext;

pub struct TextDocument {
    pub path: PathBuf,

    content: Option<String>,
}

impl TextDocument {
    pub fn create_new(path: PathBuf) -> Self {
        Self {
            path,
            content: Some("example content".to_string()),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        // TODO load the content from the path in a background thread
        Self {
            path,
            content: None
        }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext<'a>) {
        if let Some(content) = &mut self.content {
            ui.text_edit_multiline(content);
        } else {
            ui.label("loading...");
        }

        // todo, use something from the context, e.g. the `Config`.
    }
}