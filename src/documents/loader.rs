use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use log::info;
use crate::app::{AppMessage, AppMessageSender, MessageSource};

enum LoaderState<T: Send + 'static, E: Send + 'static> {
    Loading(Option<JoinHandle<Result<T, E>>>),
    Loaded(T),
    Error(E),
}


pub struct DocumentContent<T: Send + 'static, E: Send + 'static> {
    state: LoaderState<T, E>,
}

impl<T: Send + 'static, E: Send + 'static> DocumentContent<T, E> {
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
    
    pub fn is_error(&self) -> bool {
        matches!(self.state, LoaderState::Error(_))
    }
    
    pub fn error(&self) -> Option<&E> {
        match &self.state {
            LoaderState::Error(error) => Some(error),
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
        load_fn: fn(path_buf: PathBuf, ctx: &egui::Context) -> Result<T, E>,
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

                let content: Result<T, E> = load_fn(path, &ctx);

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

                    match handle.join().unwrap() {
                        Ok(content) => self.state = LoaderState::Loaded(content),
                        Err(error) => self.state = LoaderState::Error(error),
                    }
                }
            }
            _ => {}
        }
    }
}