use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use log::info;
use crate::app::{AppMessage, AppMessageSender, MessageSource};

enum LoaderState<T: Send + 'static> {
    Loading(Option<JoinHandle<T>>),
    Loaded(T),
}


pub struct DocumentContent<T: Send + 'static> {
    state: LoaderState<T>,
}

impl<T: Send + 'static> DocumentContent<T> {
    pub fn content(&self) -> Option<&T> {
        match &self.state {
            LoaderState::Loaded(content) => Some(content),
            _ => None,
        }
    }

    pub fn content_mut(&mut self) -> Option<&mut T> {
        match &mut self.state {
            LoaderState::Loaded(content) => Some(content),
            _ => None,
        }
    }

    pub fn new(content: T) -> Self {
        Self {
            state: LoaderState::Loaded(content),
        }
    }

    pub fn load(
        path: PathBuf,
        ctx: &egui::Context,
        on_loaded_message: (MessageSource, AppMessage),
        sender: AppMessageSender,
        load_fn: fn(path_buf: PathBuf, ctx: &egui::Context) -> T,
    ) -> Self {
        let ctx = ctx.clone();
        let handle = thread::Builder::new()
            .name(format!("loader: {:?}", path))
            .spawn(move || {
                info!("Loading {}", path.display());

                // add a 2-second delay to simulate slow loading.
                // this is done to that thread notification can be observed in the UI; a solution is required
                // to have the UI update when loading is complete.
                thread::sleep(Duration::from_secs(1));

                let content: T = load_fn(path, &ctx);

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