import html from "./axis-settings-modal.html?raw";
import type { ChartManager } from "../../charts/chart-manager";

let chartRef: ChartManager | null = null;

const groupToAxisIndex: Record<string, number> = {
  density: 0,
  pressure: 1,
  rate_bpm: 2,
  rate_gpm: 3,
  volume_bbl: 4,
  volume_gal: 5,
  valves: 6,
};

function fillInputsFromSelectedGroup() {
  const groupSelect = document.getElementById(
    "axis-group",
  ) as HTMLSelectElement;
  const minInput = document.getElementById("axis-min") as HTMLInputElement;
  const maxInput = document.getElementById("axis-max") as HTMLInputElement;

  if (!groupSelect || !minInput || !maxInput) return;

  const axisIndex = groupToAxisIndex[groupSelect.value];
  const range = chartRef?.getManualAxisRange(axisIndex);

  minInput.value = range ? String(range.min) : "";
  maxInput.value = range ? String(range.max) : "";
}

export function initAxisSettingsModal(chart: ChartManager) {
  chartRef = chart;
  const anchor = document.getElementById("modals-anchor");
  if (!anchor) {
    console.warn("No modals anchor found");
    return;
  }

  anchor.insertAdjacentHTML("beforeend", html);

  const modal = document.getElementById("axis-settings-modal");
  const closeBtn = document.getElementById("axis-close");
  const applyBtn = document.getElementById("axis-apply");
  const groupSelect = document.getElementById("axis-group");

  groupSelect?.addEventListener("change", () => {
    fillInputsFromSelectedGroup();
  });

  closeBtn?.addEventListener("click", () => {
    modal?.classList.add("hidden");
  });

  applyBtn?.addEventListener("click", () => {
    const group = (document.getElementById("axis-group") as HTMLSelectElement)
      .value;

    const axisIndex = groupToAxisIndex[group];

    const min = Number(
      (document.getElementById("axis-min") as HTMLInputElement).value,
    );

    const max = Number(
      (document.getElementById("axis-max") as HTMLInputElement).value,
    );

    if (
      axisIndex === undefined ||
      !Number.isFinite(min) ||
      !Number.isFinite(max)
    ) {
      console.warn("Некорректные значения оси");
      return;
    }

    if (min >= max) {
      console.warn("Min должен быть меньше Max");
      return;
    }
    //FIXME ДЕбаг
    console.log("Axis apply click:", {
      chartRef,
      axisIndex,
      min,
      max,
    });

    chartRef?.applyManualSettings({
      axisIndex,
      min,
      max,
    });

    modal?.classList.add("hidden");
  });

  //FIXME Дебаг удалить
  console.log("Axis settings modal initialized");
}

export function openAxisSettingsModal() {
  const modal = document.getElementById("axis-settings-modal");
  if (!modal) return;

  fillInputsFromSelectedGroup();

  modal.classList.remove("hidden");
}
