use crate::app::exit_app::close as close_app;
use crate::app::handler::csv_path;
use crate::services::csv_manager::CsvManager;
use tauri::App;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;

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
                    let manager = app_handle.state::<CsvManager>();
                    match manager.load_file(&path) {
                        Ok(session_id) => {
                            //TODO
                            // 🔥 Формируем лёгкий пейлоад для события
                            let payload = serde_json::json!({
                                "session_id": session_id.to_string(),
                                "filename": path.file_name()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("unknown.csv"),
                                "status": "loaded"
                            });
                            // 🔥 Эмитим событие во все окна
                            // Имя события должно байт-в-байт совпадать с тем, на что подписан фронтенд
                            if let Err(e) = app_handle.emit("csv://loaded", payload) {
                                eprintln!("❌ Не удалось отправить событие: {}", e);
                            }
                            //FIXME
                            println!("CSV данные получены");
                        }
                        Err(e) => {
                            //TODO
                            // 🔥 На случай ошибки — тоже сообщаем фронту
                            let _ = app_handle.emit(
                                "csv://error",
                                serde_json::json!({
                                    "message": format!("Ошибка загрузки: {}", e),
                                    "status": "failed"
                                }),
                            );
                            eprintln!("❌ CSV ошибка: {}", e);
                            //FIXME
                            println!("CSV ошибка");
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
