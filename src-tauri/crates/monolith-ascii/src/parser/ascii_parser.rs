use monolith_domain::CementingRecord;

use crate::error::ascii_error::AsciiError;

pub fn parse_line(line: &str) -> Result<CementingRecord, AsciiError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();

    // Отладочный вывод для диагностики проблем с парсингом
    println!("[ASCII PARSER] raw line: {}", line);
    println!("[ASCII PARSER] fields count: {}", fields.len());

    if fields.len() < 16 {
        let err = AsciiError::NotEnoughFields(fields.len());
        println!("[ASCII PARSER] error: {:?}", err);
        return Err(err);
    }

    let result = CementingRecord {
        recirc_density: parse_f32_with_unit(fields[0], "recirc_density", "ppg")?,
        downhole_density: parse_f32_with_unit(fields[1], "downhole_density", "ppg")?,
        mix_water_rate: parse_f32_with_unit(fields[2], "mix_water_rate", "gpm")?,
        combo_rate: parse_f32_with_unit(fields[3], "combo_rate", "bpm")?,
        ps_pressure: parse_f32_with_unit(fields[4], "ps_pressure", "psi")?,
        ds_pressure: parse_f32_with_unit(fields[5], "ds_pressure", "psi")?,
        mix_wtr_stg_ttl: parse_f32_with_unit(fields[6], "mix_wtr_stg_ttl", "gal")?,
        mix_wtr_job_ttl: parse_f32_with_unit(fields[7], "mix_wtr_job_ttl", "gal")?,
        combo_pump_stg_ttl: parse_f32_with_unit(fields[8], "combo_pump_stg_ttl", "bbl")?,
        combo_pump_job_ttl: parse_f32_with_unit(fields[9], "combo_pump_job_ttl", "bbl")?,
        cement_vlv_percent: parse_u8_with_unit(fields[10], "cement_vlv_percent", "cmt%")?,
        wtr_vlv_percent: parse_u8_with_unit(fields[11], "wtr_vlv_percent", "wtr%")?,
        ps_rate: parse_f32_with_unit(fields[12], "ps_rate", "bpm")?,
        ds_rate: parse_f32_with_unit(fields[13], "ds_rate", "bpm")?,
        digital_outs: parse_u8_with_unit(fields[14], "digital_outs", "DigOut")?,
        event_num: parse_u8_without_unit(fields[15], "event_num")?,
    };

    println!("[ASCII PARSER] parsed record: {:?}", result);
    Ok(result)
}

fn parse_f32_with_unit(
    raw: &str,
    field: &'static str,
    expected_unit: &'static str,
) -> Result<f32, AsciiError> {
    // Отладочный вывод для отслеживания парсинга чисел с плавающей точкой
    println!("[ASCII PARSER] parse_f32_with_unit: raw='{}', field='{}', expected_unit='{}'", raw, field, expected_unit);
    
    let (value_part, unit_part) = split_value_and_unit(raw);
    println!("[ASCII PARSER]   split -> value='{}', unit='{}'", value_part, unit_part);
    
    // Валидация единицы измерения с выводом ошибки при несоответствии
    if let Err(e) = validate_unit(field, expected_unit, unit_part) {
        println!("[ASCII PARSER]   unit validation error: {:?}", e);
        return Err(e);
    }

    // Парсинг числового значения
    match value_part.parse::<f32>() {
        Ok(value) => {
            println!("[ASCII PARSER]   parsed f32 value: {}", value);
            Ok(value)
        }
        Err(_) => {
            let err = AsciiError::ParseFloat {
                field,
                value: raw.to_string(),
            };
            println!("[ASCII PARSER]   parse error: {:?}", err);
            Err(err)
        }
    }
}

fn parse_u8_with_unit(
    raw: &str,
    field: &'static str,
    expected_unit: &'static str,
) -> Result<u8, AsciiError> {
    // Отладочный вывод для отслеживания парсинга целых чисел с единицами измерения
    println!("[ASCII PARSER] parse_u8_with_unit: raw='{}', field='{}', expected_unit='{}'", raw, field, expected_unit);
    
    let (value_part, unit_part) = split_value_and_unit(raw);
    println!("[ASCII PARSER]   split -> value='{}', unit='{}'", value_part, unit_part);
    
    // Валидация единицы измерения с выводом ошибки при несоответствии
    if let Err(e) = validate_unit(field, expected_unit, unit_part) {
        println!("[ASCII PARSER]   unit validation error: {:?}", e);
        return Err(e);
    }

    // Парсинг числового значения
    parse_u8_value(value_part, field, raw)
}

fn parse_u8_without_unit(raw: &str, field: &'static str) -> Result<u8, AsciiError> {
    // Отладочный вывод для отслеживания парсинга целых чисел без единиц измерения
    println!("[ASCII PARSER] parse_u8_without_unit: raw='{}', field='{}'", raw, field);
    
    let (value_part, unit_part) = split_value_and_unit(raw);
    println!("[ASCII PARSER]   split -> value='{}', unit='{}'", value_part, unit_part);

    // Проверка, что единица измерения отсутствует
    if !unit_part.is_empty() {
        let err = AsciiError::InvalidUnit {
            field,
            expected: "",
            actual: unit_part.to_string(),
        };
        println!("[ASCII PARSER]   unit validation error: {:?}", err);
        return Err(err);
    }

    // Парсинг числового значения
    parse_u8_value(value_part, field, raw)
}

