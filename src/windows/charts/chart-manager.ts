import * as echarts from "echarts/core";
import { LineChart } from "echarts/charts";
import {
  GridComponent,
  TooltipComponent,
  LegendComponent,
  DataZoomComponent,
  TitleComponent,
} from "echarts/components";
import { CanvasRenderer } from "echarts/renderers";

echarts.use([
  LineChart,
  GridComponent,
  TooltipComponent,
  LegendComponent,
  DataZoomComponent,
  TitleComponent,
  CanvasRenderer,
]);

// ===== Конфиг группировки полей по осям =====
export const FIELD_AXIS_MAP: Record<
  string,
  {
    group: string;
    axisIndex: number;
    label: string;
    min?: number;
    max?: number;
  }
> = {
  // 0. Density
  recirc_density: {
    group: "density",
    axisIndex: 0,
    label: "ppg",
    min: 8.0,
    max: 20.0,
  },
  downhole_density: {
    group: "density",
    axisIndex: 0,
    label: "ppg",
    min: 8.0,
    max: 20.0,
  },

  // 1. Pressure
  ps_pressure: { group: "pressure", axisIndex: 1, label: "psi", max: 15000 },
  ds_pressure: { group: "pressure", axisIndex: 1, label: "psi", max: 15000 },

  // 2. Pump rates
  ps_rate: { group: "rate_bpm", axisIndex: 2, label: "bpm" },
  ds_rate: { group: "rate_bpm", axisIndex: 2, label: "bpm" },
  combo_rate: { group: "rate_bpm", axisIndex: 2, label: "bpm" },

  // 3. Water rate
  mix_water_rate: { group: "rate_gpm", axisIndex: 3, label: "gpm" },

  // 4. Pump totals
  combo_pump_stg_ttl: { group: "volume_bbl", axisIndex: 4, label: "bbl" },
  combo_pump_job_ttl: { group: "volume_bbl", axisIndex: 4, label: "bbl" },

  // 5. Water totals
  mix_wtr_stg_ttl: { group: "volume_gal", axisIndex: 5, label: "gal" },
  mix_wtr_job_ttl: { group: "volume_gal", axisIndex: 5, label: "gal" },

  // 6. Valves
  cement_vlv_percent: {
    group: "valves",
    axisIndex: 6,
    label: "%",
    min: 0,
    max: 100,
  },
  wtr_vlv_percent: {
    group: "valves",
    axisIndex: 6,
    label: "%",
    min: 0,
    max: 100,
  },
};

export const getAxisForField = (fieldId: string) =>
  FIELD_AXIS_MAP[fieldId]?.axisIndex ?? 0;
export const getFieldConfig = (fieldId: string) =>
  FIELD_AXIS_MAP[fieldId] ?? null;

export const getAxisConfig = (axisIndex: number) => {
  const sample = Object.values(FIELD_AXIS_MAP).find(
    (cfg) => cfg.axisIndex === axisIndex,
  );
  if (!sample) return null;

  const position: "left" | "right" = axisIndex === 0 ? "left" : "right";

  return {
    position,
    name: sample.label,
    nameLocation: "middle" as const,
    nameGap: 35,
    min: sample.min,
    max: sample.max,
    scale: true,
    splitLine: {
      lineStyle: {
        type: axisIndex === 0 ? "solid" : "dashed",
        opacity: axisIndex === 0 ? 0.8 : 0.4,
      },
    },
    axisLabel: {
      margin: 8,
      formatter: (val: number) => {
        if (sample.label === "%") return `${Math.round(val)}`;
        return val >= 1000 ? `${(val / 1000).toFixed(1)}k` : `${val}`;
      },
    },
  };
};
// ===== Конец конфига =====

export type DataPoint = [number, number];

export interface SeriesConfig {
  id: string;
  name: string;
  color?: string;
  visible?: boolean;
}

interface VisibleAxisState {
  usedLogicalAxisIndices: number[];
  logicalToRealAxisIndex: Map<number, number>;
  rightAxisCount: number;
  yAxis: any[];
}

export class ChartManager {
  private chart: echarts.ECharts | null = null;
  private container: HTMLElement;
  private resizeObserver: ResizeObserver | null = null;
  private seriesData: Map<string, DataPoint[]> = new Map();
  private seriesConfig: SeriesConfig[] = [];

  // Для 1 Гц лучше сначала держать не миллион, а более реалистичное окно
  private readonly MAX_POINTS = 50_000;

  constructor(containerId: string, seriesConfigs: SeriesConfig[]) {
    const el = document.getElementById(containerId);
    if (!el) throw new Error(`Элемент "${containerId}" не найден`);

    this.container = el;
    this.seriesConfig = seriesConfigs.map((cfg) => ({
      ...cfg,
      visible: false,
    }));

    seriesConfigs.forEach((cfg) => this.seriesData.set(cfg.id, []));
    this.init();
  }

