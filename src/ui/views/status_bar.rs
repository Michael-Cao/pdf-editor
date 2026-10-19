// File Path: src/ui/views/status_bar.rs

use eframe::egui;
use crate::app::PdfEditorApp;

/// Renders a specialized utility footer panel at the absolute bottom of the application window.
pub fn render_status_bar(app: &PdfEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_footer_bar")
        .default_height(24.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left side: Status Message
                ui.colored_label(
                    egui::Color32::from_rgb(140, 180, 240), 
                    format!("📋 Status: {}", app.status_message)
                );
                
                // Right side items (processed right-to-left)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 1. Rightmost item: Cursor Anchor
                    ui.label(format!("Cursor Anchor: [X: {:.1}, Y: {:.1}]", app.click_x, app.click_y));
                    
                    // Add a small visual separator
                    ui.separator();

                    // 2. Middle-right item: Page Counter (e.g., "Page 1 / 5")
                    // Note: If your app uses 0-indexed values, add 1 to the current page.
                    let current_page = app.current_page_index + 1;
                    let total_pages = app.total_pages; 
                    
                    ui.label(format!("📄 Page {} / {}", current_page, total_pages));
                });
            });
        });
}
