use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_i18n::tr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq, Serialize, Deserialize)]
struct TabId(usize);

impl TabId {
    pub fn new() -> Self {
        let id = Self::next_id();
        Self (id)
    }

    fn next_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, SeqCst);
        id
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Tabs {
    tabs: BTreeMap<TabId, TabKind>,
}

impl Tabs {
    pub fn add(&mut self, tab_kind: TabKind) -> TabId {
        let id = TabId::new();
        self.tabs.insert(id, tab_kind);

        id
    }
}

impl Tabs {
    pub fn ids(&self) -> Vec<TabId> {
        self.tabs.keys().cloned().collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum TabKind {
    Home,
    Document(String),
    New,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tabs: Tabs,
    tree: DockState<TabId>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut tabs = Tabs::default();
        let _home_tab_id = tabs.add(TabKind::Home);

        let initial_tab_ids = tabs.ids();

        let tree = DockState::new(initial_tab_ids);

        Self {
            tabs,
            tree,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button(tr!("menu-top-level-file"), |ui| {
                        if ui.button(tr!("menu-item-quit")).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });

            egui::Frame::new()
                .show(ui, |ui| {
                    ui.horizontal(|ui|{
                        ui.button(tr!("toolbar-button-home"));
                        ui.button(tr!("toolbar-button-new"));
                        ui.button(tr!("toolbar-button-open"));
                        ui.button(tr!("toolbar-button-close-all"));
                    });
                });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tabs);
    }
}

impl TabViewer for Tabs {
    type Tab = TabId;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        let title = format!("{:?}", tab);

        egui::WidgetText::from(&*title)
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        let _tab_instance = self.tabs.get_mut(tab);

        // TODO delegate to tab kind
        ui.label(format!("tab: {:?}", tab));
    }
}
