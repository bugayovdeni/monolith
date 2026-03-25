// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";
// import { getCurrentWindow } from '@tauri-apps/api/window';
import { ChartManager, SeriesConfig} from '../charts/chart-manager';
//FIXME Logo
// import logoSrc from '../../assets/monolith.svg';
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

window.addEventListener("DOMContentLoaded", async () => {
    try {
    await csvManager.init();
    } catch (err) {
    console.error('❌ Не удалось инициализировать CsvManager:', err);
  }
  //FIXME LOGO
  // const logo = document.querySelector('.app-logo') as HTMLImageElement
  // if (logo) {
  //   logo.src = logoSrc
  // }
});

// ==== САЙДБАРЫ ====
window.addEventListener('DOMContentLoaded', () => {
  // Левый сайдбар
  const sidebarLeft = document.getElementById('sidebar-left');
  const btnLeft = document.querySelector('.toggle-btn-left') as HTMLButtonElement;
  
  if (sidebarLeft && btnLeft) {
    btnLeft.addEventListener('click', (e) => {
      e.stopPropagation(); // Чтобы клик не ушёл выше
      sidebarLeft.classList.toggle('hidden');
    });
  }

    // Правый сайдбар
  const sidebarRight = document.getElementById('sidebar-right');
  const btnRight = document.querySelector('#sidebar-right .toggle-btn') as HTMLButtonElement;
  
  if (sidebarRight && btnRight) {
    btnRight.addEventListener('click', (e) => {
      e.stopPropagation();
      sidebarRight.classList.toggle('hidden');
    });
  }
});

document.getElementById('toggle-all')?.addEventListener('click', () => {
  const checkboxes = document.querySelectorAll('.checkbox-grid input[type="checkbox"]');
  // Проверяем первый, чтобы понять, что делать
  const firstChecked = (checkboxes[0] as HTMLInputElement)?.checked;
  
  checkboxes.forEach((cb) => {
    (cb as HTMLInputElement).checked = !firstChecked;
    // Тут должен быть триггер перерисовки графика, если у тебя там реактивщина
    // cb.dispatchEvent(new Event('change')); 
  });
});

//TODO Выбор графиков
const applyBtn = document.querySelector('.btn-action') as HTMLButtonElement;
const sidebarLeft = document.querySelector('.sidebar-left') as HTMLElement;

applyBtn?.addEventListener('click', () => {
  // Собираем только отмеченные чекбоксы, у которых есть data-series
  const checkedInputs = sidebarLeft.querySelectorAll('input[type="checkbox"]:checked');
  const visibleSeriesIds = Array.from(checkedInputs)
    .map(input => (input as HTMLInputElement).dataset.series)
    .filter((id): id is string => !!id); // Отсеиваем undefined

  // Пинаем график
  chart.toggleSeriesVisibility(visibleSeriesIds);
  
  console.log(`🎨 График обновлён. Видимые серии: ${visibleSeriesIds.join(', ') || 'пусто'}`);
});
