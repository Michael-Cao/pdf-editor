// File Path: src/ui/views/toolbar.rs

use eframe::egui;
use crate::app::PdfEditorApp;

/// Renders a utility toolbar at the top of the application window.
pub fn render_toolbar(app: &mut PdfEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_toolbar")
        .default_height(32.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🛠️ PDF Editor");
                
                ui.separator();
                
                // Editor Options Group
                ui.menu_button("Editor Options", |ui| {
                    if ui.checkbox(&mut app.show_grid_lines, "Show Grid Lines").clicked() {
                        if app.show_grid_lines {
                            app.status_message = "Grid lines enabled".to_string();
                        } else {
                            app.status_message = "Grid lines disabled".to_string();
                        }
                    }
                });

                // Inline quick-toggle for ease of use
                ui.separator();
                let grid_btn_text = if app.show_grid_lines { "🌐 Hide Grid" } else { "🌐 Show Grid" };
                if ui.selectable_label(app.show_grid_lines, grid_btn_text).clicked() {
                    app.show_grid_lines = !app.show_grid_lines;
                    app.status_message = format!("Grid lines toggled to: {}", app.show_grid_lines);
                }
            });
        });
}
