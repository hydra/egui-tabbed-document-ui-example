use std::path::PathBuf;

pub struct TextDocument {
    pub path: PathBuf,

    content: Option<String>,
}

impl TextDocument {
    pub fn create_new(path: PathBuf) -> Self {
        Self {
            path,
            content: None
        }
    }
}