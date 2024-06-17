// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_positioner::{WindowExt, Position};
use tauri::Manager;
use std::process;
use totp::totp;

#[tauri::command]
fn abort() -> Result<(), i32> {
    process::exit(0)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.move_window(Position::Center);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![abort, totp::generate_token])
        .run(tauri::generate_context!())
        .expect("Error running the Tauri application.");
}
