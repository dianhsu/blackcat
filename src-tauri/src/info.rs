use std::env;


#[tauri::command]
pub fn get_os_type() -> String {
    return env::consts::OS.to_string();
}