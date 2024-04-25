use serde::{Deserialize, Serialize};
use std::path;
use tokio::fs;
#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
}

#[tauri::command]
pub async fn list_folder(directory: String) -> Result<Vec<File>, String> {
    let path = path::Path::new(&directory);
    if !path.exists() {
        return Err("Path does not exist".to_string());
    }
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    let mut files: Vec<File> = vec![];

    match fs::read_dir(path).await {
        Ok(mut entries) => {
          while let Ok(Some(entry)) = entries.next_entry().await {
            let metadata = entry.metadata().await.unwrap();
            let file = File {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
            };
            files.push(file);
          }
        },
        Err(_) => {
            return Err("Error reading directory".to_string());
        }
    };
    return Ok(files);   
}
