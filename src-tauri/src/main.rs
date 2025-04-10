// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use keyring::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[tauri::command]
fn save_credentials(credentials: Credentials) -> Result<(), String> {
    let entry = Entry::new("Branchie", &credentials.username).map_err(|e| e.to_string())?;

    entry
        .set_password(&credentials.password)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_credentials(username: String) -> Result<String, String> {
    let entry = Entry::new("Branchie", &username).map_err(|e| e.to_string())?;

    entry.get_password().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_credentials, get_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
