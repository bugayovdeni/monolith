import { ChartManager } from "../../charts/chart-manager";

export function initSeriesPanel(
  chart: ChartManager,
  seriesColors: Record<string, string>,
) {
  // === toggle-all ===
  document.getElementById("toggle-all")?.addEventListener("click", () => {
    const checkboxes = document.querySelectorAll(
      '.checkbox-grid input[type="checkbox"]',
    );

    const firstChecked = (checkboxes[0] as HTMLInputElement)?.checked;

    checkboxes.forEach((cb) => {
      (cb as HTMLInputElement).checked = !firstChecked;
    });
  });

  // === apply ===
  const applyBtn = document.querySelector(".btn-action") as HTMLButtonElement;
  const sidebarLeft = document.querySelector(".sidebar-left") as HTMLElement;

  applyBtn?.addEventListener("click", () => {
    const checkedInputs = sidebarLeft.querySelectorAll(
      'input[type="checkbox"]:checked',
    );

    const visibleSeriesIds = Array.from(checkedInputs)
      .map((input) => (input as HTMLInputElement).dataset.series)
      .filter((id): id is string => !!id);

    chart.toggleSeriesVisibility(visibleSeriesIds, seriesColors);

    console.log(
      `🎨 График обновлён. Видимые серии: ${
        visibleSeriesIds.join(", ") || "пусто"
      }`,
    );
  });
}
