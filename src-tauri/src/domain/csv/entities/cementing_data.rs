use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::cementing_record::CementingRecord;
use crate::domain::csv::value_objects::{
    cementing_units::CementingUnits, csv_metadata::CsvMetadata,
};

/// Агрегат: все данные CSV файла + время для графиков
/// Это главная структура, которая летает между слоями и улетает на фронт
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CementingData {
    // === Идентификация ===
    /// Уникальный ID сессии (для кэша, истории, отслеживания)
    pub id: Uuid,

    /// Полный путь к файлу в ФС
    pub file_path: String,

    /// Только имя файла (для отображения в UI)
    pub file_name: String,

    // === Время (ось X для графиков) ===
    /// Время начала записи (из имени файла, например Data20260123_1339)
    pub start_time: DateTime<Utc>,

    /// Время окончания (вычисляется: start + (rows-1) * interval)
    pub end_time: DateTime<Utc>,

    /// Интервал между записями в миллисекундах (фиксировано 1000 мс = 1 сек)
    pub sample_interval_ms: u32,

    // === Единицы измерения (вторая строка CSV) ===
    pub units: CementingUnits,

    // === Основные данные (16 полей × N строк) ===
    pub records: Vec<CementingRecord>,

    // === Метаданные файла ===
    pub meta: CsvMetadata,
}

impl CementingData {
    /// Создать новый агрегат
    ///
    /// # Arguments
    /// * `file_path` - полный путь к файлу
    /// * `file_name` - имя файла (без пути)
    /// * `start_time` - время начала записи (из имени файла)
    /// * `units` - единицы измерения из второй строки CSV
    /// * `records` - распарсенные записи данных
    pub fn new(
        file_path: String,
        file_name: String,
        start_time: DateTime<Utc>,
        units: CementingUnits,
        records: Vec<CementingRecord>,
    ) -> Self {
        let total_rows = records.len();
        let file_size_bytes = 0; // Заполнится позже из метаданных файла

        // Создаём метаданные (52 байта на запись - размер CementingRecord)
        let meta = CsvMetadata::new(
            file_path.clone(),
            file_name.clone(),
            total_rows,
            file_size_bytes,
            52, // std::mem::size_of::<CementingRecord>()
        );

        // Расчёт end_time
        let end_time = if total_rows > 0 {
            start_time + Duration::milliseconds((total_rows - 1) as i64 * 1000)
        } else {
            start_time
        };

        Self {
            id: Uuid::new_v4(),
            file_path,
            file_name,
            start_time,
            end_time,
            sample_interval_ms: 1000, // 1 секунда фиксировано
            units,
            records,
            meta,
        }
    }

    /// Получить время для записи по индексу
    #[inline]
    pub fn timestamp_at(&self, index: usize) -> DateTime<Utc> {
        let offset_ms = (index as i64) * (self.sample_interval_ms as i64);
        self.start_time + Duration::milliseconds(offset_ms)
    }

    /// Временной диапазон для графика (start, end)
    pub fn time_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        (self.start_time, self.end_time)
    }

    /// Длительность записи в секундах
    pub fn duration_secs(&self) -> i64 {
        (self.end_time - self.start_time).num_seconds()
    }

    /// Длительность записи в человекочитаемом формате (например "1ч 23м")
    pub fn duration_human(&self) -> String {
        let total_secs = self.duration_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            format!("{}ч {}м {}с", hours, mins, secs)
        } else if mins > 0 {
            format!("{}м {}с", mins, secs)
        } else {
            format!("{}с", secs)
        }
    }

    /// Конвертировать одну запись в точку для графика
    pub fn to_chart_point(
        &self,
        index: usize,
        field: impl Fn(&CementingRecord) -> f32,
    ) -> Option<ChartPoint> {
        self.records.get(index).map(|record| {
            let timestamp = self.timestamp_at(index);
            ChartPoint {
                x_ms: timestamp.timestamp_millis(),
                y: field(record),
                time_label: timestamp.format("%H:%M:%S").to_string(),
            }
        })
    }

    /// Получить все точки для графика по полю
    pub fn get_chart_data(&self, field: impl Fn(&CementingRecord) -> f32) -> Vec<ChartPoint> {
        (0..self.records.len())
            .filter_map(|i| self.to_chart_point(i, &field))
            .collect()
    }

    /// Получить статистику по полю (min, max, avg)
    pub fn get_field_stats(&self, field: impl Fn(&CementingRecord) -> f32) -> Option<FieldStats> {
        if self.records.is_empty() {
            return None;
        }

        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut sum = 0.0;

        for record in &self.records {
            let value = field(record);
            min = min.min(value);
            max = max.max(value);
            sum += value;
        }

        let avg = sum / self.records.len() as f32;

        Some(FieldStats { min, max, avg })
    }

    /// Обновить размер файла в метаданных (после чтения с диска)
    pub fn set_file_size(&mut self, size_bytes: u64) {
        self.meta.file_size_bytes = size_bytes;
    }

    /// Проверка: есть ли данные
    pub fn has_data(&self) -> bool {
        !self.records.is_empty()
    }

    /// Получить первую запись (если есть)
    pub fn first_record(&self) -> Option<&CementingRecord> {
        self.records.first()
    }

    /// Получить последнюю запись (если есть)
    pub fn last_record(&self) -> Option<&CementingRecord> {
        self.records.last()
    }
}

// ==================== DTO ДЛЯ ФРОНТА ====================

/// Точка данных для графика (отправка на фронт через Tauri)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChartPoint {
    /// Unix timestamp в миллисекундах (для JS Date)
    pub x_ms: i64,

    /// Значение метрики
    pub y: f32,

    /// Человекочитаемое время для тултипа (например "13:39:05")
    pub time_label: String,
}

