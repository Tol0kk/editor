#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    // Register Command with Tauri App
    .invoke_handler(tauri::generate_handler![return_string])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
