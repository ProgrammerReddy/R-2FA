// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{create_tokens, read_tokens, delete_tokens, models::Token};
use std::env;
use totp::{token::*, Otp};
use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};
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
fn show_token() -> Vec<Token> {
    read_tokens()
}

#[tauri::command(rename_all = "snake_case")]
fn submit_token(new_token: Vec<String>) {
    create_tokens(
        new_token.first().unwrap(),
        new_token[1].as_str(),
        new_token.last().unwrap(),
    );
}

#[tauri::command(rename_all = "snake_case")]
fn drop_token(remove_id: i32) -> usize {
    delete_tokens(remove_id)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.move_window(Position::Center).unwrap_or_default();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_token,
            show_token,
            submit_token,
            drop_token,
        ])
        .run(tauri::generate_context!())
        .expect("Error running the Tauri application.");
}
