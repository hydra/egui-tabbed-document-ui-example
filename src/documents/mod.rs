use egui_inbox::UiInboxSender;
use slotmap::new_key_type;
use crate::app::{AppMessage, Config, MessageSource};
use crate::documents::image::ImageDocument;
use crate::documents::text::TextDocument;

pub mod text;
pub mod image;

new_key_type! {
    /// A key for a document
    pub struct DocumentKey;
}

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
}


pub struct DocumentContext<'a> {
    pub config: &'a mut Config,
    pub sender: UiInboxSender<(MessageSource, AppMessage)>,
}