use crate::documents::DocumentContext;
use egui::Ui;
use std::path::PathBuf;

pub struct ImageDocument {
    pub path: PathBuf,
    // TODO add content
}

impl ImageDocument {
    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext<'a>) {
        ui.label("Image document");
    }
}
