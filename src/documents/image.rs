use std::path::PathBuf;
use egui::Ui;
use crate::context::Context;

pub struct ImageDocument {
    pub path: PathBuf,

    // TODO add content
}

impl ImageDocument {
    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut Context<'a>) {
        ui.label("Image document");
    }
}