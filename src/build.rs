use std::env;
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

fn main() {
    // Tell Cargo to re-run this script only if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");

    // Define the download URL for the target Windows x86_64 PDFium binary release
    let pdfium_url = "https://github.com";

    // Determine the root target directory where compilation outputs are written
    let out_dir = env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3) // Walk up the folder tree to reach the target/debug or target/release folder
        .expect("Failed to locate target destination paths");

    let destination_path = root_dir.join("pdfium.dll");

    // Download the library file only if it does not exist inside the compilation output folder
    if !destination_path.exists() {
        println!(
            "cargo:warning=pdfium.dll missing! Starting automatic background download sequence..."
        );

        if let Err(e) = download_and_extract_pdfium(pdfium_url, &destination_path) {
            panic!(
                "Failed to auto-download PDFium binary dependency asset: {:?}",
                e
            );
        }

        println!("cargo:warning=pdfium.dll successfully placed into build destination directory.");
    }
}

/// Fetches the ZIP archive over HTTPS, parses the inner payload, and writes the DLL down to disk.
fn download_and_extract_pdfium(
    url: &str,
    dest_dll_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Download the remote ZIP archive into a synchronous memory buffer wrapper
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(format!("Server returned error state: {}", response.status()).into());
    }

    let zip_bytes = response.bytes()?;
    let reader = Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(reader)?;

    // 2. Loop through files in the archive to isolate the compiled binary module
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name();

        // Target the precise nested location of the dynamic library file item
        if file_name == "bin/pdfium.dll" {
            let mut output_file = File::create(dest_dll_path)?;
            io::copy(&mut file, &mut output_file)?;
            return Ok(());
        }
    }

    Err(
        "Could not locate bin/pdfium.dll asset entry within downloaded ZIP container archive."
            .into(),
    )
}
