// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";
// import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from "@tauri-apps/api/event";
import { ChartManager, SeriesConfig } from "../charts/chart-manager";
import { portDialog } from "../modules/seria-port/dialog";

import {
  CsvManager,
  DEFAULT_SERIES_MAPPING,
} from "../../services/csv/csv-manager";

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;
// Палитра цветов
const COLORS: Record<string, string> = {
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

// Конфиг серий
const chartSeries: SeriesConfig[] = Object.entries(DEFAULT_SERIES_MAPPING).map(
  ([id, name]) => ({
    id,
    name,
    color: COLORS[id],
    visible: ["ps_pressure", "recirc_density"].includes(id),
  }),
);

//NOTE Manager Chart
//Создаём менеджер графика
const chart = new ChartManager("chart-container", chartSeries);
// === РЕЖИМ ДАННЫХ ===
let dataMode: "idle" | "live" | "archive" = "idle";
// Создаём CSV-менеджер и связываем с графиком
const csvManager = new CsvManager({
  onDataUpdate: (points) => {
    if (dataMode !== "archive") return;
    chart.updateData(points);
  },
  onBulkUpdate: (bulk) => {
    dataMode = "archive";
    chart.clear();
    chart.loadBulkData(bulk);
  },
});

window.addEventListener("DOMContentLoaded", async () => {
  try {
    await csvManager.init();
  } catch (err) {
    console.error("❌ Не удалось инициализировать CsvManager:", err);
  }

  // === МОДАЛКА: ИНИЦИАЛИЗАЦИЯ ===
  const anchor = document.getElementById("modals-anchor");
  if (anchor) portDialog.init(anchor);

  // === КНОПКА ОТКРЫТИЯ ===
  const btnOpen = document.getElementById("твой-id-кнопки"); // например "menu-connect"
  if (btnOpen) {
    btnOpen.addEventListener("click", () => portDialog.open());
  }
});

// ==== САЙДБАРЫ ====
window.addEventListener("DOMContentLoaded", () => {
  // Левый сайдбар
  const sidebarLeft = document.getElementById("sidebar-left");
  const btnLeft = document.querySelector(
    ".toggle-btn-left",
  ) as HTMLButtonElement;

  if (sidebarLeft && btnLeft) {
    btnLeft.addEventListener("click", (e) => {
      e.stopPropagation(); // Чтобы клик не ушёл выше
      sidebarLeft.classList.toggle("hidden");
    });
  }

  // Правый сайдбар
  const sidebarRight = document.getElementById("sidebar-right");
  const btnRight = document.querySelector(
    "#sidebar-right .toggle-btn",
  ) as HTMLButtonElement;

  if (sidebarRight && btnRight) {
    btnRight.addEventListener("click", (e) => {
      e.stopPropagation();
      sidebarRight.classList.toggle("hidden");
    });
  }
});

document.getElementById("toggle-all")?.addEventListener("click", () => {
  const checkboxes = document.querySelectorAll(
    '.checkbox-grid input[type="checkbox"]',
  );
  // Проверяем первый, чтобы понять, что делать
  const firstChecked = (checkboxes[0] as HTMLInputElement)?.checked;

  checkboxes.forEach((cb) => {
    (cb as HTMLInputElement).checked = !firstChecked;
    // Тут должен быть триггер перерисовки графика, если у тебя там реактивщина
    // cb.dispatchEvent(new Event('change'));
  });
});

//TODO Выбор графиков
const applyBtn = document.querySelector(".btn-action") as HTMLButtonElement;
const sidebarLeft = document.querySelector(".sidebar-left") as HTMLElement;

applyBtn?.addEventListener("click", () => {
  // Собираем только отмеченные чекбоксы, у которых есть data-series
  const checkedInputs = sidebarLeft.querySelectorAll(
    'input[type="checkbox"]:checked',
  );
  const visibleSeriesIds = Array.from(checkedInputs)
    .map((input) => (input as HTMLInputElement).dataset.series)
    .filter((id): id is string => !!id); // Отсеиваем undefined

  // Пинаем график
  chart.toggleSeriesVisibility(visibleSeriesIds);

  console.log(
    `🎨 График обновлён. Видимые серии: ${visibleSeriesIds.join(", ") || "пусто"}`,
  );
});

// === ОБНОВЛЕНИЕ СТАТУСА ПРИ КОННЕКТЕ ===
window.addEventListener("port:connected", (e: any) => {
  dataMode = "live";
  chart.clear();

  const marquee = document.querySelector(".marquee-content");
  if (marquee) {
    marquee.textContent = `● ПЛК Подключен: ${e.detail} | Файл не загружен...`;
  }
});

// === СЛУШАЕМ СОБЫТИЕ ОТ RUST ===
listen("show-port-dialog", () => {
  portDialog.open(); // 👈 Модалка вылезает, когда Раст прислал сигнал
});

// === ASCII ПОТОК ИЗ RUST ===
listen("ascii-record", (event) => {
  if (dataMode !== "live") return;

  const raw = event.payload as Record<string, number>;
  const now = Date.now();

  chart.updateData({
    recirc_density: [now, raw["Recirc Density"]],
    downhole_density: [now, raw["Downhole Density"]],
    mix_water_rate: [now, raw["Mix Water Rate"]],
    combo_rate: [now, raw["Combo Rate"]],
    ps_pressure: [now, raw["PS Pressure"]],
    ds_pressure: [now, raw["DS Pressure"]],
    mix_wtr_stg_ttl: [now, raw["Mix Wtr Stg Ttl"]],
    mix_wtr_job_ttl: [now, raw["Mix Wtr Job Ttl"]],
    combo_pump_stg_ttl: [now, raw["Combo Pump Stg Ttl"]],
    combo_pump_job_ttl: [now, raw["Combo Pump Job Ttl"]],
    cement_vlv_percent: [now, raw["Cement Vlv Percent"]],
    wtr_vlv_percent: [now, raw["Wtr Vlv Percent"]],
    ps_rate: [now, raw["PS Rate"]],
    ds_rate: [now, raw["DS Rate"]],
  });

  console.log("[FRONT] chart updated from ascii-record");
});
