use crate::app::app_tabs::document::DocumentTab;
use crate::app::app_tabs::home::HomeTab;
use crate::app::app_tabs::new::{KindChoice, NewTab};
use crate::app::app_tabs::TabKind;
use crate::app::tabs::{AppTabViewer, TabKey, Tabs};
use crate::context::Context;
use crate::documents::image::ImageDocument;
use crate::documents::text::TextDocument;
use crate::documents::{DocumentKey, DocumentKind};
use crate::file_picker::Picker;
use crate::fonts;
use egui_dock::{DockArea, DockState, Style};
use egui_i18n::tr;
use egui_inbox::{UiInbox, UiInboxSender};
use log::{debug, info};
use slotmap::SlotMap;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use egui_extras::install_image_loaders;

const SUPPORTED_TEXT_EXTENSIONS: [&'static str; 1] = ["txt"];
const SUPPORTED_IMAGE_EXTENSIONS: [&'static str; 4] = ["bmp", "png", "jpeg", "jpg"];


pub type AppMessageSender = UiInboxSender<(MessageSource, AppMessage)>;

mod app_tabs;
mod tabs;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tabs: Tabs,
    tree: DockState<TabKey>,

    config: Config,

    // state contains fields that cannot be initialized using 'Default'
    #[serde(skip)]
    state: MaybeUninit<AppState>,
}

struct AppState {
    // TODO find a better way of doing this that doesn't require this boolean
    startup_done: bool,
    file_picker: Picker,

    sender: UiInboxSender<(MessageSource, AppMessage)>,
    receiver: UiInbox<(MessageSource, AppMessage)>,
    documents: Arc<Mutex<SlotMap<DocumentKey, DocumentKind>>>,
}

#[derive(Debug)]
pub enum AppMessage {
    Refresh,
    CreateDocument(DocumentArgs),
}

#[derive(Debug)]
pub enum MessageSource {
    Document(DocumentKey),
    Tab(TabKey),
}

#[derive(Debug)]
pub struct DocumentArgs {
    name: String,
    directory: PathBuf,
    kind: KindChoice,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Config {
    show_home_tab_on_startup: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_home_tab_on_startup: true,
        }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        let config = Config::default();

        let mut tabs = Tabs::new();

        let _home_tab_id = tabs.add(TabKind::Home(HomeTab::default()));

        let initial_tab_ids = tabs.ids();

        let tree = DockState::new(initial_tab_ids);

        Self {
            tabs,
            tree,
            config,
            state: MaybeUninit::uninit(),
        }
    }
}

