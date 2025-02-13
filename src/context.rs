use std::sync::mpsc::Sender;
use crate::app::{AppMessage, Config};

pub struct Context<'a> {
    pub config: &'a mut Config,
    pub sender: &'a Sender<AppMessage>
}