use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use egui::Ui;
use egui_i18n::tr;
use log::info;
use crate::documents::DocumentContext;

pub struct TextDocument {
    pub path: PathBuf,

    loader: TextDocumentContent,
}

enum LoaderState {
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
        let handle = thread::Builder::new()
            .name(format!("loader: {:?}", path))
            .spawn(move || {
                info!("Loading {}", path.display());
                
                // FIXME add a 2 second delay to simulate slow loading so that the lack of some UI notification on
                //       thread complete can be observed in the UI.  Some UI interaction is required before the 
                //       content becomes visible, e.g. moving the mouse.
                thread::sleep(Duration::from_secs(2));

                let content = std::fs::read_to_string(path).unwrap();

                content
        }).unwrap();

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

    pub fn from_path(path: PathBuf) -> Self {
        let loader = TextDocumentContent::load(path.clone());

        Self {
            path,
            loader,
        }
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