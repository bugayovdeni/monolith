use chrono::{DateTime, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// domain/csv/services/csv_parser.rs
use crate::domain::csv::{CementingData, CementingRecord, CementingUnits, CsvError, Result};

/// CSV-парсер для файлов цементирования
pub struct CsvParser;

impl CsvParser {
    /// Распарсить CSV файл и вернуть CementingData
    pub fn parse<P: AsRef<Path>>(file_path: P) -> Result<CementingData> {
        let path = file_path.as_ref();

        // 1. Валидация пути
        if !path.exists() {
            return Err(CsvError::FileNotFound(path.display().to_string()));
        }

        // 2. Извлекаем start_time из имени файла
        let file_name = path
            .file_name()
            .ok_or_else(|| CsvError::InvalidFilename(path.display().to_string()))?
            .to_str()
            .ok_or_else(|| CsvError::InvalidFilename(path.display().to_string()))?;

        let start_time = Self::extract_start_time(file_name)?;

        // 3. Открываем файл
        let file = File::open(path)?;
        let file_size = path.metadata()?.len();
        let buf_reader = BufReader::new(file);

        // 4. Создаём CSV reader с заголовками
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true) // Разрешить разное количество колонок
            .from_reader(buf_reader);

        // 5. Читаем заголовки (строка 1)
        let headers = reader.headers()?.clone();
        if headers.len() < 16 {
            return Err(CsvError::InvalidHeaders(headers.len()));
        }

        // 6. Читаем единицы измерения (строка 2)
        let units = Self::parse_units(&mut reader)?;

        // 7. Читаем данные (строки 3..N)
        let records: Vec<CementingRecord> = reader
            .deserialize()
            .enumerate()
            .map(|(i, result)| {
                result.map_err(|e| CsvError::ParseError {
                    row: i + 3, // +3 потому что 1-заголовки, 2-единицы, 3-первая данные
                    field: "unknown".to_string(),
                    reason: e.to_string(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        if records.is_empty() {
            return Err(CsvError::EmptyFile);
        }

        // 8. Собираем CementingData
        let mut data = CementingData::new(
            path.display().to_string(),
            file_name.to_string(),
            start_time,
            units,
            records,
        );

        // 9. Обновляем размер файла в метаданных
        data.set_file_size(file_size);

        Ok(data)
    }

    /// Извлечь start_time из имени файла (Data20260123_1339.csv)
    fn extract_start_time(filename: &str) -> Result<DateTime<Utc>> {
        // Убираем расширение
        let name = filename.trim_end_matches(".csv");

        // Ожидаем формат: DataYYYYMMDD_HHMM
        if !name.starts_with("Data") || name.len() != 16 {
            return Err(CsvError::InvalidFilename(filename.to_string()));
        }

        let date_part = &name[4..12]; // "20260123"
        let time_part = &name[13..17]; // "1339"

        // Собираем в строку для парсинга: "20260123 133900"
        let datetime_str = format!("{} {}00", date_part, time_part);

        let naive = NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%d %H%M%S")
            .map_err(|e| CsvError::InvalidDate(e.to_string()))?;

        Ok(naive.and_utc())
    }

    /// Парсинг второй строки CSV в CementingUnits
    fn parse_units(reader: &mut csv::Reader<BufReader<File>>) -> Result<CementingUnits> {
        let mut units_iter = reader.records();

        let units_row = units_iter
            .next()
            .ok_or(CsvError::EmptyFile)?
            .map_err(|e| CsvError::CsvError(e))?;

        // Берём первые 16 колонок (или меньше, если нет)
        let get_unit = |idx: usize| {
            units_row
                .get(idx)
                .map(|s| s.trim().to_string())
                .unwrap_or_default()
        };

        Ok(CementingUnits {
            recirc_density: get_unit(0),
            downhole_density: get_unit(1),
            mix_water_rate: get_unit(2),
            combo_rate: get_unit(3),
            ps_pressure: get_unit(4),
            ds_pressure: get_unit(5),
            mix_wtr_stg_ttl: get_unit(6),
            mix_wtr_job_ttl: get_unit(7),
            combo_pump_stg_ttl: get_unit(8),
            combo_pump_job_ttl: get_unit(9),
            ps_rate: get_unit(10),
            ds_rate: get_unit(11),
            cement_vlv_percent: get_unit(12),
            wtr_vlv_percent: get_unit(13),
            digital_outs: get_unit(14),
            event_num: get_unit(15),
        })
    }
}
