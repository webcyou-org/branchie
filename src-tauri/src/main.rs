// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
// use tauri::api::dialog::FileDialogBuilder;

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

// #[tauri::command]
// async fn select_directory() -> Result<String, String> {
//     let path = FileDialogBuilder::new()
//         .set_title("Select Directory")
//         .pick_folder()
//         .await
//         .ok_or_else(|| "ディレクトリが選択されませんでした".to_string())?
//         .to_string_lossy()
//         .to_string();

//     Ok(path)
// }

#[tauri::command]
fn validate_repository_path(path: String) -> Result<bool, String> {
    let path = PathBuf::from(path);

    // パスが存在するかチェック
    if !path.exists() {
        return Ok(false);
    }

    // ディレクトリかどうかチェック
    if !path.is_dir() {
        return Ok(false);
    }

    // 書き込み権限があるかチェック
    // if !path
    //     .metadata()
    //     .map_err(|e| format!("メタデータの取得に失敗しました: {}", e))?
    //     .permissions()
    //     .readonly()
    // {
    //     return Ok(false);
    // }

    Ok(true)
}

#[tauri::command]
fn check_repository_exists(path: String) -> Result<bool, String> {
    let path = PathBuf::from(path);

    // .gitディレクトリが存在するかチェック
    let git_dir = path.join(".git");
    Ok(git_dir.exists() && git_dir.is_dir())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_credentials,
            get_credentials,
            get_default_repository_path,
            // select_directory,
            validate_repository_path,
            check_repository_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
