use crate::app::app_tabs::document::DocumentTab;
use crate::app::app_tabs::home::HomeTab;
use crate::app::app_tabs::new::NewTab;
use crate::app::tabs::{Tab, TabKey};
use crate::context::TabContext;
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

pub mod document;
pub mod home;
pub mod new;

#[derive(Deserialize, Serialize)]
pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
    New(NewTab),
}

impl Tab for TabKind {
    type Context = TabContext;

    fn label(&self) -> WidgetText {
        match self {
            TabKind::Home(tab) => tab.label(),
            TabKind::Document(tab) => tab.label(),
            TabKind::New(tab) => tab.label(),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab_key: &TabKey, context: &mut TabContext) {
        match self {
            TabKind::Home(tab) => tab.ui(ui, tab_key, context),
            TabKind::Document(tab) => tab.ui(ui, tab_key, context),
            TabKind::New(tab) => tab.ui(ui, tab_key, context),
        }
    }

    fn on_close(&mut self, tab_key: &TabKey, context: &mut TabContext) -> bool {
        match self {
            TabKind::Home(tab) => tab.on_close(tab_key, context),
            TabKind::Document(tab) => tab.on_close(tab_key, context),
            TabKind::New(tab) => tab.on_close(tab_key, context),
        }
    }
}
