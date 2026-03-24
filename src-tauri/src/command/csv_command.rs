use crate::domain::csv::{CementingData, CementingRecord};
use tauri::Manager;

#[tauri::command]
pub fn debug_serialize() -> Result<CementingData, String> {
    use crate::domain::csv::value_objects::CementingUnits;
    use chrono::Utc;

    // Создаем одну тестовую запись
    let record = CementingRecord {
        recirc_density: 1.23,
        ps_pressure: 45.6,
        // ... остальные поля заполни нулями или дефолтными значениями
        ..CementingRecord::zero() // если у тебя есть такой метод, или вручную
    };

    // Собираем агрегат
    let data = CementingData::new(
        "/fake/path.csv".to_string(),
        "debug.csv".to_string(),
        Utc::now(),
        CementingUnits::default_units(), // или как там у тебя создается
        vec![record],
    );

    Ok(data)
}

// === НОВАЯ КОМАНДА: получить данные по session_id ===

#[tauri::command]
pub async fn get_csv_data(
    app_handle: tauri::AppHandle,
    session_id: String,
) -> Result<CementingData, String> {
    // 1. Парсим UUID из строки
    let id = uuid::Uuid::parse_str(&session_id)
        .map_err(|_| "Невалидный session_id формат".to_string())?;

    // 2. Достаем CsvManager из состояния приложения
    let manager = app_handle.state::<crate::services::csv_manager::CsvManager>();

    // 3. Пытаемся получить данные из кэша
    match manager.get_data_result(id) {
        Ok(data) => Ok(data), // Успех — отдаём данные фронту
        Err(e) => Err(format!("Не удалось получить данные: {}", e)),
    }
}
