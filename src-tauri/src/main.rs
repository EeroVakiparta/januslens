#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod git;
mod error;
#[cfg(test)]
mod test_utils;
#[cfg(test)]
mod tests {
    mod file_operations_test;
}

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
            git::is_git_repository,
            git::get_common_repo_locations,
            
            // File operations
            git::list_files,
            git::get_status,
            git::get_diff,
            
            // Staging and commit commands
            git::stage_file,
            git::unstage_file,
            git::create_commit,

            // Branch and merge operations
            git::checkout_branch,
            git::merge_branch
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