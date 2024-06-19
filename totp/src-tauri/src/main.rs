// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use totp::{Otp, token::*};
use db::read_tokens;

use tauri::Manager;
use tauri_plugin_positioner::{WindowExt, Position};
use totp_rs::{Algorithm, TOTP};

#[tauri::command]
fn generate_token() -> String {
    let token = env_as_token();
    let otp = Otp::new(Algorithm::SHA1, 6, 30);
    let fetch = auth(token, otp);
    let mut totp = Vec::new();

    for _ in read_tokens().into_iter() {
        let otpauth = fetch.clone();
        let url = TOTP::from_url(otpauth).unwrap();
        totp.push(url.generate_current().unwrap_or_default())
    }

    totp.join(", ")
}

#[tauri::command]
fn show_tokens() -> String {
    read_tokens().iter().map(|x| x.issuer.to_string()).collect()
}

#[tauri::command]
fn token_length() -> usize {
    read_tokens().iter().len()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.move_window(Position::Center);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_token, show_tokens, token_length])
        .run(tauri::generate_context!())
        .expect("Error running the Tauri application.");
}
