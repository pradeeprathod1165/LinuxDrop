mod commands;
mod services;
mod state;

use commands::system::greet;
use services::network_service::start_websocket_server;
use state::app_state::AppState;

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async {
        start_websocket_server().await;
    });

    tauri::Builder::default()
        .manage(AppState {
            connected_devices: Mutex::new(vec![]),
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}