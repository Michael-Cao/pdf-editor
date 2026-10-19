// File Path: src/ui/views/controls.rs

use eframe::egui;
use crate::app::PdfEditorApp;
use crate::core::pdf_engine::insert_text_into_pdf_content;

/// Renders the sidebar interaction menu layout framework for injecting native text streams.
pub fn render_control_panel(app: &mut PdfEditorApp, ctx: &egui::Context) {
    egui::SidePanel::left("control_panel")
        .resizable(true)
        .default_width(240.0)
        .show(ctx, |ui| {
            ui.heading("📝 Content Editor Toolbox");
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("1. Enter Text Payload:");
                ui.text_edit_multiline(&mut app.new_value);
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("2. Selected Position Target:");
                ui.small(" (Click inside the workspace canvas sheet to adjust)");
                
                ui.add(egui::Slider::new(&mut app.click_x, 0.0..=600.0).text("X Coord"));
                ui.add(egui::Slider::new(&mut app.click_y, 0.0..=800.0).text("Y Coord"));
            });

            ui.add_space(15.0);

            if ui.button("⚡ Inject Content & Save").clicked() {
                if let Some(path) = &app.file_path {
                    match insert_text_into_pdf_content(
                        path, 
                        "output_content_modified.pdf", 
                        &app.new_value, 
                        app.click_x, 
                        app.click_y, 
                        16.0
                    ) {
                        Ok(_) => app.status_message = "Success! Changes saved to 'output_content_modified.pdf'".to_string(),
                        Err(e) => app.status_message = format!("Incision Error: {e}"),
                    }
                } else {
                    app.status_message = "Failed: You must open a PDF source file template first.".to_string();
                }
            }
        });
}
