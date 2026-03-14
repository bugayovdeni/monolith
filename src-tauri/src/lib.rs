mod command;
use command::cmd_greet::greet;
// ИМПОРТ ТРЕЙТА MANAGER (Обязательно для работы get_webview_window)
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Получаем главное окно по лейблу "main"
            if let Some(main_window) = app.get_webview_window("main") {
                // Показываем окно только после полной инициализации
                main_window.show()?;

                // Опционально: ставим фокус, чтобы окно было активным
                main_window.set_focus()?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
