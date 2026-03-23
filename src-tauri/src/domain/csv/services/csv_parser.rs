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
        // ✅ ВСТАВИТЬ ЭТО (новый код с парсингом по индексам):
        // Вспомогательные функции для парсинга
        let parse_f32 =
            |row: usize, rec: &csv::StringRecord, idx: usize, name: &str| -> Result<f32> {
                rec.get(idx)
                    .ok_or_else(|| CsvError::ParseError {
                        row,
                        field: name.into(),
                        reason: "missing column".into(),
                    })?
                    .trim()
                    .parse::<f32>()
                    .map_err(|e| CsvError::ParseError {
                        row,
                        field: name.into(),
                        reason: e.to_string(),
                    })
            };

        // ✅ УЛУЧШЕННАЯ функция для u8: умеет парсить и "45", и "45.00"
        let parse_u8 =
            |row: usize, rec: &csv::StringRecord, idx: usize, name: &str| -> Result<u8> {
                let val = rec.get(idx).map(|s| s.trim()).unwrap_or("0");
                if val.is_empty() {
                    Ok(0)
                } else {
                    // Сначала пробуем как u8, если не вышло (есть точка) — парсим как f32 и кастуем
                    val.parse::<u8>()
                        .or_else(|_| val.parse::<f32>().map(|f| f as u8))
                        .map_err(|e| CsvError::ParseError {
                            row,
                            field: name.into(),
                            reason: e.to_string(),
                        })
                }
            };

        // Парсим данные по индексам (игнорируем заголовки)
        let records: Vec<CementingRecord> = reader
            .records()
            .enumerate()
            .map(|(i, result)| {
                let rec = result.map_err(|e| CsvError::CsvError(e))?;
                let row_num = i + 3; // 1=headers, 2=units, 3+=data

                Ok(CementingRecord {
                    // === f32 поля (индексы 0-9) ===
                    recirc_density: parse_f32(row_num, &rec, 0, "recirc_density")?,
                    downhole_density: parse_f32(row_num, &rec, 1, "downhole_density")?,
                    mix_water_rate: parse_f32(row_num, &rec, 2, "mix_water_rate")?,
                    combo_rate: parse_f32(row_num, &rec, 3, "combo_rate")?,
                    ps_pressure: parse_f32(row_num, &rec, 4, "ps_pressure")?,
                    ds_pressure: parse_f32(row_num, &rec, 5, "ds_pressure")?,
                    mix_wtr_stg_ttl: parse_f32(row_num, &rec, 6, "mix_wtr_stg_ttl")?,
                    mix_wtr_job_ttl: parse_f32(row_num, &rec, 7, "mix_wtr_job_ttl")?,
                    combo_pump_stg_ttl: parse_f32(row_num, &rec, 8, "combo_pump_stg_ttl")?,
                    combo_pump_job_ttl: parse_f32(row_num, &rec, 9, "combo_pump_job_ttl")?,

                    // === u8 поля: проценты (индексы 10-11 в файле) ===
                    cement_vlv_percent: parse_u8(row_num, &rec, 10, "cement_vlv_percent")?,
                    wtr_vlv_percent: parse_u8(row_num, &rec, 11, "wtr_vlv_percent")?,

                    // === f32 поля: рейты (индексы 12-13 в файле) ===
                    ps_rate: parse_f32(row_num, &rec, 12, "ps_rate")?,
                    ds_rate: parse_f32(row_num, &rec, 13, "ds_rate")?,

                    // === u8 поля: флаги (индексы 14-15 в файле, но там могут быть "0.00") ===
                    digital_outs: parse_u8(row_num, &rec, 14, "digital_outs")?,
                    event_num: parse_u8(row_num, &rec, 15, "event_num")?,
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
        if !name.starts_with("Data") || name.len() != 17 {
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
        let units_row = reader
            .records()
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
            cement_vlv_percent: get_unit(10),
            wtr_vlv_percent: get_unit(11),
            ps_rate: get_unit(12),
            ds_rate: get_unit(13),
            digital_outs: get_unit(14),
            event_num: get_unit(15),
        })
    }
}
