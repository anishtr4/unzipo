// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use zip::read::ZipArchive;
use serde::Deserialize;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hellssffsfo, {}!", name)
}


#[derive(Deserialize)]
struct ExtractRequest {
    zip_file_path: String,
    target_dir: String,
}

#[tauri::command]
fn extract_zip(direcorty: &str, zippath: &str) -> Result<String, String> {
    let zip_file_path = PathBuf::from(zippath);
    let target_dir = PathBuf::from(direcorty);

    let file = File::open(zip_file_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = file.sanitized_name();
        let outpath = target_dir.to_owned().join(outpath);

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok("Extraction successful!".to_string())
}



fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![greet,extract_zip])
  .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
