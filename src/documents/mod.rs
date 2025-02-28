use std::sync::{Arc, Mutex};
use crate::app::{AppMessage, Config, MessageSource};
use crate::documents::image::ImageDocument;
use crate::documents::text::TextDocument;
use egui_inbox::UiInboxSender;
use slotmap::new_key_type;

pub mod loader;

pub mod image;
pub mod text;

new_key_type! {
    /// A key for a document
    pub struct DocumentKey;
}

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
}

pub struct DocumentContext {
    pub config: Arc<Mutex<Config>>,
    pub sender: UiInboxSender<(MessageSource, AppMessage)>,
}

