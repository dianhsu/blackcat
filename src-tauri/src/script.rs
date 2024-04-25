#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Snippet {
    pub name: String,
    pub title: String,
    pub remote: bool,
    pub shell: String,
    pub script: String,
}

#[tauri::command]
pub async fn execute_script(snippets: Vec<Snippet>) -> Result<String, String> {
    println!("Executing script, snippets: {:?}", snippets);
    return Ok("Success".to_string());
}