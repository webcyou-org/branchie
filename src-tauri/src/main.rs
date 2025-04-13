// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use git2::Repository;
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

#[tauri::command]
fn clone_repository(url: String, path: String, username: String) -> Result<String, String> {
    // クローン先のパスをPathBufに変換
    let mut path = PathBuf::from(path);

    // リポジトリ名を抽出
    let repo_name = url
        .split('/')
        .last()
        .ok_or_else(|| "リポジトリURLが無効です".to_string())?
        .trim_end_matches(".git");

    // リポジトリ名をディレクトリ名として追加
    path = path.join(repo_name);

    // 認証情報を取得
    let password = get_credentials(username.clone())?;

    // 認証コールバックを設定
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext(&username, &password)
    });

    // フェッチオプションを設定
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // ディレクトリが存在しない場合は作成
    if !path.exists() {
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("ディレクトリの作成に失敗しました: {}", e))?;
    }

    // リポジトリの存在チェック
    let git_dir = path.join(".git");
    if git_dir.exists() && git_dir.is_dir() {
        // すでにリポジトリがある場合は pull 等の処理
        let repo = Repository::open(&path).map_err(|e| e.to_string())?;
        {
            let mut remote = repo.find_remote("origin").map_err(|e| e.to_string())?;
            remote
                .fetch(&["master"], Some(&mut fetch_options), None)
                .map_err(|e| e.to_string())?;
        }
        Ok(format!("Pulled existing repo at {:?}", path))
    } else {
        // 新しいリポジトリをクローン
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);
        builder.clone(&url, &path).map_err(|e| e.to_string())?;
        Ok(format!("Cloned new repo at {:?}", path))
    }
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
            check_repository_exists,
            clone_repository
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
