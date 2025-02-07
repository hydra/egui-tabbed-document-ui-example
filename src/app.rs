use crate::app::app_tabs::home::HomeTab;
use crate::app::app_tabs::TabKind;
use crate::app::tabs::{TabKey, Tabs};
use egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_i18n::tr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
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
                    ui.button(tr!("toolbar-button-new"));
                    ui.button(tr!("toolbar-button-open"));
                    ui.button(tr!("toolbar-button-close-all"));

                    if home_button.clicked() {
                        let home_tab = self.tree.iter_all_tabs().find_map(|(surface_and_node, tab_key)|{
                            let tab = self.tabs.get(tab_key).unwrap();

                            match tab {
                                TabKind::Home(_) => Some((tab_key, tab, surface_and_node)),
                                _ => None,
                            }
                        });

                        if let Some((home_tab, home_tab_key, surface_and_node)) = home_tab {
                            // focus the existing home tab
                            self.tree.set_focused_node_and_surface(surface_and_node);
                        } else {
                            // create a new home tab
                            let home_tab_id = self.tabs.add(TabKind::Home(HomeTab::default()));
                            self.tree.push_to_focused_leaf(home_tab_id);
                        }
                    }
                });
            });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tabs);
    }
}
