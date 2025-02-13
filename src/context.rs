use std::sync::mpsc::Sender;
use slotmap::SlotMap;
use crate::app::{AppMessage, Config};
use crate::documents::{DocumentKey, DocumentKind};

pub struct Context<'a> {
    pub config: &'a mut Config,
    pub sender: &'a Sender<AppMessage>,
    pub documents: &'a mut SlotMap<DocumentKey, DocumentKind>
}