use tauri::AppHandle;

///
/// ### Команда \
/// Вызов модального окна \
///  выбор порта
///

#[tauri::command]
pub fn open_port_dialog(app: AppHandle) -> Result<String, String> {
    crate::services::serial_port::serial_service::open_dialog(app);
    Ok("ok".into())
}
