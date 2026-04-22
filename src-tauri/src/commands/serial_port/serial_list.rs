/// Команда Tauri для получения списка доступных последовательных портов.
///
/// # Возвращаемое значение
/// `Vec<String>` — список имён портов.
///
/// # Последовательность вызовов
/// 1. Фронтенд (TypeScript) вызывает `invoke("get_serial_ports")`.
/// 2. Tauri маршрутизирует вызов в эту команду.
/// 3. Команда вызывает `serial_service::get_ports()`.
/// 4. `serial_service::get_ports()` вызывает `serial_scanner::scan()`.
/// 5. Сканер использует `serialport::available_ports()` для получения списка портов.
/// 6. Результат возвращается через цепочку обратно на фронтенд.
///
/// # Пример использования на фронтенде
/// ```typescript
/// const ports = await invoke<string[]>("get_serial_ports");
/// console.log(ports); // ["COM1", "COM3"]
/// ```
///
/// # Примечания
/// - Команда асинхронная, но фактически выполняет синхронную работу.
/// - Используется в модальном окне выбора порта при сканировании.
#[tauri::command]
pub async fn get_serial_ports() -> Vec<String> {
    crate::services::serial_port::serial_service::get_ports()
}
