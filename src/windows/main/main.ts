// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";
// import { getCurrentWindow } from '@tauri-apps/api/window';
import { ChartManager, SeriesConfig, type DataPoint  } from '../charts/chart-manager';
import logoSrc from '../../assets/monolith.svg';
import { CsvManager, DEFAULT_SERIES_MAPPING } from '../../services/csv/csv-manager';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;
// Палитра цветов
const COLORS: Record<string, string> = {
  'ps_pressure': '#e74c3c', 'ds_pressure': '#c0392b',
  'recirc_density': '#3498db', 'downhole_density': '#2980b9',
  'mix_water_rate': '#2ecc71', 'combo_rate': '#27ae60',
  'ps_rate': '#1abc9c', 'ds_rate': '#16a085',
  'mix_wtr_stg_ttl': '#9b59b6', 'mix_wtr_job_ttl': '#8e44ad',
  'combo_pump_stg_ttl': '#34495e', 'combo_pump_job_ttl': '#2c3e50',
  'cement_vlv_percent': '#e67e22', 'wtr_vlv_percent': '#d35400',
};

// Конфиг серий
const chartSeries: SeriesConfig[] = Object.entries(DEFAULT_SERIES_MAPPING).map(([id, name]) => ({
  id, name, color: COLORS[id], visible: ['ps_pressure', 'recirc_density'].includes(id)
}));

// Создаём менеджер графика
const chart = new ChartManager('chart-container', chartSeries);
// Создаём CSV-менеджер и связываем с графиком
const csvManager = new CsvManager({
  onDataUpdate: (points) => chart.updateData(points),
  onBulkUpdate: (bulk) => chart.loadBulkData(bulk) 
});

// Функция-помощник для навешивания событий
// function bindWindowAction(id: string, action: () => Promise<void>) {
//   const element = document.getElementById(id);
//   if (element) {
//     element.addEventListener('click', () => {
//       action().catch((err) => console.error(`Ошибка ${id}:`, err));
//     });
//   } else {
//     console.warn(`Элемент ${id} не найден`);
//   }
// }

// let greetInputEl: HTMLInputElement | null;
// let greetMsgEl: HTMLElement | null;

//FIXME
// async function greet() {
//   if (greetMsgEl && greetInputEl) {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     greetMsgEl.textContent = await invoke("greet", {
//       name: greetInputEl.value,
//     });
//   }
// }

//FIXME
window.addEventListener("DOMContentLoaded", async () => {
  // greetInputEl = document.querySelector("#greet-input");
  // greetMsgEl = document.querySelector("#greet-msg");
  // document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   greet();
  // });
    try {
    await csvManager.init();
  } catch (err) {
    console.error('❌ Не удалось инициализировать CsvManager:', err);
  }

  const logo = document.querySelector('.app-logo') as HTMLImageElement
  if (logo) {
    logo.src = logoSrc
  }
});

//FIXME ==== ГРАФИКИ ==
// setInterval(() => {
//   const now = Date.now();

//   chart.updateData(
//     [now, Math.random() * 100],
//     [now, Math.random() * 50]
//   );
// }, 1000);

// ==== Side Bar ===
window.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.getElementById('sidebar')!;
  const btn = document.querySelector('.toggle-btn') as HTMLButtonElement;

  btn.addEventListener('click', () => {
    console.log('Toggle, current classes:', sidebar.classList);
    sidebar.classList.toggle('hidden');
  });
});

