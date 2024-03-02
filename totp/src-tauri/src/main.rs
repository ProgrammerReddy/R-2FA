// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_positioner::{WindowExt, Position};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.move_window(Position::Center);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
