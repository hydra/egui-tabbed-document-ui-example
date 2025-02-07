use crate::app::app_tabs::home::HomeTab;
use crate::app::app_tabs::TabKind;
use crate::app::tabs::{TabKey, Tabs};
use egui_dock::{DockArea, DockState, Style};
use egui_i18n::tr;
use crate::app::app_tabs::new::NewTab;
use crate::fonts;

mod app_tabs;
mod tabs;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tabs: Tabs,
    tree: DockState<TabKey>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut tabs = Tabs::default();
        let _home_tab_id = tabs.add(TabKind::Home(HomeTab::default()));

        let initial_tab_ids = tabs.ids();

        let tree = DockState::new(initial_tab_ids);

        Self { tabs, tree }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        fonts::initialize(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn show_home_tab(&mut self) {
        let home_tab = self.tree.iter_all_tabs().find_map(|(_surface_and_node, tab_key)| {
            let tab_kind = self.tabs.get(tab_key).unwrap();

            match tab_kind {
                TabKind::Home(_) => {
                    Some(tab_key)
                },
                _ => None,
            }
        });

        if let Some(home_tab_key) = &home_tab {
            // although we have the tab, we don't know the tab_index, which is required for the call to `set_active_tab`,
            // so we have to call `find_tab`
            let find_result = self.tree.find_tab(home_tab_key).unwrap();
            self.tree.set_active_tab(find_result);

        } else {
            // create a new home tab
            let tab_id = self.tabs.add(TabKind::Home(HomeTab::default()));
            self.tree.push_to_focused_leaf(tab_id);
        }
    }

    fn add_new_tab(&mut self) {
        // create a new 'new' tab
        let tab_id = self.tabs.add(TabKind::New(NewTab::default()));
        self.tree.push_to_focused_leaf(tab_id);
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

            egui::Frame::new().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let home_button = ui.button(tr!("toolbar-button-home"));
                    let new_button = ui.button(tr!("toolbar-button-new"));
                    let _open_button = ui.button(tr!("toolbar-button-open"));
                    let _close_all = ui.button(tr!("toolbar-button-close-all"));

                    if home_button.clicked() {
                        self.show_home_tab();
                    }

                    if new_button.clicked() {
                        self.add_new_tab();
                    }
                });
            });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tabs);
    }
}
