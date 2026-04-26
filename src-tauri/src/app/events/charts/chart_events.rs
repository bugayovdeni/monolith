use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AxisModePayload {
    pub mode: AxisMode,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AxisMode {
    AutoExpand,
}

#[derive(Serialize, Clone)]
pub struct AxisSettingsOpenPayload {
    pub action: &'static str,
}