impl AppState {
    pub fn init() -> Self {
        let (sender, receiver) = UiInbox::channel();

        Self {
            startup_done: false,
            file_picker: Picker::default(),

            sender,
            receiver,
            documents: Default::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        fonts::initialize(&cc.egui_ctx);

        install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut instance = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        instance.state.write(AppState::init());
        // Safety: `Self::state()` is now safe to call.

        instance
    }

    fn show_home_tab(&mut self) {
        let home_tab = self.find_home_tab();

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

    fn find_home_tab(&self) -> Option<&TabKey> {
        let home_tab = self
            .tree
            .iter_all_tabs()
            .find_map(|(_surface_and_node, tab_key)| {
                let tab_kind = self.tabs.get(tab_key).unwrap();

                match tab_kind {
                    TabKind::Home(_) => Some(tab_key),
                    _ => None,
                }
            });
        home_tab
    }

    fn add_new_tab(&mut self) {
        // create a new 'new' tab
        let tab_id = self.tabs.add(TabKind::New(NewTab::default()));
        self.tree.push_to_focused_leaf(tab_id);
    }

    fn pick_file(&mut self) {
        if !self.state().file_picker.is_picking() {
            self.state().file_picker.pick_file();
        }
    }

    fn open_file(&mut self, ctx: &egui::Context, path: PathBuf) {
        info!("open file. path: {:?}", path);

        let title = path.file_name().unwrap().to_string_lossy().to_string();

        let sender = self.state().sender.clone();

        let document_key = self.state().documents.lock().unwrap().insert_with_key({
            let sender = sender.clone();

            |new_key| {
                Self::document_from_path(&path, ctx, sender, new_key)
            }
        });
        let tab_kind = TabKind::Document(DocumentTab::new(title, path, document_key));

        self.add_tab(tab_kind);
    }

    fn create_document_tab(&mut self, ctx: &mut egui::Context, args: DocumentArgs) {
        let tab_kind = self.create_document_tab_inner(ctx, args);

        self.add_tab(tab_kind);
    }

    fn add_tab(&mut self, tab_kind: TabKind) {
        let tab_id = self.tabs.add(tab_kind);
        self.tree.push_to_focused_leaf(tab_id);
    }

    fn create_document_tab_inner(&mut self, ctx: &egui::Context, args: DocumentArgs) -> TabKind {
        let DocumentArgs {
            mut name,
            directory: mut path,
            kind,
        } = args;

        match kind {
            KindChoice::Text => {
                name.push_str(".txt");
                path.push(&name);

                let title = path.file_name().unwrap().to_string_lossy().to_string();

                let text_document = TextDocument::create_new(path.clone());
                let document_kind = DocumentKind::TextDocument(text_document);

                let document_key = self.state().documents.lock().unwrap().insert(document_kind);
                TabKind::Document(DocumentTab::new(title, path, document_key))
            }
            KindChoice::Image => {
                name.push_str(".bmp");
                path.push(&name);

                let title = path.file_name().unwrap().to_string_lossy().to_string();

                let image_document = ImageDocument::create_new(path.clone(), ctx);
                let document_kind = DocumentKind::ImageDocument(image_document);

                let document_key = self.state().documents.lock().unwrap().insert(document_kind);
                TabKind::Document(DocumentTab::new(title, path, document_key))
                
            },
        }
    }

    /// provide mutable access to the state.
    ///
    /// Safety: it's always safe, because `new` calls `state.write()`
    ///
    /// Note: it's either `self.state()` everywhere or `self.state.unwrap()` if `AppSate` was wrapped in an `Option`
    /// instead if `MaybeUninit`, this is less verbose.
    fn state(&mut self) -> &mut AppState {
        unsafe { self.state.assume_init_mut() }
    }

    /// Safety: call only once on startup, before the tabs are shown.
    fn show_home_tab_on_startup(&mut self) {
        if self.config.show_home_tab_on_startup {
            self.show_home_tab();
        } else {
            if let Some(home_tab_key) = self.find_home_tab() {
                let find_result = self.tree.find_tab(home_tab_key).unwrap();
                self.tree.remove_tab(find_result);
            }
        }
    }

    /// Due to bugs in egui_dock where it doesn't call `on_close` when closing tabs, it's possible that the tabs
    /// and the dock tree are out of sync.  `on_close` should be removing elements from `self.tabs` corresponding to the
    /// tab being closed, but because it is not called there can be orphaned elements, we need to find and remove them.
    pub fn cleanup_tabs(&mut self) {
        let known_tab_keys = self
            .tree
            .iter_all_tabs()
            .map(|(_surface_and_node, tab_key)| tab_key.clone())
            .collect::<Vec<_>>();
        
        // FIXME this doesn't close the documents corresponding to orphaned tabs.
        //       we need to call `on_close` for each closed tab manually.
        
        self.tabs.retain_all(&known_tab_keys);
    }

    /// when the app starts up, the documents will be empty, and the document tabs will have keys that don't exist
    /// in the documents list (because it's empty now).
    /// we have to find these tabs, create documents, store them in the map and replace the tab's document key
    /// with the new key generated when adding the key to the map
    ///
    /// Safety: call only once on startup, before the tabs are shown.
    fn restore_documents_on_startup(&mut self, ctx: &egui::Context) {
        // we have to do this as a two-step process to above borrow-checker issues

        // step 1 - find the document tabs, return the tab keys and paths.
        let tab_keys_and_paths = self
            .tabs
            .iter_mut()
            .filter_map(|(tab_key, tab_kind)| match tab_kind {
                TabKind::Document(document_tab) => {
                    Some((tab_key.clone(), document_tab.path.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        // step 2 - store the documents and update the document key for the tab.
        for (tab_key, path) in tab_keys_and_paths {
            let sender = self.state().sender.clone();

            let new_key = self.state().documents.lock().unwrap().insert_with_key({
                let sender = sender.clone();
                |new_key| {
                    Self::document_from_path(&path, ctx, sender, new_key)
                }
            });
            if let TabKind::Document(ref mut document_tab) = self.tabs.get_mut(&tab_key).unwrap() {
                document_tab.document_key = new_key;
            } else {
                unreachable!()
            }
        }
    }

    fn document_from_path(path: &PathBuf, ctx: &egui::Context, sender: UiInboxSender<(MessageSource, AppMessage)>, new_key: DocumentKey) -> DocumentKind {
        let extension = path.extension().unwrap().to_str().unwrap();

        if SUPPORTED_TEXT_EXTENSIONS.contains(&extension) {
            let text_document = TextDocument::from_path(path.clone(), ctx, new_key, sender);
            DocumentKind::TextDocument(text_document)
        } else if SUPPORTED_IMAGE_EXTENSIONS.contains(&extension) {
            let image_document = ImageDocument::from_path(path.clone(), ctx, new_key, sender);
            DocumentKind::ImageDocument(image_document)
        } else {
            todo!()
        }
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


        let mut messages: Vec<(MessageSource, AppMessage)> =
            self.state().receiver.read(ctx).collect();

        for (source, message) in messages.drain(..) {
            match (source, message) {
                (MessageSource::Tab(tab_key), AppMessage::CreateDocument(args)) => {
                    // replace tabs here...

                    let document_tab_kind = self.create_document_tab_inner(ctx, args);

                    if let Some(tab_kind) = self.tabs.get_mut(&tab_key) {
                        *tab_kind = document_tab_kind;
                    } else {
                        // message is sent from a tab that does not exist.
                        unreachable!()
                    }
                }
                (source, AppMessage::Refresh) => {
                    // nothing to do, we're already refreshing at this point.
                    debug!("refresh message received. source: {:?}", source);
                }
                (_, _) => {
                    // unprocessed message
                    unreachable!()
                }
            }
        }
        
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
                    let open_button = ui.button(tr!("toolbar-button-open"));
                    let close_all_button = ui.button(tr!("toolbar-button-close-all"));

                    if home_button.clicked() {
                        self.show_home_tab();
                    }

                    if new_button.clicked() {
                        self.add_new_tab();
                    }

                    if open_button.clicked() {
                        self.pick_file()
                    }

                    if close_all_button.clicked() {
                        // FIXME there's a bug in `egui_dock` where the `on_close` handler is not called
                        //       when programmatically closing all the tabs - reported via discord: https://discord.com/channels/900275882684477440/1075333382290026567/1340993744941617233
                        self.tree.retain_tabs(|_tab_key| false);
                    }
                });
            });
        });

        if !self.state().startup_done {
            self.state().startup_done = true;

            self.show_home_tab_on_startup();
            self.restore_documents_on_startup(ctx);
        }

        // FIXME remove this when `on_close` bugs in egui_dock are fixed.
        self.cleanup_tabs();

        // TODO discover whether cloning a sender is expensive or not
        let sender = self.state().sender.clone();
        let documents = self.state().documents.clone();

        let mut context = Context {
            config: &mut self.config,
            sender,
            documents,
        };

        let mut my_tab_viewer = AppTabViewer {
            tabs: &mut self.tabs,
            context: &mut context,
        };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut my_tab_viewer);

        if let Ok(picked_file) = self.state().file_picker.picked() {
            // FIXME this `update` method does not get called immediately after picking a file, instead update gets
            //       called when the user moves the mouse or interacts with the window again.
            self.open_file(ctx, picked_file);
        }
    }
}
