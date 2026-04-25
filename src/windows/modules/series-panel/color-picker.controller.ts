import { ChartManager } from "../../charts/chart-manager";

export function initColorPicker(
  chart: ChartManager,
  seriesColors: Record<string, string>,
  defaultColors: Record<string, string>,
) {
  document
    .querySelectorAll<HTMLButtonElement>("[data-color-series]")
    .forEach((btn) => {
      btn.addEventListener("click", (e) => {
        const btn = e.currentTarget as HTMLButtonElement;
        const seriesId = btn.dataset.colorSeries;

        if (!seriesId) return;

        const input = document.createElement("input");
        input.type = "color";
        input.value =
          seriesColors[seriesId] ?? defaultColors[seriesId] ?? "#ffffff";

        input.style.position = "absolute";
        input.style.left = "-9999px";

        document.body.appendChild(input);

        input.addEventListener("input", () => {
          const color = input.value;

          seriesColors[seriesId] = color;
          chart.setSeriesColor(seriesId, color);

          btn.style.background = color;
          btn.dataset.color = color;
        });

        input.click();

        input.addEventListener("change", () => {
          input.remove();
        });
      });
    });
}