  private init() {
    this.chart = echarts.init(this.container, null, { renderer: "canvas" });
    this.chart.setOption(this.buildOption(), { notMerge: true });
    this.observeResize();

    console.log(
      "🔍 Реальные ID серий:",
      this.seriesConfig.map((c) => c.id),
    );
  }

  private buildOption() {
    const visibleConfigs = this.getVisibleSeriesConfigs();

    if (visibleConfigs.length === 0) {
      return {
        title: {
          text: "Нет выбранных графиков",
          left: "center",
          top: "middle",
        },
        tooltip: {},
        legend: { data: [] },
        xAxis: { type: "category", data: [] },
        yAxis: { type: "value" },
        series: [],
      };
    }

    const axisState = this.buildVisibleAxisState(visibleConfigs);
    const visibleConfigByName = new Map(
      visibleConfigs.map((cfg) => [cfg.name, cfg]),
    );

    return {
      animation: false,

      tooltip: {
        trigger: "axis",
        axisPointer: { type: "cross" },
        formatter: (params: any[]) => {
          if (!params?.length) return "";

          const time = new Date(params[0].axisValue).toLocaleTimeString();

          const rows = params
            .map((p) => {
              const cfg = visibleConfigByName.get(p.seriesName);
              const fieldCfg = cfg ? getFieldConfig(cfg.id) : null;
              const val = Array.isArray(p.value) ? p.value[1] : p.value;
              const formattedVal =
                typeof val === "number" ? val.toFixed(2) : val;

              return `${p.marker} ${p.seriesName}: <b>${formattedVal} ${fieldCfg?.label || ""}</b>`;
            })
            .join("<br/>");

          return `<b>${time}</b><br/>${rows}`;
        },
      },

      legend: {
        data: visibleConfigs.map((cfg) => cfg.name),
        type: "scroll",
        bottom: 0,
      },

      grid: {
        left: "50px",
        right:
          axisState.rightAxisCount > 0
            ? `${80 + (axisState.rightAxisCount - 1) * 50}px`
            : "40px",
        top: "40px",
        bottom: "40px",
        containLabel: false,
      },

      xAxis: {
        type: "time",
        boundaryGap: false,
        axisLabel: {
          formatter: (v: number) => new Date(v).toLocaleTimeString(),
        },
      },

      yAxis: axisState.yAxis,

      dataZoom: [
        { id: "insideZoom", type: "inside", start: 0, end: 100 },
        {
          id: "sliderZoom",
          type: "slider",
          start: 0,
          end: 100,
          height: 20,
          bottom: 20,
        },
      ],

      series: this.buildSeries(
        visibleConfigs,
        axisState.logicalToRealAxisIndex,
      ),
    };
  }

  private buildVisibleAxisState(
    visibleConfigs: SeriesConfig[],
  ): VisibleAxisState {
    const usedLogicalAxisIndices = Array.from(
      new Set(visibleConfigs.map((cfg) => getAxisForField(cfg.id))),
    ).sort((a, b) => a - b);

    const logicalToRealAxisIndex = new Map<number, number>();
    usedLogicalAxisIndices.forEach((logicalIdx, realIdx) => {
      logicalToRealAxisIndex.set(logicalIdx, realIdx);
    });

    let rightAxisCount = 0;

    const yAxis = usedLogicalAxisIndices.map((logicalIdx) => {
      const axisCfg = getAxisConfig(logicalIdx);
      const isRight = axisCfg?.position === "right";

      let offset = 0;
      if (isRight) {
        offset = rightAxisCount * 55;
        rightAxisCount++;
      }

      return {
        type: "value" as const,
        scale: axisCfg?.scale ?? true,
        position: axisCfg?.position ?? "left",
        offset,
        name: axisCfg?.name,
        nameLocation: axisCfg?.nameLocation ?? "middle",
        nameGap: axisCfg?.nameGap ?? 35,
        min: axisCfg?.min,
        max: axisCfg?.max,
        splitLine: axisCfg?.splitLine,
        axisLabel: axisCfg?.axisLabel,
      };
    });

    return {
      usedLogicalAxisIndices,
      logicalToRealAxisIndex,
      rightAxisCount,
      yAxis,
    };
  }

