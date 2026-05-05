use super::serial_scanner;
use tauri::{AppHandle, Emitter, Manager};

/// Открывает модальное окно выбора порта на фронтенде.
///
/// # Параметры
/// - `app`: экземпляр `AppHandle` Tauri для доступа к окнам.
///
/// # Действие
/// Ищет главное окно с идентификатором `"main"` и отправляет событие `"show-port-dialog"`.
/// Фронтенд (TypeScript) слушает это событие и показывает модальное окно.
///
/// # Последовательность вызовов
/// 1. Получает главное окно через `app.get_webview_window("main")`.
/// 2. Если окно найдено, отправляет событие `"show-port-dialog"` без данных.
/// 3. Фронтенд (`dialog.ts`) обрабатывает событие и вызывает `PortDialog.open()`.
///
/// # Возвращаемое значение
/// Функция ничего не возвращает. Ошибки отправки события игнорируются (используется `let _`).
pub fn open_dialog(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("show-port-dialog", ());
    }
}

/// Возвращает список доступных последовательных портов.
///
/// # Возвращаемое значение
/// Вектор строк с именами портов, полученный вызовом `serial_scanner::scan()`.
///
/// # Последовательность вызовов
/// 1. Вызывает `serial_scanner::scan()` для сканирования портов.
/// 2. Возвращает результат напрямую.
///
/// # Примечания
/// Функция является тонкой обёрткой над сканером и используется командами Tauri.
pub fn get_ports() -> Vec<String> {
    serial_scanner::scan()
}

pub fn connect_port(port_name: &str) -> Result<(), String> {
    let _port = monolith_serial::port::open_port(port_name).map_err(|e| e.to_string())?;

    Ok(())
}
