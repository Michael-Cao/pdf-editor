use std::path::PathBuf;
use pdf_oxide::editor::{DocumentEditor, SaveOptions};
use pdf_oxide::editor::form_fields::FormFieldValue;

/// Modifies a specified form field inside a PDF document and exports the result.
pub fn modify_pdf_form(source: &PathBuf, field_key: &str, field_val: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the source PDF document and map its cross-reference streams.
    let mut editor = DocumentEditor::open(source)?;
    
    // Locate the form field by its key ID and overwrite its inner text data value.
    editor.set_form_field_value(field_key, FormFieldValue::Text(field_val.to_string()))?;
    
    // Commit the adjustments back to a fresh output file using safe incremental saves.
    editor.save_with_options("output_desktop.pdf", SaveOptions::incremental())?;
    
    Ok(())
}
