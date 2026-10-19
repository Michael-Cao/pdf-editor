// File Path: src/app.rs

use crate::core::pdf_engine::render_pdf_page_to_rgba;
use eframe::egui;
use std::path::PathBuf;

// Import our completely decoupled view panel render functions from the unified ui tree
use crate::ui::views::controls::render_control_panel;
use crate::ui::views::edit::render_edit_area;
use crate::ui::views::menu_bar::render_menu_bar;
use crate::ui::views::status_bar::render_status_bar;
use crate::ui::views::toolbar::render_toolbar;

/// Holds the active desktop runtime and GUI configuration parameters.
pub struct PdfEditorApp {
    pub file_path: Option<PathBuf>,
    pub status_message: String,
    pub target_field: String,
    pub new_value: String,
    pub page_texture: Option<egui::TextureHandle>,
    pub click_x: f32,
    pub click_y: f32,
    pub zoom_factor: f32,
    pub is_editing_inline: bool,
    pub inline_text_buffer: String,
    pub current_save_path: Option<PathBuf>,
    pub recent_files: Vec<PathBuf>,
    pub max_recent_files: usize,

    pub current_page_index: usize, // 0-indexed current page
    pub total_pages: usize,        // Total count of pages

    pub show_grid_lines: bool,
}

impl Default for PdfEditorApp {
    fn default() -> Self {
        Self {
            file_path: None,
            status_message: "No document loaded. Open a PDF or create a New one to start editing."
                .to_string(),
            target_field: "Helvetica".to_string(),
            new_value: "Click on the page to start typing directly!".to_string(),
            page_texture: None,
            click_x: 50.0,
            click_y: 600.0,
            zoom_factor: 1.0,
            is_editing_inline: false,
            inline_text_buffer: String::new(),
            current_save_path: None,
            recent_files: Vec::new(),
            max_recent_files: 20,

            current_page_index: 0,
            total_pages: 1,

            show_grid_lines: false,
        }
    }
}

impl PdfEditorApp {
    /// Loads a target file path structure and binds its visual pixels down to the active GPU container context.
    pub fn load_and_rasterize_document(&mut self, ctx: &egui::Context, path: PathBuf) {
        match render_pdf_page_to_rgba(&path) {
            Ok((width, height, rgba_bytes)) => {
                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba_bytes);
                let texture = ctx.load_texture(
                    "active_pdf_page",
                    color_image,
                    egui::TextureOptions::default(),
                );

                self.page_texture = Some(texture);
                self.file_path = Some(path.clone());
                self.current_save_path = Some(path.clone());
                self.zoom_factor = 1.0;
                self.is_editing_inline = false;

                if !self.recent_files.contains(&path) {
                    self.recent_files.insert(0, path.clone());
                    if self.recent_files.len() > 5 {
                        self.recent_files.pop();
                    }
                }

                if let Some(file_name) = self.file_path.as_ref().and_then(|p| p.file_name()) {
                    self.status_message = format!(
                        "'{}' file is loaded successfully.",
                        file_name.to_string_lossy()
                    );
                }

                self.recent_files.retain(|p| p != &path);
                self.recent_files.insert(0, path);
                self.recent_files.truncate(self.max_recent_files);

                ctx.request_repaint();
            }
            Err(err) => {
                self.status_message = format!("Rasterization Error: {err}");
            }
        }
    }
}

impl eframe::App for PdfEditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Run completely decoupled view layout components sequentially
        render_menu_bar(self, ctx, frame);
        render_status_bar(self, ctx);
        render_toolbar(self, ctx);
        render_control_panel(self, ctx);
        render_edit_area(self, ctx);
    }
}
