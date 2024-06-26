// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{create_tokens, read_tokens, delete_tokens, models::Token, establish_connection};
use std::env;
use totp::{token::*, Otp, TotpError};
use tauri::{Error, Manager};
use tauri_plugin_positioner::{Position, WindowExt};
use totp_rs::{Algorithm, TOTP};

#[tauri::command]
fn generate_token() -> Result<Vec<String>, TotpError> {
    let token = env_as_token();
    let otp = Otp::new(Algorithm::SHA1, 6, 30);
    let fetch = auth(token, otp);
    let mut totp = Vec::new();
    let connection = establish_connection();

    for _ in read_tokens(connection).into_iter() {
        let otpauth = fetch.clone();
        let url = TOTP::from_url(otpauth).map_err(TotpError::UrlError)?;
        totp.push(url.generate_current().map_err(TotpError::STError)?)
    }

    Ok(totp)
}

#[tauri::command]
fn show_token() -> Vec<Token> {
    let connection = establish_connection();
    read_tokens(connection).unwrap_or_default()
}

#[tauri::command(rename_all = "snake_case")]
fn submit_token(new_token: Vec<String>) -> Option<Vec<Token>> {
    let connection = establish_connection();

    Some(create_tokens(
        new_token.first()?,
        new_token[1].as_str(),
        new_token.last()?,
        connection
    ).unwrap_or_default())
}

#[tauri::command(rename_all = "snake_case")]
fn drop_token(remove_id: i32) -> usize {
    let connection = establish_connection();
    delete_tokens(remove_id, connection).unwrap_or_default()
}

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main")
                .ok_or("Tauri window couldn't be found.")?;

            window.move_window(Position::Center)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_token,
            show_token,
            submit_token,
            drop_token,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
