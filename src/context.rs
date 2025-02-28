use crate::app::{AppMessage, Config, MessageSource};
use crate::documents::{DocumentKey, DocumentKind};
use egui_inbox::UiInboxSender;
use slotmap::SlotMap;
use std::sync::{Arc, Mutex};

pub struct TabContext {
    pub config:Arc<Mutex<Config>>,
    pub sender: UiInboxSender<(MessageSource, AppMessage)>,
    pub documents: Arc<Mutex<SlotMap<DocumentKey, DocumentKind>>>,
}
