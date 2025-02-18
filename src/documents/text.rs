use crate::app::{AppMessage, AppMessageSender, MessageSource};
use crate::documents::{DocumentContext, DocumentKey};
use egui::Ui;
use egui_i18n::tr;
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

        if let Some(content) = self.loader.content_mut() {
            ui.text_edit_multiline(content);
        } else {
            ui.label(tr!("file-loading"));
        }

        // todo, use something from the context, e.g. the `Config`.
    }
}
