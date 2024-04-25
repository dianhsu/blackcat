// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use config::{update_config, CONFIG};
use file::list_folder;
use info::get_os_type;
use process::list_process;
use script::execute_script;
use tracing::debug;
mod config;
mod file;
mod info;
mod process;
mod script;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_os_type,
            execute_script,
            list_folder,
            list_process
        ])
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    debug!("cli matches: {:?}", matches);
                    update_config(matches)
                }
                Err(_) => {}
            }
            if CONFIG.debug {
                tracing_subscriber::fmt::init();
            }

            config::check_config();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
