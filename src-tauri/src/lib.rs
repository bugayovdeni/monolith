mod app;
use app::handler::{close_handler::close_event, menu_handler::menu_event};
use app::menu_app::setup_menu;
use services::csv_manager::CsvManager;
mod command;
mod domain;
mod services;
use command::{cmd_greet::greet, csv_command::debug_serialize, csv_command::get_csv_data};
//NOTE ИМПОРТ ТРЕЙТА MANAGER (Обязательно для работы get_webview_window)
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            debug_serialize,
            get_csv_data
        ])
        .setup(|app| {
            // Получаем главное окно по лейблу "main"
            if let Some(main_window) = app.get_webview_window("main") {
                // Показываем окно только после полной инициализации
                main_window.show()?;
                // Опционально: ставим фокус, чтобы окно было активным
                main_window.set_focus()?;
                //TODO Обработчик закрытия приложения
                close_event(&main_window);
            }

            //TODO Настройки меню
            setup_menu(app)?;

            //TODO Обработчик меню
            let _app_handle = app.handle().clone();
            menu_event(app, &_app_handle);

            //TODO Менеджер обработки сохраненных CSV файлов
            let csv_handle = CsvManager::new();
            app.manage(csv_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
