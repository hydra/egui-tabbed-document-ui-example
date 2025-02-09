#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod file_picker;
pub mod fonts;
pub mod i18n;
pub mod context;
pub use app::TemplateApp;
