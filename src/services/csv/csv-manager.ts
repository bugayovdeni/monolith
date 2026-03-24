import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";
import { CementingData, CsvLoadedEvent } from '../../types/csv';
import { DataPoint } from '../../windows/charts/chart-manager';

export const DEFAULT_SERIES_MAPPING: Record<string, string> = {
  'recirc_density': 'Recirc Density',
  'downhole_density': 'Downhole Density',
  'mix_water_rate': 'Mix Water Rate',
  'combo_rate': 'Combo Rate',
  'ps_pressure': 'PS Pressure',
  'ds_pressure': 'DS Pressure',
  'mix_wtr_stg_ttl': 'Mix Wtr Stg Ttl',
  'mix_wtr_job_ttl': 'Mix Wtr Job Ttl',
  'combo_pump_stg_ttl': 'Combo Pump Stg Ttl',
  'combo_pump_job_ttl': 'Combo Pump Job Ttl',
  'ps_rate': 'PS Rate',
  'ds_rate': 'DS Rate',
  'cement_vlv_percent': 'Cement Vlv Percent',
  'wtr_vlv_percent': 'Wtr Vlv Percent',
};

export interface CsvManagerOptions {
  onDataUpdate?: (points: Record<string, DataPoint>) => void;
  onBulkUpdate?: (bulkData: Record<string, DataPoint[]>) => void;
  fieldMapping?: Record<string, string>;
}

export class CsvManager {
  private sessionId: string | null = null;
  private mapping: Record<string, string>;

  constructor(private readonly options: CsvManagerOptions = {}) {
    this.mapping = options.fieldMapping ?? DEFAULT_SERIES_MAPPING;
  }

  async init(): Promise<void> {
    try {
      await listen<CsvLoadedEvent>('csv://loaded', (event: Event<CsvLoadedEvent>) => {
        console.log('📥 CsvManager: новая сессия', event.payload.session_id);
        this.sessionId = event.payload.session_id;
        this.fetchAndProcess();
      });
      console.log('✅ CsvManager: подписка активна');
    } catch (err) {
      console.error('❌ CsvManager: ошибка подписки:', err);
      throw err;
    }
  }

private async fetchAndProcess(): Promise<void> {
  if (!this.sessionId) return;
  try {
    const data = await invoke<CementingData>('get_csv_data', { sessionId: this.sessionId });
    console.log(`✅ CsvManager: ${data.file_name}, ${data.records.length} записей`);

    const startTimeMs = new Date(data.start_time as string).getTime();
    const interval = data.sample_interval_ms || 1000;

    // 👇 1. Создаём буфер для всех точек: { seriesId: [ [ts, val], [ts, val], ... ] }
    const bulkData: Record<string, DataPoint[]> = {};
    for (const seriesId of Object.keys(this.mapping)) {
      bulkData[seriesId] = [];
    }

    // 👇 2. Просто собираем данные, НИЧЕГО НЕ РИСУЕМ
    data.records.forEach((rec, index) => {
      const ts = startTimeMs + index * interval;
      
      for (const [seriesId, jsonFieldName] of Object.entries(this.mapping)) {
        const val = rec[jsonFieldName];
        if (val !== undefined && val !== null) {
          const num = Number(val);
          if (!isNaN(num)) {
            bulkData[seriesId].push([ts, num]);
          }
        }
      }
    });

    // 👇 3. ОДИН РАЗ кидаем всё в график
    if (this.options.onBulkUpdate) {
      this.options.onBulkUpdate(bulkData);
    }

  } catch (err) {
    console.error('💥 CsvManager: ошибка:', err);
  }
}

}