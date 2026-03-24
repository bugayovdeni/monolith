// src/types/csv.ts

// === Основные типы ===

/** Уникальный идентификатор сессии (UUID v4 в строковом формате) */
export type SessionId = string;

/** Unix timestamp в миллисекундах (для совместимости с JS Date) */
export type TimestampMs = number;

/** ISO 8601 дата-время строка (от chrono::DateTime<Utc>) */
export type IsoDateTime = string;

// === Единицы измерения (CementingUnits) ===
//TODO Уточнить поля, реализация CementingUnits
export interface CementingUnits {
  recirc_density: string;      // например "г/см³"
  downhole_density: string;
  mix_water_rate: string;      // например "л/мин"
  combo_rate: string;
  ps_pressure: string;         // например "МПа"
  ds_pressure: string;
  mix_wtr_stg_ttl: string;
  mix_wtr_job_ttl: string;
  combo_pump_stg_ttl: string;
  combo_pump_job_ttl: string;
  ps_rate: string;
  ds_rate: string;
  cement_vlv_percent: string;  // "%"
  wtr_vlv_percent: string;
  [key: string]: string;       // на всякий случай
}

// === Одна запись данных (CementingRecord) ===
// Все числовые поля — как в Rust: f32 → number, u8 → number
export interface CementingRecord {
  // === f32 поля (12 шт) ===
  "Recirc Density": number;        // ← сериализуется как есть, если не переименовать
  "Downhole Density": number;
  "Mix Water Rate": number;
  "Combo Rate": number;
  "PS Pressure": number;
  "DS Pressure": number;
  "Mix Wtr Stg Ttl": number;
  "Mix Wtr Job Ttl": number;
  "Combo Pump Stg Ttl": number;
  "Combo Pump Job Ttl": number;
  "PS Rate": number;
  "DS Rate": number;

  // === u8 поля (4 шт, 0-100 или 0-255) ===
  "Cement Vlv Percent": number;
  "Wtr Vlv Percent": number;
  "Digital Outs": number;
  "Event Num": number;
}


// Rust
// #[serde(rename_all = "snake_case")]
// И тогда в ТС будет удобно: data.recirc_density

// === Метаданные файла (CsvMetadata) ===
export interface CsvMetadata {
  file_path: string;
  file_name: string;
  total_rows: number;
  file_size_bytes: number;
  record_size_bytes: number;
  parsed_at: IsoDateTime;
  [key: string]: any;
}

// === Главный агрегат (CementingData) ===
export interface CementingData {
  // === Идентификация ===
  id: SessionId;              // UUID string
  file_path: string;
  file_name: string;

  // === Время ===
  start_time: string;    // RFC3339, например "2026-01-23T13:39:00Z"
  end_time: string;
  sample_interval_ms: number; // обычно 1000

  // === Единицы ===
  units: any;

  // === Данные ===
  records: Record<string, any>[];

  // === Мета ===
  meta: any;

  // === Методы-хелперы (опционально, если добавлять на фронт) ===
  // duration_human?: string; // можно вычислять на фронте
}

// === Вспомогательные DTO для графиков ===

/** Точка для графика (ChartPoint) */
export interface ChartPoint {
  x_ms: TimestampMs;          // Unix ms, для new Date()
  y: number;                  // значение метрики
  time_label: string;         // "13:39:05" для тултипа
}

/** Статистика по полю (FieldStats) */
export interface FieldStats {
  min: number;
  max: number;
  avg: number;
}

// === События и запросы ===

/** Пейлоад события csv://loaded */
export interface CsvLoadedEvent {
  session_id: SessionId;
  filename: string;
  status: 'loaded';
  records_count?: number;
  time_range?: { start: IsoDateTime; end: IsoDateTime };
}

/** Пейлоад события csv://error */
export interface CsvErrorEvent {
  message: string;
  code?: string;
  details?: Record<string, any>;
}

/** Запрос на получение данных (вход для get_csv_data) */
export interface GetCsvDataRequest {
  session_id: SessionId;
}

/** Опционально: запрос с фильтрацией (на будущее) */
export interface GetCsvDataFilteredRequest extends GetCsvDataRequest {
  fields?: Array<keyof CementingRecord>;  // какие поля вернуть
  time_from?: IsoDateTime;                // фильтр по времени
  time_to?: IsoDateTime;
  limit?: number;                         // пагинация
}