  private buildSeries(
    visibleConfigs: SeriesConfig[],
    logicalToRealAxisIndex: Map<number, number>,
  ) {
    return visibleConfigs.map((cfg) => {
      const logicalAxisIdx = getAxisForField(cfg.id);
      const realAxisIdx = logicalToRealAxisIndex.get(logicalAxisIdx) ?? 0;

      return {
        id: cfg.id,
        name: cfg.name,
        type: "line",
        smooth: false,
        showSymbol: false,
        yAxisIndex: realAxisIdx,
        lineStyle: {
          width: 2,
          color: cfg.color,
        },
        itemStyle: {
          color: cfg.color,
        },
        emphasis: {
          focus: "series",
        },
        data: this.seriesData.get(cfg.id) || [],
        sampling: "lttb",
        large: true,
        largeThreshold: 2000,
        progressive: 2000,
        progressiveThreshold: 10000,
        animation: false,
      };
    });
  }

  public updateData(points: Partial<Record<string, DataPoint>>) {
    let hasUpdates = false;

    for (const [seriesId, point] of Object.entries(points)) {
      const data = this.seriesData.get(seriesId);
      if (!data || !point) continue;

      data.push(point);

      // Вместо shift() — пакетная обрезка
      if (data.length > this.MAX_POINTS) {
        data.splice(0, data.length - this.MAX_POINTS);
      }

      hasUpdates = true;
    }

    if (!hasUpdates || !this.chart) return;

    const visibleConfigs = this.getVisibleSeriesConfigs();
    if (visibleConfigs.length === 0) return;

    // Обновляем только данные серий, без полной пересборки осей / grid / legend
    this.chart.setOption({
      series: visibleConfigs.map((cfg) => ({
        id: cfg.id,
        data: this.seriesData.get(cfg.id) || [],
      })),
    });
  }

  public loadBulkData(bulkData: Record<string, DataPoint[]>) {
    let hasUpdates = false;

    for (const [seriesId, points] of Object.entries(bulkData)) {
      const data = this.seriesData.get(seriesId);
      if (!data) continue;

      data.length = 0;
      data.push(...points);

      if (data.length > this.MAX_POINTS) {
        data.splice(0, data.length - this.MAX_POINTS);
      }

      hasUpdates = true;
    }

    if (hasUpdates && this.chart) {
      this.chart.setOption(this.buildOption(), { notMerge: true });

      const visibleConfigs = this.getVisibleSeriesConfigs();
      if (visibleConfigs.length > 0) {
        this.chart.dispatchAction({
          type: "dataZoom",
          start: 95,
          end: 100,
        });
      }
    }
  }

  public toggleSeriesVisibility(
    visibleIds: string[],
    seriesColors?: Record<string, string>,
  ) {
    this.seriesConfig.forEach((cfg) => {
      cfg.visible = visibleIds.includes(cfg.id);

      const color = seriesColors?.[cfg.id];
      if (color) {
        cfg.color = color;
      }
    });

    // Здесь оси могут реально измениться, поэтому rebuild оправдан
    this.rebuildChart();
  }

  public clear() {
    this.seriesData.forEach((_, k) => this.seriesData.set(k, []));

    if (!this.chart) return;

    this.chart.clear();
    this.chart.setOption(this.buildOption(), { notMerge: true });
  }

  private observeResize() {
    if (typeof ResizeObserver !== "undefined") {
      this.resizeObserver = new ResizeObserver(() => this.chart?.resize());
      this.resizeObserver.observe(this.container);
    } else {
      window.addEventListener("resize", () => this.chart?.resize());
    }
  }

  public destroy() {
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
      this.resizeObserver = null;
    }

    this.chart?.dispose();
    this.chart = null;
  }

  public zoomToEnd() {
    this.chart?.setOption({
      dataZoom: [
        { id: "insideZoom", start: 95, end: 100 },
        { id: "sliderZoom", start: 95, end: 100 },
      ],
    });
  }

  private getVisibleSeriesConfigs(): SeriesConfig[] {
    return this.seriesConfig.filter((cfg) => cfg.visible === true);
  }

  private rebuildChart() {
    if (!this.chart) return;
    this.chart.setOption(this.buildOption(), { notMerge: true });
  }

  public setSeriesColor(seriesId: string, color: string) {
    const cfg = this.seriesConfig.find((s) => s.id === seriesId);
    if (cfg) {
      cfg.color = color;
    }

    if (!this.chart) {
      console.warn("Chart is not initialized");
      return;
    }

    const option = this.chart.getOption() as any;

    const updatedSeries = option.series?.map((s: any) => {
      if (s.id !== seriesId) return s;

      return {
        ...s,
        lineStyle: {
          ...s.lineStyle,
          color,
        },
        itemStyle: {
          ...s.itemStyle,
          color,
        },
      };
    });

    if (!updatedSeries) {
      console.warn("Series not found in chart option");
      return;
    }

    this.chart.setOption({ series: updatedSeries });
  }
}
