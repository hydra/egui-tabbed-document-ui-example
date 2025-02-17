use std::collections::btree_map::{Iter, IterMut};
use crate::app::app_tabs::TabKind;
use egui::{Id, Ui, WidgetText};
use egui_dock::TabViewer;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::app::AppMessage;
use crate::context::Context;

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

    pub fn get_mut(&mut self, key: &TabKey) -> Option<&mut TabKind> {
        self.tabs.get_mut(key)
    }

    pub fn new() -> Tabs {
        Self {
            next_id: 0,
            tabs: BTreeMap::default(),
        }
    }

    pub fn iter(&self) -> Iter<TabKey, TabKind> {
        self.tabs.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<TabKey, TabKind> {
        self.tabs.iter_mut()
    }

    pub fn retain_all(&mut self, tab_keys: &[TabKey]) {
        self.tabs.retain(|tab_key, _| {
            let retain = tab_keys.contains(tab_key);
            
            if !retain {
                info!("Removing orphaned tab. key: {:?}", tab_key);
            }
            retain
        });
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

    fn on_close(&mut self, tab_key: &mut TabKey, app: &mut App) -> bool { true }
}

pub struct AppTabViewer<'a> {
    pub tabs: &'a mut Tabs,
    pub context: &'a mut Context<'a>,
}

impl<'a> TabViewer for AppTabViewer<'a> {
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
        // FIXME this isn't called when the 'close all' button in the tab bar is used.
        //       reported to maintainer - https://discord.com/channels/900275882684477440/1075333382290026567/1339624259697246348
        debug!("closing tab, id: {:?}", tab);

        let tab_instance = self.tabs.tabs.get_mut(tab).unwrap();
        let allow_close = tab_instance.on_close(tab, self.context);
        if allow_close {
            let _removed = self.tabs.tabs.remove(tab);
        }

        allow_close
    }
}
