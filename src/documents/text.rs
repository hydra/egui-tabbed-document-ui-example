use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use egui::Ui;
use egui_i18n::tr;
use egui_inbox::UiInboxSender;
use log::info;
use crate::app::{AppMessage, AppMessageSender, MessageSource};
use crate::documents::{DocumentContext, DocumentKey};

pub struct TextDocument {
    pub path: PathBuf,

    loader: TextDocumentContent,
}

enum LoaderState {
    Load(PathBuf),
    Loading(Option<JoinHandle<String>>),
    Loaded(String)
}

struct TextDocumentContent {
    state: LoaderState
}

impl TextDocumentContent {
    fn content_mut(&mut self) -> Option<&mut String> {
        match &mut self.state {
            LoaderState::Loaded(content) => Some(content),
            _ => None
        }
    }

    fn new(content: String) -> Self {
        Self {
            state: LoaderState::Loaded(content)
        }
    }
    
    fn load(path: PathBuf) -> Self {
        Self {
            state: LoaderState::Load(path),
        }
    }
    
    pub fn update(&mut self, document_key: &DocumentKey, sender: &UiInboxSender<(MessageSource, AppMessage)>) {
        match &mut self.state {
            LoaderState::Load(path) => {

                let path = path.clone();
                let sender = sender.clone();
                let document_key = document_key.clone();
                
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
                        let message = (MessageSource::Document(document_key), AppMessage::Refresh);
                        sender.send(message).expect("sent");

                        content
                    }).unwrap();
                    
                self.state = LoaderState::Loading(Some(handle));
            }
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

    pub fn from_path(path: PathBuf) -> Self {
        let loader = TextDocumentContent::load(path.clone());
        
        Self {
            path,
            loader,
        }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext<'a>) {

        self.loader.update(&_context.document_key, &_context.sender);


        if let Some(content) = self.loader.content_mut() {
            ui.text_edit_multiline(content);
        } else {
            ui.label(tr!("file-loading"));
        }

        // todo, use something from the context, e.g. the `Config`.
    }
}