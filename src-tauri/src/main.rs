// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use config::{update_config, CONFIG};
use tracing::debug;
use file::list_folder;

mod file;
mod config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn get_os_type() -> String {
  return env::consts::OS.to_string();
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Snippet {
  pub name: String,
  pub title: String,
  pub remote: bool,
  pub shell: String,
  pub script: String
}

#[tauri::command]
async fn execute_script(snippets: Vec<Snippet>) -> Result<String, String> {
  println!("Executing script, snippets: {:?}", snippets);
  return Ok("Success".to_string());
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_os_type, execute_script, list_folder])
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
