#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Read;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_dir, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    entries: Vec<(bool, String)>,
}

#[tauri::command]
fn read_dir(root: &str) -> Result<String, String> {
    let entry = match std::fs::read_dir(root) {
        Ok(entry) => entry,
        Err(e) => return Err(format!(r#"Failed to read dir {} : {}"#, root, e)),
    };
    let entries = entry
        .flat_map(|e| {
            e.map(|entry| {
                (
                    entry.file_type().map(|t| t.is_dir()).unwrap_or(false),
                    entry.file_name().to_string_lossy().to_string(),
                )
            })
        })
        .collect();
    let payload = serde_json::json!(Payload { entries }).to_string();
    Ok(payload)
}

#[tauri::command]
fn read_file(filename: &str) -> Result<String, String> {
    let mut file = File::open(filename).map_err(|e| format!(r#"Failed to open '{}': {}"#, filename, e))?;
    let mut buf = vec![];
    file.read_to_end (&mut buf).map_err(|e| format!(r#"Failed to read '{}': {}"#, filename, e))?;
    let contents = String::from_utf8_lossy (&buf);

    Ok(contents.to_string())
}
