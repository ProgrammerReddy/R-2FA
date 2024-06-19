// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process, env};
use totp::{Token, Otp};
use db::*;

use tauri::Manager;
use tauri_plugin_positioner::{WindowExt, Position};
use totp_rs::{Algorithm, TOTP};
use dotenv::dotenv;

#[tauri::command]
fn abort() -> Result<(), i32> {
    process::exit(0)
}

fn env_as_token() -> Token {
    let _ = dotenv().ok();
    let issuer = read_tokens_issuer().join(", ");
    let account_name = read_tokens_account_name();
    let secret = read_tokens_secret();

    Token::new(issuer, account_name, secret)
}

fn auth(token: Token, otp: Otp) -> String {
    format!(
        "otpauth://totp/{}:{}@?secret={}&issuer={}&algorithm={}&digits={}&period={}", 
        token.issuer, 
        token.account_name, 
        token.secret, 
        token.issuer,
        otp.algorithm,
        otp.digits,
        otp.step
    )
}

#[tauri::command]
fn generate_token() -> String {
    let token = env_as_token();
    let otp = Otp::new(Algorithm::SHA1, 6, 30);
    let otpauth = auth(token, otp);
    let totp = TOTP::from_url(otpauth).unwrap();
    
    let generate = read_tokens_id().into_iter().map(|_| {
        totp.generate_current().unwrap_or_default()
    }).collect::<Vec<String>>();
    
    generate.join(", ")
}

#[tauri::command]
fn show_tokens() -> String {
    read_tokens_issuer().join(", ")
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.move_window(Position::Center);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![abort, generate_token, show_tokens])
        .run(tauri::generate_context!())
        .expect("Error running the Tauri application.");
}
