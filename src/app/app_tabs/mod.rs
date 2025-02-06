use crate::app::app_tabs::document::DocumentTab;
use crate::app::app_tabs::home::HomeTab;
use crate::app::app_tabs::new::NewTab;
use crate::app::tabs::{Tab, TabKey};
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

pub mod document;
pub mod home;
pub mod new;

#[derive(Debug, Deserialize, Serialize)]
pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
    New(NewTab),
}

impl Tab for TabKind {
    fn label(&self) -> WidgetText {
        match self {
            TabKind::Home(tab) => tab.label(),
            TabKind::Document(tab) => tab.label(),
            TabKind::New(tab) => tab.label(),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab_key: &mut TabKey) {
        match self {
            TabKind::Home(tab) => tab.ui(ui, tab_key),
            TabKind::Document(tab) => tab.ui(ui, tab_key),
            TabKind::New(tab) => tab.ui(ui, tab_key),
        }
    }
}
