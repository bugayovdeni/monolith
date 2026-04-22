#[tauri::command]
pub fn connect_port(port_name: String) -> Result<String, String> {
    crate::services::serial_port::serial_service::connect_port(&port_name)?;
    Ok(format!("Порт {port_name} открыт"))
}
