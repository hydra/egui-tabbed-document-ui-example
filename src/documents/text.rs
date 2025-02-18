use crate::app::{AppMessage, AppMessageSender, MessageSource};
use crate::documents::{DocumentContext, DocumentKey};
use egui::{frame, Margin, TextEdit, Ui};
use egui_i18n::tr;
use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent};
use egui_taffy::taffy::{AlignItems, Display, FlexDirection, Size, Style};
use egui_taffy::{tui, TuiBuilderLogic};
use log::info;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct TextDocument {
    pub path: PathBuf,

    loader: TextDocumentContent,
}

enum LoaderState {
    Loading(Option<JoinHandle<String>>),
    Loaded(String),
}

struct TextDocumentContent {
    state: LoaderState,
}

impl TextDocumentContent {
    fn content(&self) -> Option<&String> {
        match &self.state {
            LoaderState::Loaded(content) => Some(content),
            _ => None,
        }
    }

    fn content_mut(&mut self) -> Option<&mut String> {
        match &mut self.state {
            LoaderState::Loaded(content) => Some(content),
            _ => None,
        }
    }

    fn new(content: String) -> Self {
        Self {
            state: LoaderState::Loaded(content),
        }
    }

    fn load(
        path: PathBuf,
        on_loaded_message: (MessageSource, AppMessage),
        sender: AppMessageSender,
    ) -> Self {
        let handle = thread::Builder::new()
            .name(format!("loader: {:?}", path))
            .spawn(move || {
                info!("Loading {}", path.display());

                // add a 2-second delay to simulate slow loading.
                // this is done to that thread notification can be observed in the UI; a solution is required
                // to have the UI update when loading is complete.
                thread::sleep(Duration::from_secs(1));

                let content = std::fs::read_to_string(path).unwrap();

                // send a message via the sender to cause the UI to be updated when loading is complete.
                sender.send(on_loaded_message).expect("sent");

                content
            })
            .unwrap();

        Self {
            state: LoaderState::Loading(Some(handle)),
        }
    }

    pub fn update(&mut self) {
        match &mut self.state {
            LoaderState::Loading(handle) => {
                if handle.as_ref().unwrap().is_finished() {
                    let handle = handle.take().unwrap();

                    let result = handle.join().unwrap();
                    self.state = LoaderState::Loaded(result);
                }
            }
            _ => {}
        }
    }
}

impl TextDocument {
    pub fn create_new(path: PathBuf) -> Self {
        Self {
            path,
            loader: TextDocumentContent::new("example content".to_string()),
        }
    }

    pub fn from_path(path: PathBuf, document_key: DocumentKey, sender: AppMessageSender) -> Self {
        let message = (MessageSource::Document(document_key), AppMessage::Refresh);
        let loader = TextDocumentContent::load(path.clone(), message, sender);

        Self { path, loader }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext<'a>) {
        self.loader.update();

        ui.ctx().style_mut(|style| {
            // if this is not done, text in labels/checkboxes/etc wraps
            style.wrap_mode = Some(egui::TextWrapMode::Extend);
            style.spacing.window_margin = Margin::same(0);
        });

        let default_style = || Style {
            padding: length(2.),
            gap: length(2.),
            ..Default::default()
        };

        let mut frame = frame::Frame::new();
        frame.outer_margin = Margin::same(0);
        frame.inner_margin = Margin::same(0);

        egui::SidePanel::left("sidebar")
            .resizable(true)
            .frame(frame)
            .show_inside(ui, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    tui(ui, ui.id().with("grid"))
                        .reserve_available_width()
                        .style(Style {
                            align_items: Some(AlignItems::Stretch),
                            flex_direction: FlexDirection::Column,
                            size: Size {
                                width: percent(1.),
                                height: auto(),
                            },
                            padding: length(0.),
                            gap: length(0.),
                            ..default_style()
                        })
                        .show(|tui| {
                            tui.style(Style {
                                flex_grow: 1.0,
                                display: Display::Grid,
                                grid_template_columns: vec![fit_content(percent(1.)), fr(1.)],
                                grid_template_rows: vec![fr(1.), fr(1.)],

                                // ensure items are centered vertically on rows
                                align_items: Some(AlignItems::Center),
                                padding: length(0.),
                                margin: length(0.),
                                ..default_style()
                            })
                            .add(|tui| {
                                tui.style(Style { ..default_style() })
                                    .add_with_border(|tui| {
                                        tui.label(tr!("document-sidebar-file-path"));
                                    });
                                tui.style(Style {
                                    flex_grow: 1.0,
                                    ..default_style()
                                })
                                .add_with_border(|tui| {
                                    tui.ui_add(egui::Label::new(self.path.display().to_string()))
                                });
                                tui.style(Style { ..default_style() })
                                    .add_with_border(|tui| {
                                        tui.label(tr!("document-sidebar-text-length"));
                                    });
                                tui.style(Style {
                                    flex_grow: 1.0,
                                    ..default_style()
                                })
                                .add_with_border(|tui| {
                                    let label_content = self
                                        .loader
                                        .content()
                                        .map_or(tr!("generic-unknown-value"), |content| {
                                            content.len().to_string()
                                        });
                                    tui.label(label_content);
                                });
                                // end of grid content
                            });
                            // end of container content
                        });
                    // end of scroll content
                });
                // end of sidebar content
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.content_ui(ui);
            });
        });

        // todo, use something from the context, e.g. the `Config`.
    }

    fn content_ui(&mut self, ui: &mut Ui) {
        if let Some(content) = self.loader.content_mut() {
            ui.add_sized(ui.available_size(), TextEdit::multiline(content));
        } else {
            ui.spinner();
            ui.label(tr!("file-loading"));
        }
    }
}
