use std::sync::Mutex;

pub struct AppState {
    pub connected_devices: Mutex<Vec<String>>,
}