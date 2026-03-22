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
