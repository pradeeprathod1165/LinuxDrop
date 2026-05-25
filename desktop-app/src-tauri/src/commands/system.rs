use crate::state::app_state::AppState;

#[tauri::command]
pub fn greet(name: &str, state: tauri::State<AppState>) -> String {
    let mut devices = state.connected_devices.lock().unwrap();

    devices.push(name.to_string());

    format!(
        "OpenBridge backend connected: {} | Total Devices: {}",
        name,
        devices.len()
    )
}