#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod git;
mod error;

use log::{error, info};
use std::sync::Once;

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
        .invoke_handler(tauri::generate_handler![
            git::list_repositories,
            git::open_repository,
            git::get_branches,
            git::get_commits
        ])
        .run(tauri::generate_context!())
        .expect("Error while running JanusLens application");
} 