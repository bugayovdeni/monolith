import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ChartManager } from "../charts/chart-manager";
import { portDialog } from "../modules/seria-port/dialog";
import { CsvManager } from "../../services/csv/csv-manager";
import { chartSeries, COLORS, seriesColors } from "../charts/chart-config";
import { initLiveData } from "../data/live/live-data.controller";
import { initSidebars } from "../modules/sidebar/sidebar.controller";
import { initSeriesPanel } from "../modules/series-panel/series-panel.controller";
import { initColorPicker } from "../modules/series-panel/color-picker.controller";

export function startMainApp() {
  const chart = new ChartManager("chart-container", chartSeries);

  let dataMode: "idle" | "live" | "archive" = "idle";

  initLiveData(chart, () => dataMode);

  const csvManager = new CsvManager({
    onDataUpdate: (points) => {
      if (dataMode !== "archive") return;
      chart.updateData(points);
    },
    onBulkUpdate: async (bulk) => {
      await invoke("stop_serial");

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

    initSidebars();
    initSeriesPanel(chart, seriesColors);
    initColorPicker(chart, seriesColors, COLORS);

    const anchor = document.getElementById("modals-anchor");
    if (anchor) portDialog.init(anchor);

    const btnOpen = document.getElementById("твой-id-кнопки");
    if (btnOpen) {
      btnOpen.addEventListener("click", () => portDialog.open());
    }
  });

  window.addEventListener("port:connected", (e: any) => {
    dataMode = "live";
    chart.clear();

    const marquee = document.querySelector(".marquee-content");
    if (marquee) {
      marquee.textContent = `● ПЛК Подключен: ${e.detail}`;
    }
  });

  listen("show-port-dialog", () => {
    portDialog.open();
  });
}
