#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("OpenBridge backend connected: {}", name)
}