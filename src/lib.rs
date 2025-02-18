#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod context;
pub mod documents;
pub mod file_picker;
pub mod fonts;
pub mod i18n;
pub use app::TemplateApp;
