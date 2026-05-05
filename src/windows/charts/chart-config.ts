import { SeriesConfig } from "./chart-manager";
import { DEFAULT_SERIES_MAPPING } from "../../services/csv/csv-manager";

export const COLORS: Record<string, string> = {
  ps_pressure: "#e74c3c",
  ds_pressure: "#c0392b",
  recirc_density: "#3498db",
  downhole_density: "#2980b9",
  mix_water_rate: "#2ecc71",
  combo_rate: "#27ae60",
  ps_rate: "#1abc9c",
  ds_rate: "#16a085",
  mix_wtr_stg_ttl: "#9b59b6",
  mix_wtr_job_ttl: "#8e44ad",
  combo_pump_stg_ttl: "#34495e",
  combo_pump_job_ttl: "#2c3e50",
  cement_vlv_percent: "#e67e22",
  wtr_vlv_percent: "#d35400",
};

export const seriesColors: Record<string, string> = { ...COLORS };

export const SERIES_LABELS_RU: Record<string, string> = {
  recirc_density: "Цирк Плотн",
  downhole_density: "Забой Плотн",
  mix_water_rate: "Расход воды",
  combo_rate: "Общий расход",

  ps_pressure: "Давление ПС",
  ds_pressure: "Давление ДС",

  mix_wtr_stg_ttl: "Вода Стадия",
  mix_wtr_job_ttl: "Вода Работа",

  combo_pump_stg_ttl: "Прокачка Стадия",
  combo_pump_job_ttl: "Прокачка Всего",

  cement_vlv_percent: "Цемент %",
  wtr_vlv_percent: "Вода %",

  ps_rate: "Расход ПС",
  ds_rate: "Расход ДС",
};

export const chartSeries: SeriesConfig[] = Object.entries(
  DEFAULT_SERIES_MAPPING,
).map(([id, name]) => ({
  id,
  name: SERIES_LABELS_RU[id] ?? name,
  color: COLORS[id],
  visible: ["ps_pressure", "recirc_density"].includes(id),
}));
