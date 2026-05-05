# Последовательность вызовов: сканирование Serial Ports и отправка в модальное окно

## Обзор
Документ описывает полный поток вызовов между фронтендом (TypeScript) и бэкендом (Rust) в приложении Tauri для сканирования последовательных портов и отображения их в модальном окне.

## Архитектура компонентов

### Фронтенд (TypeScript)
- **`src/windows/modules/seria-port/dialog.ts`** — класс `PortDialog`, управляющий модальным окном.
- **Шаблон и стили** — `template.ts`, `styles.css`.

### Бэкенд (Rust)
- **`src-tauri/src/commands/serial_port/`** — команды Tauri:
  - `serial_dialog.rs` — команда `open_port_dialog`.
  - `serial_list.rs` — команда `get_serial_ports`.
- **`src-tauri/src/services/serial_port/`** — сервисный слой:
  - `serial_service.rs` — функции `open_dialog` и `get_ports`.
  - `serial_scanner.rs` — функция `scan`.
- **Зависимости**: крейт `serialport` для кроссплатформенного сканирования портов.

## Полная последовательность вызовов

### 1. Инициация открытия диалога (пользователь нажимает кнопку)
```
Frontend (TypeScript) → invoke("open_port_dialog")
                      ↓
Tauri Router → команда `open_port_dialog` (serial_dialog.rs)
                      ↓
serial_service::open_dialog(app)
                      ↓
app.get_webview_window("main")?.emit("show-port-dialog", ())
                      ↓
Frontend: слушатель события "show-port-dialog" → PortDialog.open()
```

### 2. Сканирование портов при открытии диалога
```
PortDialog.open() → вызывает PortDialog.scan()
                      ↓
Frontend → invoke("get_serial_ports")
                      ↓
Tauri Router → команда `get_serial_ports` (serial_list.rs)
                      ↓
serial_service::get_ports()
                      ↓
serial_scanner::scan()
                      ↓
serialport::available_ports() → возвращает Vec<SerialPortInfo>
                      ↓
Преобразование в Vec<String> → возврат по цепочке
                      ↓
Frontend получает массив имён портов → PortDialog.renderList()
```

### 3. Отображение списка портов
- Фронтенд рендерит каждый порт как элемент `.port-item`.
- Пользователь выбирает порт кликом.
- Кнопка "Подключиться" активируется.

### 4. Подключение к выбранному порту (опционально)
- Вызов отдельной команды (не реализовано в текущем коде).

## Детализация каждого слоя

### Слой сканирования (`serial_scanner.rs`)
```rust
/// Сканирует доступные последовательные порты на системе.
pub fn scan() -> Vec<String>
```
**Внутренние вызовы:**
1. `serialport::available_ports()` — нативный вызов ОС.
2. Преобразование `SerialPortInfo.port_name` в `String`.
3. Обработка ошибок: логирование в stdout, возврат пустого вектора.

### Сервисный слой (`serial_service.rs`)
- `get_ports()` — тонкая обёртка над `scan()`.
- `open_dialog(app)` — отправка события на фронтенд.

### Слой команд Tauri
- `get_serial_ports()` — асинхронная команда, возвращает `Vec<String>`.
- `open_port_dialog()` — синхронная команда, возвращает `Ok("ok")`.

### Фронтенд (`dialog.ts`)
- **Событие `"show-port-dialog"`** — инициирует открытие модального окна.
- **Метод `scan()`** — вызывает `invoke("get_serial_ports")`, обновляет UI.
- **Рендеринг** — динамическое создание DOM-элементов для каждого порта.

## Диаграмма последовательности (текстовая)

```
Пользователь
    │
    ▼
Кнопка "Выбрать порт"
    │
    ▼
invoke("open_port_dialog")
    │
    ▼
Tauri: open_port_dialog()
    │
    ▼
serial_service::open_dialog()
    │
    ▼
emit("show-port-dialog")
    │
    ▼
PortDialog.open()
    │
    ▼
PortDialog.scan()
    │
    ▼
invoke("get_serial_ports")
    │
    ▼
Tauri: get_serial_ports()
    │
    ▼
serial_service::get_ports()
    │
    ▼
serial_scanner::scan()
    │
    ▼
serialport::available_ports()
    │
    ▼
Возврат списка портов
    │
    ▼
Рендеринг списка в UI
    │
    ▼
Пользователь выбирает порт
```

## Зависимости и ограничения

### Платформенные особенности
- **Windows**: имена портов `COM1`, `COM2`, ...
- **Linux**: `/dev/ttyUSB0`, `/dev/ttyACM0`, ...
- **macOS**: `/dev/cu.usbserial-*`, `/dev/tty.usbserial-*`.

### Требуемые разрешения
- На Linux может потребоваться членство в группе `dialout`.
- На Windows драйверы виртуальных COM-портов должны быть установлены.

### Обработка ошибок
- Сканирование: ошибки логируются, возвращается пустой список.
- Отправка события: ошибки игнорируются (используется `let _`).
- Фронтенд: ошибки сканирования отображаются в статусной строке.

## Расширение функциональности

### Добавление автоматического сканирования
- Можно запускать периодическое сканирование через `setInterval`.
- Отправлять событие обновления списка через `emit("ports-updated")`.

### Подключение к порту
- Добавить команду `connect_to_port(port: String)`.
- Использовать `serialport::new()` для открытия соединения.

### Фильтрация портов
- Исключать виртуальные порты (например, Bluetooth).
- Добавить поддержку поиска по VID/PID.

## Заключение
Текущая реализация обеспечивает базовый поток: открытие диалога → сканирование портов → отображение. Документация в коде (Rust Doc) теперь содержит детальное описание каждого шага, что упрощает поддержку и расширение функционала.

---
*Документ создан автоматически на основе анализа кода от 2026-04-22.*