fn parse_u8_value(value_part: &str, field: &'static str, raw: &str) -> Result<u8, AsciiError> {
    // Отладочный вывод для парсинга целых чисел (u8)
    println!("[ASCII PARSER] parse_u8_value: value_part='{}', field='{}', raw='{}'", value_part, field, raw);
    
    // Сначала пробуем распарсить как u8
    if let Ok(value) = value_part.parse::<u8>() {
        println!("[ASCII PARSER]   parsed as u8: {}", value);
        return Ok(value);
    }

    // Если не удалось, пробуем как f32 и приводим к u8
    println!("[ASCII PARSER]   trying to parse as f32...");
    match value_part.parse::<f32>() {
        Ok(float_value) => {
            let result = float_value as u8;
            println!("[ASCII PARSER]   parsed as f32: {} -> u8: {}", float_value, result);
            Ok(result)
        }
        Err(_) => {
            let err = AsciiError::ParseU8 {
                field,
                value: raw.to_string(),
            };
            println!("[ASCII PARSER]   parse error: {:?}", err);
            Err(err)
        }
    }
}

fn split_value_and_unit(raw: &str) -> (&str, &str) {
    let mut parts = raw.split_whitespace();

    let value_part = parts.next().unwrap_or("");
    let unit_part = parts.next().unwrap_or("");

    (value_part, unit_part)
}

fn validate_unit(
    field: &'static str,
    expected: &'static str,
    actual: &str,
) -> Result<(), AsciiError> {
    // Отладочный вывод для валидации единиц измерения
    println!("[ASCII PARSER] validate_unit: field='{}', expected='{}', actual='{}'", field, expected, actual);
    
    if actual == expected {
        println!("[ASCII PARSER]   unit OK");
        Ok(())
    } else {
        let err = AsciiError::InvalidUnit {
            field,
            expected,
            actual: actual.to_string(),
        };
        println!("[ASCII PARSER]   unit mismatch: {:?}", err);
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    use crate::error::ascii_error::AsciiError;

    #[test]
    fn parse_line_returns_error_when_fields_are_less_than_16() {
        let line = "13.44 ppg,10.88 ppg,12 gpm";

        let result = parse_line(line);

        assert!(matches!(result, Err(AsciiError::NotEnoughFields(3))));
    }

    #[test]
    fn parse_line_parses_first_16_fields_into_cementing_record() {
        let line = "13.44 ppg,10.88 ppg,12 gpm,7.0 bpm,1359 psi,4413 psi,0 gal,0 gal,0.0 bbl,0.0 bbl,81.89 cmt%,82.55 wtr%,0.0 bpm,0.0 bpm,0 DigOut,0";

        let record = parse_line(line).unwrap();

        assert_eq!(record.recirc_density, 13.44);
        assert_eq!(record.downhole_density, 10.88);
        assert_eq!(record.mix_water_rate, 12.0);
        assert_eq!(record.combo_rate, 7.0);
        assert_eq!(record.ps_pressure, 1359.0);
        assert_eq!(record.ds_pressure, 4413.0);
        assert_eq!(record.mix_wtr_stg_ttl, 0.0);
        assert_eq!(record.mix_wtr_job_ttl, 0.0);
        assert_eq!(record.combo_pump_stg_ttl, 0.0);
        assert_eq!(record.combo_pump_job_ttl, 0.0);
        assert_eq!(record.cement_vlv_percent, 81);
        assert_eq!(record.wtr_vlv_percent, 82);
        assert_eq!(record.ps_rate, 0.0);
        assert_eq!(record.ds_rate, 0.0);
        assert_eq!(record.digital_outs, 0);
        assert_eq!(record.event_num, 0);
    }

    #[test]
    fn parse_line_returns_error_for_invalid_unit() {
        let line = "13.44 psi,10.88 ppg,12 gpm,7.0 bpm,1359 psi,4413 psi,0 gal,0 gal,0.0 bbl,0.0 bbl,81.89 cmt%,82.55 wtr%,0.0 bpm,0.0 bpm,0 DigOut,0";

        let result = parse_line(line);

        assert!(matches!(
            result,
            Err(AsciiError::InvalidUnit {
                field: "recirc_density",
                expected: "ppg",
                ..
            })
        ));
    }

    #[test]
    fn parse_line_returns_error_when_event_num_has_unit() {
        let line = "13.44 ppg,10.88 ppg,12 gpm,7.0 bpm,1359 psi,4413 psi,0 gal,0 gal,0.0 bbl,0.0 bbl,81.89 cmt%,82.55 wtr%,0.0 bpm,0.0 bpm,0 DigOut,0 event";

        let result = parse_line(line);

        assert!(matches!(
            result,
            Err(AsciiError::InvalidUnit {
                field: "event_num",
                expected: "",
                ..
            })
        ));
    }
}
