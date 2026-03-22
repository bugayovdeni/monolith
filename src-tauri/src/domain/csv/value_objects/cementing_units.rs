use serde::{Deserialize, Serialize};

/// Единицы измерения для 16 полей (из второй строки CSV)
/// Порядок полей должен строго соответствовать CementingRecord
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CementingUnits {
    // === f32 поля ===
    /// Recirc Density
    pub recirc_density: String, // "ppg"

    /// Downhole Density
    pub downhole_density: String, // "ppg"

    /// Mix Water Rate
    pub mix_water_rate: String, // "gpm"

    /// Combo Rate
    pub combo_rate: String, // "bpm"

    /// PS Pressure
    pub ps_pressure: String, // "psi"

    /// DS Pressure
    pub ds_pressure: String, // "psi"

    /// Mix Wtr Stg Ttl
    pub mix_wtr_stg_ttl: String, // "gal"

    /// Mix Wtr Job Ttl
    pub mix_wtr_job_ttl: String, // "gal"

    /// Combo Pump Stg Ttl
    pub combo_pump_stg_ttl: String, // "bbl"

    /// Combo Pump Job Ttl
    pub combo_pump_job_ttl: String, // "bbl"

    /// PS Rate
    pub ps_rate: String, // "bpm"

    /// DS Rate
    pub ds_rate: String, // "bpm"

    // === u8 поля ===
    /// Cement Vlv Percent
    pub cement_vlv_percent: String, // "cmt%"

    /// Wtr Vlv Percent
    pub wtr_vlv_percent: String, // "wtr%"

    /// Digital Outs
    pub digital_outs: String, // "" или "state"

    /// Event Num
    pub event_num: String, // "" или "code"
}

impl CementingUnits {
    /// Дефолтные единицы (если в CSV нет второй строки или ошибка парсинга)
    pub fn default_units() -> Self {
        Self {
            recirc_density: "ppg".to_string(),
            downhole_density: "ppg".to_string(),
            mix_water_rate: "gpm".to_string(),
            combo_rate: "bpm".to_string(),
            ps_pressure: "psi".to_string(),
            ds_pressure: "psi".to_string(),
            mix_wtr_stg_ttl: "gal".to_string(),
            mix_wtr_job_ttl: "gal".to_string(),
            combo_pump_stg_ttl: "bbl".to_string(),
            combo_pump_job_ttl: "bbl".to_string(),
            ps_rate: "bpm".to_string(),
            ds_rate: "bpm".to_string(),
            cement_vlv_percent: "cmt%".to_string(),
            wtr_vlv_percent: "wtr%".to_string(),
            digital_outs: String::new(), // Без единиц (состояние)
            event_num: String::new(),    // Без единиц (номер события)
        }
    }

    /// Получить единицу измерения по имени поля
    pub fn get_by_field(&self, field_name: &str) -> Option<&str> {
        match field_name {
            "recirc_density" => Some(&self.recirc_density),
            "downhole_density" => Some(&self.downhole_density),
            "mix_water_rate" => Some(&self.mix_water_rate),
            "combo_rate" => Some(&self.combo_rate),
            "ps_pressure" => Some(&self.ps_pressure),
            "ds_pressure" => Some(&self.ds_pressure),
            "mix_wtr_stg_ttl" => Some(&self.mix_wtr_stg_ttl),
            "mix_wtr_job_ttl" => Some(&self.mix_wtr_job_ttl),
            "combo_pump_stg_ttl" => Some(&self.combo_pump_stg_ttl),
            "combo_pump_job_ttl" => Some(&self.combo_pump_job_ttl),
            "ps_rate" => Some(&self.ps_rate),
            "ds_rate" => Some(&self.ds_rate),
            "cement_vlv_percent" => Some(&self.cement_vlv_percent),
            "wtr_vlv_percent" => Some(&self.wtr_vlv_percent),
            "digital_outs" => Some(&self.digital_outs),
            "event_num" => Some(&self.event_num),
            _ => None,
        }
    }

    /// Получить единицу измерения по индексу поля (0-15)
    pub fn get_by_index(&self, index: usize) -> Option<&str> {
        match index {
            0 => Some(&self.recirc_density),
            1 => Some(&self.downhole_density),
            2 => Some(&self.mix_water_rate),
            3 => Some(&self.combo_rate),
            4 => Some(&self.ps_pressure),
            5 => Some(&self.ds_pressure),
            6 => Some(&self.mix_wtr_stg_ttl),
            7 => Some(&self.mix_wtr_job_ttl),
            8 => Some(&self.combo_pump_stg_ttl),
            9 => Some(&self.combo_pump_job_ttl),
            10 => Some(&self.ps_rate),
            11 => Some(&self.ds_rate),
            12 => Some(&self.cement_vlv_percent),
            13 => Some(&self.wtr_vlv_percent),
            14 => Some(&self.digital_outs),
            15 => Some(&self.event_num),
            _ => None,
        }
    }

    /// Проверить, что все единицы заполнены (не пустые)
    pub fn is_complete(&self) -> bool {
        !self.recirc_density.is_empty()
            && !self.downhole_density.is_empty()
            && !self.mix_water_rate.is_empty()
            && !self.combo_rate.is_empty()
            && !self.ps_pressure.is_empty()
            && !self.ds_pressure.is_empty()
            && !self.mix_wtr_stg_ttl.is_empty()
            && !self.mix_wtr_job_ttl.is_empty()
            && !self.combo_pump_stg_ttl.is_empty()
            && !self.combo_pump_job_ttl.is_empty()
            && !self.ps_rate.is_empty()
            && !self.ds_rate.is_empty()
            && !self.cement_vlv_percent.is_empty()
            && !self.wtr_vlv_percent.is_empty()
        // digital_outs и event_num могут быть пустыми
    }
}

/// Тест на создание дефолтных единиц
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_units() {
        let units = CementingUnits::default_units();
        assert_eq!(units.recirc_density, "ppg");
        assert_eq!(units.ps_pressure, "psi");
        assert_eq!(units.cement_vlv_percent, "cmt%");
        assert!(units.digital_outs.is_empty());
        assert!(units.event_num.is_empty());
    }

    #[test]
    fn test_get_by_index() {
        let units = CementingUnits::default_units();
        assert_eq!(units.get_by_index(0), Some("ppg"));
        assert_eq!(units.get_by_index(4), Some("psi"));
        assert_eq!(units.get_by_index(12), Some("cmt%"));
        assert_eq!(units.get_by_index(99), None);
    }

    #[test]
    fn test_get_by_field() {
        let units = CementingUnits::default_units();
        assert_eq!(units.get_by_field("recirc_density"), Some("ppg"));
        assert_eq!(units.get_by_field("ps_pressure"), Some("psi"));
        assert_eq!(units.get_by_field("unknown_field"), None);
    }
}
