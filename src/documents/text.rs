use std::path::PathBuf;
use egui::Ui;
use crate::context::Context;

pub struct TextDocument {
    pub path: PathBuf,

    content: Option<String>,
}

impl TextDocument {
    pub fn create_new(path: PathBuf) -> Self {
        Self {
            path,
            content: None
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path,
            content: None
        }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut Context<'a>) {
        ui.label("Text document");

        // todo, use something from the context, e.g. the `Config`.
    }
}