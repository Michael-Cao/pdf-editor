mod core;
mod app;

use app::PdfEditorApp;

fn main() -> eframe::Result<()> {
    // Define the default viewport dimensions and parameters for the desktop client container.
    let options = eframe::NativeOptions::default();
    
    // Bootstrap the immediate mode UI event polling lifecycle thread.
    eframe::run_native(
        "PDF Editor App",
        options,
        Box::new(|_cc| Ok(Box::new(PdfEditorApp::default()))),
    )
}
