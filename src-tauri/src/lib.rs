mod command;
use command::cmd_greet::greet;
// ИМПОРТ ТРЕЙТА MANAGER (Обязательно для работы get_webview_window)
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::{AppHandle, Manager};
//диалог
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Получаем главное окно по лейблу "main"
            if let Some(main_window) = app.get_webview_window("main") {
                // Показываем окно только после полной инициализации
                main_window.show()?;
                // Опционально: ставим фокус, чтобы окно было активным
                main_window.set_focus()?;

                // 2. Вешаем обработчик закрытия НА ОКНО
                // Клонируем ссылку на окно, если понадобится внутри замыкания
                let window_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // ВАЖНО: отменяем стандартное закрытие, иначе окно сдохнет до того, как сохраняться данные
                        api.prevent_close();
                        //FIXME Удалить print
                        println!("Попытка закрытия перехвачена. Делаем свои дела...");

                        // Обработчик перехвата закрытия окна
                        let app_handle = window_clone.app_handle().clone();
                        exit_app(&app_handle);
                    }
                });
            }

            //TODO Меню
            let file_menu = SubmenuBuilder::new(app, "Файл")
                .text("open", "Открыть")
                .text("quit", "Выход")
                .build()?;

            let menu = MenuBuilder::new(app).items(&[&file_menu]).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
                println!("menu event: {:?}", event.id());
                match event.id().0.as_str() {
                    "open" => {
                        //FIXME Удалить print
                        println!("Open File");
                    }
                    "quit" => {
                        exit_app(&_app_handle);
                    }
                    _ => {
                        //FIXME Удалить print
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

///TODO Перехватчик закрытия окна
///  ==== ВЫХОД из ПРОГРАММЫ ====
///
fn exit_app(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle
        .dialog()
        .message("Подтверждаете Выход?")
        .title("Выход из Программы")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Да".to_string(),
            "Нет".to_string(),
        ))
        .show(move |result| {
            if result {
                app_handle.exit(0);
            }
        });
}
