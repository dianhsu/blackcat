
use std::{fs, path};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct File {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub is_dir: bool,
}

#[tauri::command]
pub fn list_folder(directory: String) -> Result<Vec<File>, String> {
  println!("Listing folder: {:?}", directory);
  let path = path::Path::new(&directory);
  if !path.exists() {
    return Err("Path does not exist".to_string());
  }
  if !path.is_dir() {
    return Err("Path is not a directory".to_string());
  }
  let mut files = vec![];
  for entry in fs::read_dir(path).unwrap() {
    files.push(File{
      name: entry.as_ref().unwrap().file_name().into_string().unwrap(),
      path: entry.as_ref().unwrap().path().display().to_string(),
      size: entry.as_ref().unwrap().metadata().unwrap().len(),
      is_dir: entry.as_ref().unwrap().metadata().unwrap().is_dir(),
    });
    println!("{:?}", entry.unwrap().path().display());
  }
  return Ok(files);
}
