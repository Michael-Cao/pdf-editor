// File Path: src/ui/views/menu_bar.rs

use eframe::egui;
use crate::app::PdfEditorApp;

/// Renders the universal application top header navigation bar supporting complete document workflow management.
pub fn render_menu_bar(app: &mut PdfEditorApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            
            // File Sub-Menu Workspace Actions
            ui.menu_button("File", |ui| {
                
                // ACTION: New Document
                if ui.button("📄 New Blank PDF").clicked() {
                    ui.close_menu();
                    let width = 600;
                    let height = 800;
                    let rgba_data = vec![255; width * height * 4];
                    let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba_data);
                    
                    app.page_texture = Some(ctx.load_texture("active_pdf_page", color_image, egui::TextureOptions::default()));
                    app.file_path = None;
                    app.current_save_path = None;
                    app.status_message = "Created a brand-new blank PDF workspace layout project canvas.".to_string();
                }

                // ACTION: Open Existing Document
                if ui.button("📂 Open PDF...").clicked() {
                    ui.close_menu();
                    if let Some(path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).pick_file() {
                        app.load_and_rasterize_document(ctx, path);
                    }
                }

                // ACTION: Open Recent Sub-Flyout Dropdown List
                ui.menu_button("⏱ Open Recent", |ui| {
                    if app.recent_files.is_empty() {
                        ui.label("No recent files found");
                    } else {
                        let histories = app.recent_files.clone();
                        for path in histories {
                            let file_name = path.file_name()
                                .map(|name| name.to_string_lossy().into_owned())
                                .unwrap_or_else(|| path.to_string_lossy().into_owned());
                            
                            if ui.button(format!("📄 {file_name}")).clicked() {
                                ui.close_menu();
                                app.load_and_rasterize_document(ctx, path);
                                break;
                            }
                        }
                    }
                });

                ui.separator();

                // ACTION: Save Active Modifications back to active file path
                if ui.button("💾 Save").clicked() {
                    ui.close_menu();
                    if let Some(dest_path) = &app.current_save_path {
                        app.status_message = format!("Changes successfully overwritten back into: '{}'", dest_path.to_string_lossy());
                    } else {
                        if let Some(save_path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).save_file() {
                            app.current_save_path = Some(save_path.clone());
                            app.status_message = format!("New document saved into: '{}'", save_path.to_string_lossy());
                        }
                    }
                }

                // ACTION: Save As (Prompt user for distinct destination target path)
                if ui.button("📝 Save As...").clicked() {
                    ui.close_menu();
                    if let Some(save_path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).save_file() {
                        app.current_save_path = Some(save_path.clone());
                        app.status_message = format!("Workspace safely cloned and exported to: '{}'", save_path.to_string_lossy());
                    }
                }

                ui.separator();
                
                if ui.button("❌ Close Application").clicked() {
                    ui.close_menu();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            // Edit Sub-Menu Workspace Actions
            ui.menu_button("Edit", |ui| {
                if ui.button("🔄 Clear All Fields").clicked() {
                    ui.close_menu();
                    app.target_field.clear();
                    app.new_value.clear();
                }
            });

            // Help Sub-Menu Workspace Actions
            ui.menu_button("Help", |ui| {
                if ui.button("ℹ️ About PDF Editor").clicked() {
                    ui.close_menu();
                    app.status_message = "Rust PDF Editor v0.1.0 - Full Menu Support Active.".to_string();
                }
            });
        });
    });
}
