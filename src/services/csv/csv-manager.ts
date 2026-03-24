import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";
import { CementingData, CsvLoadedEvent } from '../../types/csv';
import { DataPoint } from '../../windows/charts/chart-manager';

// ==== Утилиты для маркеза ====

/** Пытается выдрать дату из имени файла: хорошо_2025-03-24.csv → Date */
function parseDateFromFilename(filename: string): Date | null {
  const isoMatch = filename.match(/(\d{4}-\d{2}-\d{2})/);
  if (isoMatch?.[1]) {
    const date = new Date(isoMatch[1]);
    if (!isNaN(date.getTime())) return date;
  }
  // Если вдруг попадётся 20250324 без дефисов — раскомментируй
  /*
  const compact = filename.match(/(\d{4})(\d{2})(\d{2})/);
  if (compact) {
    const [, y, m, d] = compact;
    const date = new Date(`${y}-${m}-${d}`);
    if (!isNaN(date.getTime())) return date;
  }
  */
  return null;
}

/** Форматирует дату по-человечески: 24.03.2025 */
function formatRuDate(date: Date): string {
  return date.toLocaleDateString('ru-RU', {
    day: '2-digit', month: '2-digit', year: 'numeric'
  });
}

/** Обновляет текст в маркезе. Вызывается ОДИН РАЗ за сессию. */
export function updateMarqueeStatus(text: string): void {
  // Проверяем, что мы в браузере/окне, а не в тестах
  if (typeof document !== 'undefined') {
    const el = document.querySelector('.marquee-content') as HTMLElement;
    if (el) {
      el.innerHTML = `${text} &nbsp;&nbsp;&nbsp;`;
    }
  }
}

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
  private marqueeUpdated = false;

  constructor(private readonly options: CsvManagerOptions = {}) {
    this.mapping = options.fieldMapping ?? DEFAULT_SERIES_MAPPING;
  }

  async init(): Promise<void> {
    try {
      await listen<CsvLoadedEvent>('csv://loaded', (event: Event<CsvLoadedEvent>) => {
        console.log('📥 CsvManager: новая сессия', event.payload.session_id);
        this.sessionId = event.payload.session_id;
        this.marqueeUpdated = false; 

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

    // ==== 🎯 Пытаемся определить дату для маркеза ====
    let workDate: Date | null = parseDateFromFilename(data.file_name);
    
    // Если в имени файла даты нет — пробуем выдрать из первой записи
    if (!workDate && data.records.length > 0 && data.start_time) {
      workDate = new Date(data.start_time as string);
      if (isNaN(workDate.getTime())) workDate = null;
    }

    const startTimeMs = new Date(data.start_time as string).getTime();
    const interval = data.sample_interval_ms || 1000;

    const bulkData: Record<string, DataPoint[]> = {};
    for (const seriesId of Object.keys(this.mapping)) {
      bulkData[seriesId] = [];
    }

    let validPointsCount = 0;

    data.records.forEach((rec, index) => {
      const ts = startTimeMs + index * interval;
      
      for (const [seriesId, jsonFieldName] of Object.entries(this.mapping)) {
        const val = rec[jsonFieldName];
        if (val !== undefined && val !== null) {
          const num = Number(val);
          if (!isNaN(num)) {
            bulkData[seriesId].push([ts, num]);
            validPointsCount++;
          }
        }
      }
    });

    // ==== 🚨 Проверка на «битый» файл ====
    if (validPointsCount === 0 || Object.values(bulkData).every(arr => arr.length === 0)) {
      console.warn('⚠️ CsvManager: файл пустой или данные не распарсились');
      if (!this.marqueeUpdated) {
        updateMarqueeStatus('⚠️ Файл битый, данных нет');
        this.marqueeUpdated = true;
      }
      return; // Не дёргаем график, нет смысла
    }

    // ==== ✅ Успех — обновляем маркез (если ещё не обновили) ====
    if (!this.marqueeUpdated && workDate) {
      updateMarqueeStatus(`Работа от ${formatRuDate(workDate)}`);
      this.marqueeUpdated = true;
    }

    // Кидаем данные в график
    if (this.options.onBulkUpdate) {
      this.options.onBulkUpdate(bulkData);
    }

  } catch (err) {
    console.error('💥 CsvManager: критическая ошибка:', err);
    // Если всё упало — честно говорим в маркез
    if (!this.marqueeUpdated) {
      updateMarqueeStatus('💥 Ошибка загрузки данных');
      this.marqueeUpdated = true;
    }
  }
}

}