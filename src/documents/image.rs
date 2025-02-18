use crate::documents::{DocumentContext, DocumentKey};
use egui::{epaint, frame, Color32, ColorImage, Image, ImageData, TextureId, TextureOptions, Ui, Vec2, Widget};
use std::path::PathBuf;
use std::sync::Arc;
use eframe::epaint::Margin;
use egui::load::SizedTexture;
use egui_i18n::tr;
use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent};
use egui_taffy::taffy::{AlignItems, Display, FlexDirection, Size, Style};
use egui_taffy::{tui, TuiBuilderLogic};
use log::info;
use url::Url;
use crate::app::{AppMessage, AppMessageSender, MessageSource};
use crate::documents::loader::DocumentContent;

pub struct ImageDocument {
    pub path: PathBuf,

    loader: DocumentContent<egui::TextureHandle>,
}

impl ImageDocument {
    pub fn create_new(path: PathBuf, ui: &mut egui::Ui) -> Self {

        let image_data: ImageData = ImageData::Color(Arc::new(ColorImage::new([100, 100], Color32::RED)));
        
        let texture_handle = ui.ctx().load_texture(
            "my-image",
            image_data,
            Default::default()
        );

        Self {
            path,
            loader: DocumentContent::new(texture_handle),
        }
    }

    pub fn from_path(path: PathBuf, document_key: DocumentKey, sender: AppMessageSender) -> Self {
        let message = (MessageSource::Document(document_key), AppMessage::Refresh);
        let loader = DocumentContent::load(path.clone(), message, sender, |path| {
            let url = Url::from_file_path(path).unwrap();
            let uri = url.to_string();
            info!("uri: {}", uri);

            todo!()
        });

        Self {
            path,
            loader,
        }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext<'a>) {
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
    }

    fn content_ui(&mut self, ui: &mut Ui) {
        if let Some(texture) = self.loader.content_mut() {
            egui::Frame::new().show(ui, |ui|{

                let image_source = (texture.id(), texture.size_vec2());
                
                let image = Image::new(image_source);
                
                ui.add_sized(ui.available_size(), image);
                
            });
        } else {
            ui.spinner();
            ui.label(tr!("file-loading"));
        }
    }
}
