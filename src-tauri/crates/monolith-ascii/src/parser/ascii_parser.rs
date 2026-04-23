use monolith_domain::CementingRecord;

use crate::error::ascii_error::AsciiError;

pub fn parse_line(line: &str) -> Result<CementingRecord, AsciiError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();

    if fields.len() < 16 {
        return Err(AsciiError::NotEnoughFields(fields.len()));
    }

    Ok(CementingRecord {
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
    })
}

fn parse_f32_with_unit(
    raw: &str,
    field: &'static str,
    expected_unit: &'static str,
) -> Result<f32, AsciiError> {
    let (value_part, unit_part) = split_value_and_unit(raw);
    validate_unit(field, expected_unit, unit_part)?;

    value_part
        .parse::<f32>()
        .map_err(|_| AsciiError::ParseFloat {
            field,
            value: raw.to_string(),
        })
}

fn parse_u8_with_unit(
    raw: &str,
    field: &'static str,
    expected_unit: &'static str,
) -> Result<u8, AsciiError> {
    let (value_part, unit_part) = split_value_and_unit(raw);
    validate_unit(field, expected_unit, unit_part)?;

    parse_u8_value(value_part, field, raw)
}

fn parse_u8_without_unit(raw: &str, field: &'static str) -> Result<u8, AsciiError> {
    let (value_part, unit_part) = split_value_and_unit(raw);

    if !unit_part.is_empty() {
        return Err(AsciiError::InvalidUnit {
            field,
            expected: "",
            actual: unit_part.to_string(),
        });
    }

    parse_u8_value(value_part, field, raw)
}

fn parse_u8_value(value_part: &str, field: &'static str, raw: &str) -> Result<u8, AsciiError> {
    if let Ok(value) = value_part.parse::<u8>() {
        return Ok(value);
    }

    let float_value = value_part.parse::<f32>().map_err(|_| AsciiError::ParseU8 {
        field,
        value: raw.to_string(),
    })?;

    Ok(float_value as u8)
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
    if actual == expected {
        Ok(())
    } else {
        Err(AsciiError::InvalidUnit {
            field,
            expected,
            actual: actual.to_string(),
        })
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
