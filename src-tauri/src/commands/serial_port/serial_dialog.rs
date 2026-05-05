use tauri::AppHandle;

/// Команда Tauri для открытия модального окна выбора последовательного порта.
///
/// # Параметры
/// - `app`: экземпляр `AppHandle` для доступа к окнам приложения.
///
/// # Возвращаемое значение
/// `Result<String, String>` — всегда `Ok("ok")` при успешной отправке события.
///
/// # Последовательность вызовов
/// 1. Фронтенд (TypeScript) вызывает `invoke("open_port_dialog")`.
/// 2. Tauri маршрутизирует вызов в эту команду.
/// 3. Команда вызывает `serial_service::open_dialog(app)`.
/// 4. `open_dialog` ищет главное окно `"main"` и отправляет событие `"show-port-dialog"`.
/// 5. Фронтенд (`dialog.ts`) слушает событие и вызывает `PortDialog.open()`.
/// 6. Модальное окно появляется на экране.
///
/// # Пример использования на фронтенде
/// ```typescript
/// await invoke("open_port_dialog");
/// // Окно диалога появится автоматически
/// ```
///
/// # Примечания
/// - Команда синхронная, но событие отправляется асинхронно внутри Tauri.
/// - Ошибки отправки события игнорируются, команда всегда возвращает `Ok`.
#[tauri::command]
pub fn open_port_dialog(app: AppHandle) -> Result<String, String> {
    crate::services::serial_port::serial_service::open_dialog(app);
    Ok("ok".into())
}
