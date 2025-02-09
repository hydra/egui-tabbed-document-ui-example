use crate::app::app_tabs::TabKind;
use egui::{Id, Ui, WidgetText};
use egui_dock::TabViewer;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::context::Context;
use crate::TemplateApp;

#[derive(Debug, Clone, Hash, Copy, Ord, Eq, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct TabKey(usize);

#[derive(Serialize, Deserialize)]
pub struct Tabs {
    next_id: usize,
    tabs: BTreeMap<TabKey, TabKind>,
}

impl Tabs {
    fn next_key(&mut self) -> TabKey {
        loop {
            self.next_id = self.next_id.wrapping_add(1);
            let candidate_id = TabKey(self.next_id);
            if !self.tabs.contains_key(&candidate_id) {
                return candidate_id;
            }
        }
    }

    pub fn add(&mut self, tab_kind: TabKind) -> TabKey {
        let id = self.next_key();
        self.tabs.insert(id, tab_kind);

        id
    }

    pub fn get(&self, key: &TabKey) -> Option<&TabKind> {
        self.tabs.get(key)
    }

    pub fn new() -> Tabs {
        Self {
            next_id: 0,
            tabs: BTreeMap::default(),
        }
    }
}

impl Tabs {
    pub fn ids(&self) -> Vec<TabKey> {
        self.tabs.keys().cloned().collect()
    }
}

pub trait Tab<App> {
    fn label(&self) -> WidgetText;
    fn ui(&mut self, ui: &mut Ui, tab_key: &mut TabKey, app: &mut App);
}

pub struct MyTabViewer<'a> {
    pub tabs: &'a mut Tabs,
    pub context: &'a mut Context<'a>,
}

impl<'a> TabViewer for MyTabViewer<'a> {
    type Tab = TabKey;

    fn id(&mut self, tab: &mut Self::Tab) -> Id {
        Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        let tab_instance = self.tabs.tabs.get_mut(tab).unwrap();
        tab_instance.label()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        // see the api docs for `on_close`, if the active tab was just closed, we still arrive here.
        if let Some(tab_instance) = self.tabs.tabs.get_mut(tab) {
            tab_instance.ui(ui, tab, self.context);
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        debug!("closing tab, id: {:?}", tab);
        let _removed = self.tabs.tabs.remove(tab);

        true
    }
}
