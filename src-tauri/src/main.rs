// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[tauri::command]
fn get_default_repository_path() -> Result<String, String> {
    let data_dir =
        dirs::data_dir().ok_or_else(|| "データディレクトリが見つかりませんでした".to_string())?;

    let repo_dir = data_dir.join("Branchie").join("repository");

    // ディレクトリが存在しない場合は作成
    if !repo_dir.exists() {
        std::fs::create_dir_all(&repo_dir)
            .map_err(|e| format!("ディレクトリの作成に失敗しました: {}", e))?;
    }

    Ok(repo_dir.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_credentials,
            get_credentials,
            get_default_repository_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
