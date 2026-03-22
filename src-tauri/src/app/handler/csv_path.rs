use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

/// ## Асинхронно открывает диалог и возвращает выбранный путь
///
/// Возвращает:
/// - `Some(PathBuf)` — файл выбран
/// - `None` — пользователь отменил диалог
/// - `Err(String)` — ошибка канала (маловероятно, но вдруг)
pub async fn pick_file(app: &AppHandle) -> Result<Option<PathBuf>, String> {
    let (tx, rx) = oneshot::channel();

    // Клонируем app для коллбэка — он улетает в другой контекст
    let app_handle = app.clone();

    app_handle
        .dialog()
        .file()
        .add_filter("CSV Files", &["csv"]) // 🔥 Добавляем фильтр: название + расширения
        .pick_file(move |file_path| {
            // Превращаем Tauri FilePath в стандартный PathBuf
            let path = file_path
                .as_ref()
                .and_then(|fp| fp.as_path()) // На мобилках может быть URI вместо пути
                .map(|p| p.to_path_buf());

            // Игнорируем ошибку отправки — если получатель отвалился, нам уже похуй
            let _ = tx.send(path);
        });

    // Ждём результат. Ошибка канала = что-то пошло не так на уровне потоков
    rx.await.map_err(|e| format!("Channel error: {}", e))
}
