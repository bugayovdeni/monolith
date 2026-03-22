use crate::app::exit_app::close as close_app;
use crate::app::handler::csv_path;
use crate::services::csv_services::csv_manager::run as run_manager;
use tauri::App;
use tauri::AppHandle;

///
/// ## Обработчик закрытия приложения
/// из меню \
///
/// *параметры* \
/// `App`\
/// `AppHandle`

pub fn menu_event(app: &App, app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app.on_menu_event(move |_app_handle, event| {
        let app_handle = app_handle.clone();
        //FIXME println!
        println!("menu event: {:?}", event.id());
        match event.id().0.as_str() {
            "open" => {
                // 🔥 СПАВНИМ асинхронную задачу
                // Используем tauri::async_runtime, чтобы не тащить лишний tokio
                tauri::async_runtime::spawn(async move {
                    // 1. Получаем путь (мой красивый await)
                    let path = match csv_path::pick_file(&app_handle).await {
                        Ok(Some(p)) => {
                            //FIXME println!
                            println!("путь получен {}", p.display());
                            p
                        }
                        Ok(None) => return, // Юзер нажал "Отмена" — просто выходим
                        Err(e) => {
                            //FIXME println!
                            println!("Dialog error: {}", e);
                            return;
                        }
                    };

                    // 2. Читаем и валидим (синхронная операция, но в отдельном потоке)
                    // Блокировка тут не страшна, так как мы не в главном UI-потоке
                    match run_manager(&path) {
                        Ok(data) => {
                            //TODO
                            // 3. Успех — шлём данные во фронтенд
                            // ui_event::send_success(&app_handle, data);
                        }
                        Err(e) => {
                            //TODO
                            // 4. Ошибка валидации/чтения — шлём ошибку
                            // ui_event::send_error(&app_handle, format!("CSV Error: {}", e));
                        }
                    }
                });
            }
            "quit" => {
                //TODO Перехватчик закрытия окна
                close_app(&_app_handle);
            }
            _ => {
                //FIXME Удалить print
                println!("unexpected menu event");
            }
        }
    });
}
