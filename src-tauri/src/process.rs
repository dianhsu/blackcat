use sysinfo;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}
#[tauri::command]
pub fn list_process() -> Vec<ProcessInfo> {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    system.processes().into_iter().map(|(pid, process)| {
        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        }
    }).collect()
}


