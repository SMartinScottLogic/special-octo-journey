#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: Vec<String>,
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
  // This is a very simplistic example but it shows how to return a Result
  // and use it in the front-end.
  if name.contains(' ') {
    Err("Name should not contain spaces".to_string())
  } else {
    let entry = match std::fs::read_dir(".") {
      Ok(entry) => entry,
      Err(e) => return Err(format!(r#"Failed to read dir "." : {}"#, e)),
    };
    let s = entry.flat_map(|e| e.map(|entry| entry.file_name()))
        .map(|f| f.to_string_lossy().to_string())
        .collect();
    Ok(serde_json::json!(Payload { message: s}).to_string())
  }
}
