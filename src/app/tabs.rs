use egui::{Id, Ui, WidgetText};
use egui_dock::TabViewer;
use egui_dock::tab_viewer::OnCloseResponse;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::{Iter, IterMut};
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Hash, Copy, Ord, Eq, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct TabKey(usize);

#[derive(Serialize, Deserialize, Default)]
pub struct Tabs<TabKind, Context> {
    next_id: usize,
    tabs: BTreeMap<TabKey, TabKind>,
    _phantom: PhantomData<Context>,
}

#[allow(dead_code)]
impl<Context, TabKind: Tab<Context = Context>> Tabs<TabKind, Context> {
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

    pub fn new() -> Self {
        Self {
            next_id: 0,
            tabs: BTreeMap::default(),
            _phantom: Default::default(),
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, TabKey, TabKind> {
        self.tabs.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, TabKey, TabKind> {
        self.tabs.iter_mut()
    }

    pub fn retain_all(&mut self, tab_keys: &[TabKey], tab_context: &mut Context) {
        self.tabs.retain(|tab_key, tab| {
            let retain = tab_keys.contains(tab_key);
            
            if !retain {
                let close_response = tab.on_close(tab_key, tab_context);

                if matches!(close_response, OnCloseResponse::Close) {
                    info!("Removing orphaned tab. key: {:?}", tab_key);
                }
            }
            retain
        });
    }
}

impl<TabKind, TabContext> Tabs<TabKind, TabContext> {
    pub fn ids(&self) -> Vec<TabKey> {
        self.tabs.keys().cloned().collect()
    }
}

pub trait Tab {
    
    type Context;
    
    fn label(&self) -> WidgetText;
    fn ui<'a>(&mut self, ui: &mut Ui, tab_key: &TabKey, app: &mut Self::Context);

    // handle a tab being closed
    // this is where any per-tab clean-up code should be performed
    //
    // return 'true' to allow the tab to be closed, 'false' to prevent closing.
    // FIXME due to bugs in egui_dock, this is not always called, see related FIXMEs in the codebase
    //       do NOT rely on this method for now, workarounds are required.
    fn on_close<'a>(&mut self, _tab_key: &TabKey, _app: &mut Self::Context) -> OnCloseResponse {
        OnCloseResponse::Close
    }
}

pub struct AppTabViewer<'a, TabContext, TabKind: Tab> {
    pub tabs: &'a mut Tabs<TabKind, TabContext>,
    pub context: &'a mut TabContext,
}

impl<'a, TabContext, TabKind: Tab<Context = TabContext>> TabViewer for AppTabViewer<'a, TabContext, TabKind> {
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

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        // FIXME this isn't called when the 'close all' button in the tab bar is used.
        //       reported to maintainer - https://discord.com/channels/900275882684477440/1075333382290026567/1339624259697246348
        debug!("closing tab, id: {:?}", tab);

        let tab_instance = self.tabs.tabs.get_mut(tab).unwrap();
        let close_response = tab_instance.on_close(tab, self.context);
        if matches!(close_response, OnCloseResponse::Close) {
            let _removed = self.tabs.tabs.remove(tab);
        }
        close_response
        // let allow_close = tab_instance.on_close(tab, self.context);
        // if allow_close {
        //     let _removed = self.tabs.tabs.remove(tab);
        // }
        //
        // allow_close
    }
}
