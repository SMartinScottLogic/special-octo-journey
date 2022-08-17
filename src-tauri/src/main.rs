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
  entries: Vec<(bool, String)>,
}

#[tauri::command]
fn hello(root: &str) -> Result<String, String> {
  // This is a very simplistic example but it shows how to return a Result
  // and use it in the front-end.
  if root.contains(' ') {
    Err("Name should not contain spaces".to_string())
  } else {
    let entry = match std::fs::read_dir(root) {
      Ok(entry) => entry,
      Err(e) => return Err(format!(r#"Failed to read dir {} : {}"#, root, e)),
    };
    let entries = entry.flat_map(|e| e.map(|entry| (entry.file_type().map(|t| t.is_dir()).unwrap_or(false), entry.file_name().to_string_lossy().to_string())))
        .collect();
    let payload = serde_json::json!(Payload { entries }).to_string();
    println!("root {root}, entries: {payload}");
    Ok(payload)
  }
}
