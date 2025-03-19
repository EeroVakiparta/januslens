#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod git;
mod error;

use log::info;
use std::sync::Once;
use tauri::Manager;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        env_logger::init();
        info!("JanusLens application initialized");
    });
}

fn main() {
    setup();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Repository and branch commands
            git::list_repositories,
            git::open_repository,
            git::get_branches,
            git::get_commits,
            git::create_branch,
            git::delete_branch,
            git::checkout_branch,
            
            // File operations
            git::list_files,
            git::get_status,
            git::get_diff,
            
            // Staging and commit commands
            git::stage_file,
            git::unstage_file,
            git::create_commit
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running JanusLens application");
} 