use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use slotmap::SlotMap;
use crate::app::{AppMessage, Config};
use crate::documents::{DocumentKey, DocumentKind};

pub struct Context<'a> {
    pub config: &'a mut Config,
    pub sender: Sender<AppMessage>,
    pub documents: Arc<Mutex<SlotMap<DocumentKey, DocumentKind>>>
}