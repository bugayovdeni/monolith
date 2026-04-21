use tauri::{Emitter, Manager};

///
/// ### Команда \
/// Вызов модального окна \
///  выбор порта
///
#[tauri::command]
pub async fn open_port_dialog(app: tauri::AppHandle) -> Result<String, String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("show-port-dialog", ());
    }
    Ok("ok".into())
}
