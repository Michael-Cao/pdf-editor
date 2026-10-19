use eframe::egui;
use std::path::PathBuf;
use crate::core::pdf_engine::modify_pdf_form;

/// Holds the active desktop runtime and GUI configuration parameters.
pub struct PdfEditorApp {
    pub file_path: Option<PathBuf>,
    pub status_message: String,
    pub target_field: String,
    pub new_value: String,
}

impl Default for PdfEditorApp {
    fn default() -> Self {
        Self {
            file_path: None,
            status_message: "Ready to load document.".to_string(),
            target_field: "employee_name".to_string(),
            new_value: String::new(),
        }
    }
}

impl eframe::App for PdfEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw the primary centralized workbench user view.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦀 Rust Desktop PDF Editor");
            ui.add_space(8.0);
            
            // Layout the file selection system.
            ui.horizontal(|ui| {
                if ui.button("📂 Open PDF Template").clicked() {
                    // Launch an asynchronous native file window explorer dialog.
                    if let Some(path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).pick_file() {
                        self.file_path = Some(path);
                        self.status_message = "File loaded into workspace successfully.".to_string();
                    }
                }
                
                if let Some(path) = &self.file_path {
                    ui.label(format!("Active: {}", path.to_string_lossy()));
                }
            });

            ui.separator();
            ui.add_space(8.0);

            // Collect target structural form transformation coordinates from the user.
            ui.label("Target Form Keys:");
            ui.text_edit_singleline(&mut self.target_field);
            
            ui.label("Replacement Value Text:");
            ui.text_edit_singleline(&mut self.new_value);

            ui.add_space(12.0);

            // Execute the backend binary mutation pipeline loop.
            if ui.button("💾 Export Modified PDF").clicked() {
                if let Some(path) = &self.file_path {
                    match modify_pdf_form(path, &self.target_field, &self.new_value) {
                        Ok(_) => self.status_message = "File updated successfully!".to_string(),
                        Err(e) => self.status_message = format!("Processing Error: {e}"),
                    }
                } else {
                    self.status_message = "Failed: No source file selected.".to_string();
                }
            }

            ui.add_space(12.0);
            
            // Output operational feedback telemetry to the footer grid.
            ui.label(&self.status_message);
        });
    }
}
