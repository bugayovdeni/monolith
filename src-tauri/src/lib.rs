mod app;
use app::handler::{close_handler::close_event, menu_handler::menu_event};
use app::menu_app::setup_menu;
use services::csv_manager::CsvManager;
mod commands;
mod domain;
mod services;
use commands::csv::{csv_command::debug_serialize, csv_command::get_csv_data};
use commands::serial_port::{
    serial_ascii::start_ascii_stream_command, serial_connect::connect_port,
    serial_dialog::open_port_dialog, serial_list::get_serial_ports,
};
//NOTE ИМПОРТ ТРЕЙТА MANAGER (Обязательно для работы get_webview_window)
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            debug_serialize,
            get_csv_data,
            open_port_dialog,
            get_serial_ports,
            connect_port,
            start_ascii_stream_command
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
