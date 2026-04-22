#[tauri::command]
pub async fn get_serial_ports() -> Vec<String> {
    crate::services::serial_port::serial_service::get_ports()
}
