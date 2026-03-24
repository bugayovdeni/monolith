use serde::{Deserialize, Serialize};

/// Одна запись данных цементирования (44 байта в памяти)
/// Поля отсортированы по размеру для минимального padding

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CementingRecord {
    // === f32 (4 байта) - 8 полей ===
    #[serde(rename = "Recirc Density")]
    pub recirc_density: f32,

    #[serde(rename = "Downhole Density")]
    pub downhole_density: f32,

    #[serde(rename = "Mix Water Rate")]
    pub mix_water_rate: f32,

    #[serde(rename = "Combo Rate")]
    pub combo_rate: f32,

    #[serde(rename = "PS Pressure")]
    pub ps_pressure: f32,

    #[serde(rename = "DS Pressure")]
    pub ds_pressure: f32,

    #[serde(rename = "Mix Wtr Stg Ttl")]
    pub mix_wtr_stg_ttl: f32,

    #[serde(rename = "Mix Wtr Job Ttl")]
    pub mix_wtr_job_ttl: f32,

    #[serde(rename = "Combo Pump Stg Ttl")]
    pub combo_pump_stg_ttl: f32,

    #[serde(rename = "Combo Pump Job Ttl")]
    pub combo_pump_job_ttl: f32,

    #[serde(rename = "PS Rate")]
    pub ps_rate: f32,

    #[serde(rename = "DS Rate")]
    pub ds_rate: f32,

    // === u8 (1 байт) - 4 поля (0-100) ===
    /// Cement Vlv Percent - cmt%
    #[serde(rename = "Cement Vlv Percent")]
    pub cement_vlv_percent: u8,

    #[serde(rename = "Wtr Vlv Percent")]
    pub wtr_vlv_percent: u8,

    #[serde(rename = "Digital Outs")]
    pub digital_outs: u8,

    #[serde(rename = "Event Num")]
    pub event_num: u8,
}

impl CementingRecord {
    /// Создать запись со значениями по умолчанию (все нули)
    pub fn zero() -> Self {
        Self {
            recirc_density: 0.00,
            downhole_density: 0.00,
            mix_water_rate: 0.0,
            combo_rate: 0.0,
            ps_pressure: 0.0,
            ds_pressure: 0.0,
            mix_wtr_stg_ttl: 0.0,
            mix_wtr_job_ttl: 0.0,
            combo_pump_stg_ttl: 0.0,
            combo_pump_job_ttl: 0.0,
            ps_rate: 0.0,
            ds_rate: 0.0,
            cement_vlv_percent: 0,
            wtr_vlv_percent: 0,
            digital_outs: 0,
            event_num: 0,
        }
    }

    /// Проверить, что запись не пустая (хотя бы одно поле > 0)
    pub fn is_non_zero(&self) -> bool {
        self.recirc_density > 0.00 || self.ps_pressure != 0.0 || self.cement_vlv_percent > 0
    }
}

/// Проверка размера структуры (тест в рантайме)
#[test]
fn test_record_size() {
    assert_eq!(std::mem::size_of::<CementingRecord>(), 52);
}
