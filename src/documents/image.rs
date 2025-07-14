use crate::documents::{DocumentContext, DocumentKey};
use egui::{frame, Color32, ColorImage, Context, Image, ImageData, ImageSource, SizeHint, TextureHandle, TextureOptions, Ui};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use eframe::epaint::Margin;
use egui::load::{SizedTexture, TexturePoll};
use egui_i18n::tr;
use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent};
use egui_taffy::taffy::{AlignItems, Display, FlexDirection, Size, Style};
use egui_taffy::{tui, TuiBuilderLogic};
use image::ImageReader;
use image::GenericImageView;
use log::{debug, error, info};
use url::Url;
use crate::app::{AppMessage, AppMessageSender, MessageSource};
use crate::documents::loader::DocumentContent;

pub struct ImageDocument {
    pub path: PathBuf,

    loader: DocumentContent<(Url, TextureHandle), ImageLoaderError>,
    context: Context,
}

enum ImageLoaderError {
    Error
}

impl Drop for ImageDocument {
    fn drop(&mut self) {
        self.forget_existing_image()
    }
}

impl ImageDocument {
    pub fn create_new(path: PathBuf, ctx: &Context) -> Self {

        let url = Url::from_file_path(path.clone()).unwrap();
        info!("creating image. uri: {}", url);

        let image_data: ImageData = ImageData::Color(Arc::new(ColorImage::filled([100, 100], Color32::RED)));

        let texture_handle = ctx.load_texture(
            url.as_str(),
            image_data,
            Default::default()
        );

        Self {
            path,
            context: ctx.clone(),
            loader: DocumentContent::new((url, texture_handle)),
        }
    }

    pub fn from_path(path: PathBuf, ctx: &Context, document_key: DocumentKey, sender: AppMessageSender) -> Self {
        let message = (MessageSource::Document(document_key), AppMessage::Refresh);
        let loader = DocumentContent::load(path.clone(), ctx, message, sender, move |path, ctx| {
            fn load_image_from_file_using_image_crate(ctx: &Context, path: PathBuf, url: Url) -> Option<(Url, TextureHandle)> {
                // Open and decode the image
                let img = ImageReader::open(path).ok()?.decode().ok()?;
                let size = img.dimensions();

                // Convert image to RGBA8
                let rgba = img.to_rgba8();
                let pixels = rgba.as_flat_samples();

                // Create an egui ColorImage
                let color_image = egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], pixels.as_slice());

                // Load the texture into egui, we use the url, so we can forget it later, forgetting an image requires a url, not a path...
                let texture_handle = ctx.load_texture(url.as_str(), color_image, TextureOptions::default());
                    
                Some((url, texture_handle))
            }
            fn load_image_from_file_using_egui_extras(ctx: &Context, _path: PathBuf, url: Url) -> Option<(Url, TextureHandle)> {
                let texture = loop {
                    let poll = ctx.try_load_texture(url.as_str(), TextureOptions::default(), SizeHint::default()).ok()?;
                    match poll {
                        TexturePoll::Pending { .. } => {
                            debug!("texture poll pending");
                            thread::sleep(Duration::from_millis(100));
                        }
                        TexturePoll::Ready { texture } => {
                            debug!("texture poll ready");
                            break texture;
                        }
                    }
                };

                // very important that the id is given to the texture manager to prevent double-free/use-after-free panics.
                ctx.tex_manager().write().retain(texture.id);
                let texture_handle = TextureHandle::new(ctx.tex_manager(), texture.id);
                Some((url, texture_handle))
            }

            let url = Url::from_file_path(path.clone()).unwrap();
            info!("uri: {}", url);

            //let result = load_image_from_file_using_image_crate(ctx, path, url);
            let result = load_image_from_file_using_egui_extras(ctx, path, url);
            match result {
                None => {
                    error!("Failed to load image");
                    Err(ImageLoaderError::Error)
                }
                Some(result) => {
                    info!("Image loaded. texture_id: {:?}", result.1.id());
                    Ok(result)
                }
            }
        });

        Self {
            path,
            loader,
            context: ctx.clone(),
        }
    }

    fn forget_existing_image(&mut self) {
        if let Some((uri, _existing_texture)) = self.loader.take() {
            debug!("forgetting existing image. uri: {}", uri);
            
            // forget the image so that the image is loaded from disk again.
            self.context.forget_image(uri.as_str());
        }
    }

    pub fn ui<'a>(&mut self, ui: &mut Ui, _context: &mut DocumentContext) {
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

        egui::SidePanel::left(ui.id().with("sidebar"))
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
        if self.loader.is_error() {
            ui.label(tr!("file-loading-error"));
        } else {
            if let Some((_url, texture_handle)) = self.loader.content_mut() {
                egui::Frame::new().show(ui, |ui| {
                    let image_source = ImageSource::Texture(SizedTexture::from_handle(&texture_handle));
                    let image = Image::new(image_source);

                    ui.add_sized(ui.available_size(), image);
                });
            } else {
                ui.spinner();
                ui.label(tr!("file-loading"));
            }
        }
    }
}