/// Статистика по полю (min, max, avg)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FieldStats {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

// ==================== ТЕСТЫ ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_units() -> CementingUnits {
        CementingUnits::default_units()
    }

    fn sample_records() -> Vec<CementingRecord> {
        vec![
            CementingRecord {
                recirc_density: 8.10,
                downhole_density: 0.0,
                mix_water_rate: 0.0,
                combo_rate: 0.0,
                ps_pressure: 21.0,
                ds_pressure: -36.0,
                mix_wtr_stg_ttl: 1201.89,
                mix_wtr_job_ttl: 1201.89,
                combo_pump_stg_ttl: 0.0,
                combo_pump_job_ttl: 0.0,
                ps_rate: 0.0,
                ds_rate: 0.0,
                cement_vlv_percent: 37,
                wtr_vlv_percent: 90,
                digital_outs: 0,
                event_num: 0,
            },
            CementingRecord {
                recirc_density: 8.11,
                downhole_density: 0.0,
                mix_water_rate: 0.0,
                combo_rate: 0.0,
                ps_pressure: 21.0,
                ds_pressure: -34.0,
                mix_wtr_stg_ttl: 1201.89,
                mix_wtr_job_ttl: 1201.89,
                combo_pump_stg_ttl: 0.0,
                combo_pump_job_ttl: 0.0,
                ps_rate: 0.0,
                ds_rate: 0.0,
                cement_vlv_percent: 37,
                wtr_vlv_percent: 90,
                digital_outs: 0,
                event_num: 0,
            },
            CementingRecord {
                recirc_density: 8.07,
                downhole_density: 0.0,
                mix_water_rate: 0.0,
                combo_rate: 0.0,
                ps_pressure: 20.0,
                ds_pressure: -34.0,
                mix_wtr_stg_ttl: 1201.89,
                mix_wtr_job_ttl: 1201.89,
                combo_pump_stg_ttl: 0.0,
                combo_pump_job_ttl: 0.0,
                ps_rate: 0.0,
                ds_rate: 0.0,
                cement_vlv_percent: 37,
                wtr_vlv_percent: 90,
                digital_outs: 0,
                event_num: 0,
            },
        ]
    }

    fn sample_data() -> CementingData {
        let start_time = chrono::NaiveDateTime::parse_from_str("20260123 133900", "%Y%m%d %H%M%S")
            .unwrap()
            .and_utc();

        CementingData::new(
            "/path/to/Data20260123_1339.csv".to_string(),
            "Data20260123_1339.csv".to_string(),
            start_time,
            sample_units(),
            sample_records(),
        )
    }

    #[test]
    fn test_new_data() {
        let data = sample_data();

        assert_eq!(data.file_name, "Data20260123_1339.csv");
        assert_eq!(data.records.len(), 3);
        assert_eq!(data.sample_interval_ms, 1000);
        assert!(data.has_data());
    }

    #[test]
    fn test_timestamp_at() {
        let data = sample_data();

        // Первая запись: 13:39:00
        let t0 = data.timestamp_at(0);
        assert_eq!(t0.format("%H:%M:%S").to_string(), "13:39:00");

        // Вторая запись: 13:39:01
        let t1 = data.timestamp_at(1);
        assert_eq!(t1.format("%H:%M:%S").to_string(), "13:39:01");

        // Третья запись: 13:39:02
        let t2 = data.timestamp_at(2);
        assert_eq!(t2.format("%H:%M:%S").to_string(), "13:39:02");
    }

    #[test]
    fn test_time_range() {
        let data = sample_data();
        let (start, end) = data.time_range();

        assert_eq!(start.format("%H:%M:%S").to_string(), "13:39:00");
        assert_eq!(end.format("%H:%M:%S").to_string(), "13:39:02");
    }

    #[test]
    fn test_duration() {
        let data = sample_data();

        assert_eq!(data.duration_secs(), 2); // 3 записи = 2 секунды интервала
        assert_eq!(data.duration_human(), "2с");
    }

    #[test]
    fn test_chart_point() {
        let data = sample_data();

        let point = data.to_chart_point(0, |r| r.recirc_density);
        assert!(point.is_some());

        let point = point.unwrap();
        assert_eq!(point.y, 8.10);
        assert_eq!(point.time_label, "13:39:00");
    }

    #[test]
    fn test_chart_data() {
        let data = sample_data();

        let chart: Vec<ChartPoint> = data.get_chart_data(|r| r.recirc_density);
        assert_eq!(chart.len(), 3);

        assert_eq!(chart[0].y, 8.10);
        assert_eq!(chart[1].y, 8.11);
        assert_eq!(chart[2].y, 8.07);
    }

    #[test]
    fn test_field_stats() {
        let data = sample_data();

        let stats = data.get_field_stats(|r| r.recirc_density);
        assert!(stats.is_some());

        let stats = stats.unwrap();
        assert_eq!(stats.min, 8.07);
        assert_eq!(stats.max, 8.11);
        assert!((8.09..8.10).contains(&stats.avg)); // ~8.093
    }

    #[test]
    fn test_first_last_record() {
        let data = sample_data();

        let first = data.first_record();
        assert!(first.is_some());
        assert_eq!(first.unwrap().recirc_density, 8.10);

        let last = data.last_record();
        assert!(last.is_some());
        assert_eq!(last.unwrap().recirc_density, 8.07);
    }

    #[test]
    fn test_empty_data() {
        let start_time = Utc::now();
        let data = CementingData::new(
            "/empty.csv".to_string(),
            "empty.csv".to_string(),
            start_time,
            sample_units(),
            vec![],
        );

        assert!(!data.has_data());
        assert_eq!(data.records.len(), 0);
        assert_eq!(data.duration_secs(), 0);
        assert_eq!(data.first_record(), None);
    }
}
