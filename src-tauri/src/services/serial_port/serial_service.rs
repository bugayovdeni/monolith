use super::serial_scanner;
use tauri::{AppHandle, Emitter, Manager};

pub fn open_dialog(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("show-port-dialog", ());
    }
}

pub fn get_ports() -> Vec<String> {
    serial_scanner::scan()
}
