// File Path: src/ui/views/edit.rs

use eframe::egui;
use crate::app::PdfEditorApp;
use crate::core::pdf_engine::insert_text_into_pdf_content;

/// Renders an active graphical workspace that spawns inline input editors directly over the PDF document.
pub fn render_edit_area(app: &mut PdfEditorApp, ctx: &egui::Context) {
    
    // Process Global Keyboard Hotkeys Input Event Listeners for Scale Changes
    ctx.input_mut(|i| {
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Equals)) {
            app.zoom_factor *= 1.15;
            app.status_message = format!("Zoom level scaled up to: {:.0}%", app.zoom_factor * 100.0);
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Minus)) {
            app.zoom_factor *= 0.85;
            if app.zoom_factor < 0.15 { app.zoom_factor = 0.15; }
            app.status_message = format!("Zoom level scaled down to: {:.0}%", app.zoom_factor * 100.0);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if let Some(texture) = &app.page_texture {
                    
                    let native_size = texture.size_vec2();
                    let available_height = ui.available_height();
                    let height_fit_ratio = available_height / native_size.y;
                    let target_display_scale = height_fit_ratio * app.zoom_factor;
                    let display_size = native_size * target_display_scale;

                    ui.centered_and_justified(|ui| {
                        
                        // Fit image container to exact display size based on current layout version specs
                        let image_widget = egui::Image::new(texture)
                            .fit_to_exact_size(display_size)
                            .sense(egui::Sense::click());
                            
                        let response = ui.add(image_widget);

                        // Intercept Click Events to Activate the Inline Editor Focus
                        if response.clicked() {
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                let relative_x = pointer_pos.x - response.rect.min.x;
                                let relative_y = pointer_pos.y - response.rect.min.y;

                                app.click_x = relative_x / target_display_scale;
                                app.click_y = native_size.y - (relative_y / target_display_scale);

                                app.is_editing_inline = true;
                                app.inline_text_buffer.clear(); 
                            }
                        }

                        // THE EDITOR LAYER: Render an overlay active input input widget precisely above the PDF coordinates
                        if app.is_editing_inline {
                            let screen_x = response.rect.min.x + (app.click_x * target_display_scale);
                            let screen_y = response.rect.min.y + ((native_size.y - app.click_y) * target_display_scale);
                            let inline_area_pos = egui::pos2(screen_x, screen_y);
                            
                            egui::Window::new("inline_text_editor")
                                .fixed_pos(inline_area_pos)
                                .title_bar(false) 
                                .frame(egui::Frame::window(&ui.style()).fill(egui::Color32::from_black_alpha(220)))
                                .resizable(false)
                                .show(ctx, |ui| {
                                    let text_input = ui.text_edit_singleline(&mut app.inline_text_buffer);
                                    text_input.request_focus();

                                    if text_input.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        if !app.inline_text_buffer.is_empty() {
                                            if let Some(path) = &app.file_path {
                                                match insert_text_into_pdf_content(
                                                    path,
                                                    "output_content_modified.pdf",
                                                    &app.inline_text_buffer,
                                                    app.click_x,
                                                    app.click_y,
                                                    16.0
                                                ) {
                                                    Ok(_) => app.status_message = format!("Text content written straight into PDF at X:{:.1} Y:{:.1}!", app.click_x, app.click_y),
                                                    Err(e) => app.status_message = format!("Incision Error: {e}"),
                                                }
                                            }
                                        }
                                        app.is_editing_inline = false;
                                    }
                                });
                        }

                        // Draw visual guide circle matching active cursor positioning target inputs
                        if app.is_editing_inline {
                            let painter = ui.painter();
                            let target_screen_pos = egui::pos2(
                                response.rect.min.x + (app.click_x * target_display_scale),
                                response.rect.min.y + ((native_size.y - app.click_y) * target_display_scale)
                            );
                            painter.circle_filled(target_screen_pos, 4.0, egui::Color32::GREEN);
                        }
                    });

                } else {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Select 'File -> Open PDF...' to view and edit a document.\nClick anywhere directly on the sheet canvas to write text object structures.");
                    });
                }
            });
    });
}
