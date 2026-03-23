use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::domain::csv::entities::CementingData;
use crate::domain::csv::services::csv_parser::CsvParser; // ← только парсер
use crate::domain::csv::{CsvError, Result}; // ← ошибки напрямую из домена!

/// Менеджер CSV-файлов (Application Service)
/// Управляет кэшем, загрузкой, доступом к данным
pub struct CsvManager {
    /// Кэш загруженных файлов по ID
    cache: Arc<RwLock<HashMap<Uuid, CementingData>>>,
}

impl CsvManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Загрузить CSV файл и вернуть ID сессии
    pub fn load_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<Uuid> {
        // 1. Парсим файл через доменный сервис
        let data = CsvParser::parse(path)?;

        // 2. Сохраняем в кэш по ID
        let id = data.id;
        {
            let mut cache = self.cache.write().map_err(|_| {
                CsvError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Cache lock poisoned",
                ))
            })?;
            cache.insert(id, data);
        }

        Ok(id)
    }

    /// Получить данные по ID (результат)
    pub fn get_data_result(&self, id: Uuid) -> Result<CementingData> {
        let cache = self.cache.read().map_err(|_| {
            CsvError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cache lock poisoned",
            ))
        })?;
        cache
            .get(&id)
            .cloned()
            .ok_or_else(|| CsvError::FileNotFound(format!("Файл с ID {} не найден в кэше", id)))
    }

    /// Удалить файл из кэша
    pub fn unload_file(&self, id: Uuid) -> bool {
        let mut cache = match self.cache.write() {
            Ok(guard) => guard,
            Err(_) => return false,
        };
        cache.remove(&id).is_some()
    }

    /// Очистить весь кэш
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Получить список загруженных файлов
    pub fn list_files(&self) -> Vec<CementingData> {
        let cache = match self.cache.read() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };
        cache.values().cloned().collect()
    }
}

impl Default for CsvManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod poc_debug_tests {
    use super::*;
    use std::path::Path;

    /// 🔧 POC: Быстрая проверка, что CSV парсится и данные валидны
    /// Просто подставь путь ниже и запускай с --nocapture
    #[test]
    fn test_poc_show_me_the_data() {
        // ================================
        // 👇 ПУТЬ СЮДА 👇
        // ================================
        let HARDCODED_PATH = r#"path"#;
        // ================================

        println!("\n🔍 [POC] Проверяем файл: {}", HARDCODED_PATH);

        // Guard: проверяем, что файл вообще есть
        assert!(
            Path::new(HARDCODED_PATH).exists(),
            "❌ Файл не найден: {}. Положи его туда или поправь путь.",
            HARDCODED_PATH
        );

        let manager = CsvManager::new();

        // 1. Грузим
        let id = match manager.load_file(HARDCODED_PATH) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("❌ Ошибка парсинга: {:?}", e);
                eprintln!("💡 Тип ошибки: {}", std::any::type_name_of_val(&e));
                panic!("Тест остановлен — смотри вывод выше");
            }
        };
        println!("✅ Файл загружен. Сессия: {}", id);

        // 2. Достаем из кэша
        let data = manager
            .get_data_result(id)
            .expect("❌ Данные не найдены в кэше");

        // 3. Базовая валидация
        assert!(data.has_data(), "❌ Данные пусты");
        println!("✅ Валидация пройдена. Записей: {}", data.records.len());

        // ================================
        // 📊 ВЫВОД ДЛЯ ОТЛАДКИ
        // ================================

        println!("\n📦 Метаданные:");
        println!("   • Файл: {}", data.file_name);
        println!(
            "   • Период: {} → {}",
            data.start_time.format("%H:%M:%S"),
            data.end_time.format("%H:%M:%S")
        );
        println!("   • Длительность: {}", data.duration_human());

        println!("\n📋 Превью данных (первые 3 записи):");
        for (idx, rec) in data.records.iter().take(3).enumerate() {
            println!("   ┌─ Запись #{} (t+{}с):", idx, idx);
            // 👇 Теперь всё по факту, без гадания 👇
            println!(
                "   │  • recirc_density:    {:.2} {}",
                rec.recirc_density, data.units.recirc_density
            );
            println!(
                "   │  • ps_pressure:       {:.2} {}",
                rec.ps_pressure, data.units.ps_pressure
            );
            println!(
                "   │  • ds_pressure:       {:.2} {}",
                rec.ds_pressure, data.units.ds_pressure
            );
            println!(
                "   │  • cement_vlv:        {} {}",
                rec.cement_vlv_percent, data.units.cement_vlv_percent
            );
            println!(
                "   │  • wtr_vlv:           {} {}",
                rec.wtr_vlv_percent, data.units.wtr_vlv_percent
            );
            println!(
                "   │  • mix_water_rate:    {:.2} {}",
                rec.mix_water_rate, data.units.mix_water_rate
            );
            println!("   └─");
        }

        // Бонус: статистика
        if let Some(stats) = data.get_field_stats(|r| r.recirc_density) {
            println!("\n📈 Статистика (recirc_density):");
            println!(
                "   • Min: {:.3}, Max: {:.3}, Avg: {:.3} {}",
                stats.min, stats.max, stats.avg, data.units.recirc_density
            );
        }

        println!("\n🎉 POC SUCCESS. Данные на месте.\n");
    }
}
