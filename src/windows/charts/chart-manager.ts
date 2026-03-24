import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import { GridComponent, TooltipComponent, LegendComponent, DataZoomComponent, TitleComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

echarts.use([LineChart, GridComponent, TooltipComponent, LegendComponent, DataZoomComponent, TitleComponent, CanvasRenderer]);

export type DataPoint = [number, number];

export interface SeriesConfig {
  id: string;
  name: string;
  color?: string;
  visible?: boolean;
}

export class ChartManager {
  private chart: echarts.ECharts | null = null;
  private container: HTMLElement;
  private resizeObserver: ResizeObserver | null = null;
  private seriesData: Map<string, DataPoint[]> = new Map();
  private seriesConfig: SeriesConfig[] = [];
  private readonly MAX_POINTS = 1_000_000;

  constructor(containerId: string, seriesConfigs: SeriesConfig[]) {
    const el = document.getElementById(containerId);
    if (!el) throw new Error(`Элемент "${containerId}" не найден`);
    this.container = el;
    this.seriesConfig = seriesConfigs;
    seriesConfigs.forEach(cfg => this.seriesData.set(cfg.id, []));
    this.init();
  }

  private init() {
    this.chart = echarts.init(this.container, null, { renderer: 'canvas' });
    this.chart.setOption(this.buildOption());
    this.observeResize();
  }

  private buildOption() {
    return {
      tooltip: { trigger: 'axis', axisPointer: { type: 'cross' } },
      legend: { data: this.seriesConfig.map(cfg => cfg.name), type: 'scroll', bottom: 0 },
      grid: { left: '3%', right: '4%', top: '40px', bottom: '40px', containLabel: true },
      xAxis: { type: 'time', boundaryGap: false, axisLabel: { formatter: (v: number) => new Date(v).toLocaleTimeString() } },
      yAxis: { type: 'value', scale: true, splitLine: { lineStyle: { type: 'dashed' } } },
      dataZoom: [{ type: 'inside', start: 0, end: 100 }, { type: 'slider', start: 0, end: 100, height: 20, bottom: 20 }],
      series: this.seriesConfig.map(cfg => ({
        name: cfg.name, type: 'line', smooth: false, showSymbol: false, 
        lineStyle: { width: 2, color: cfg.color },
        data: this.seriesData.get(cfg.id) || [],
        sampling: 'lttb'
      }))
    };
  }

  public updateData(points: Partial<Record<string, DataPoint>>) {
    if (!this.chart) return;
    let hasUpdates = false;
    for (const [seriesId, point] of Object.entries(points)) {
      const data = this.seriesData.get(seriesId);
      if (!data || !point) continue;
      data.push(point);
      if (data.length > this.MAX_POINTS) data.shift();
      hasUpdates = true;
    }
    if (hasUpdates) {
      this.chart.setOption({
        series: this.seriesConfig.map(cfg => ({ name: cfg.name, data: this.seriesData.get(cfg.id) }))
      });
    }
  }

  public clear() {
    this.seriesData.forEach((_, k) => this.seriesData.set(k, []));
    this.chart?.clear();
    this.chart?.setOption(this.buildOption());
  }

  private observeResize() {
    if (typeof ResizeObserver !== 'undefined') {
      this.resizeObserver = new ResizeObserver(() => this.chart?.resize());
      this.resizeObserver.observe(this.container);
    } else {
      window.addEventListener('resize', () => this.chart?.resize());
    }
  }

  public destroy() {
    if (this.resizeObserver) this.resizeObserver.disconnect();
    this.chart?.dispose();
    this.chart = null;
  }

  public zoomToEnd() {
  this.chart?.setOption({
    dataZoom: [{ start: 95, end: 100 }, { start: 95, end: 100 }]
    });
  }

  // 👇 Добавить в class ChartManager, после updateData:

/** 
 * Загрузить данные оптом. 
 * bulkData: { seriesId: [ [ts, val], [ts, val], ... ] }
 */
public loadBulkData(bulkData: Record<string, DataPoint[]>) {
  let hasUpdates = false;
  
  for (const [seriesId, points] of Object.entries(bulkData)) {
    const data = this.seriesData.get(seriesId);
    if (!data) continue;
    
    // Чистим старое и заливаем новое
    data.length = 0;
    data.push(...points);
    
    // Обрезаем, если вдруг больше лимита (на всякий)
    if (data.length > this.MAX_POINTS) {
      data.splice(0, data.length - this.MAX_POINTS);
    }
    hasUpdates = true;
  }

  if (hasUpdates && this.chart) {
    this.chart.setOption({
      series: this.seriesConfig.map(cfg => ({
        name: cfg.name,
        data: this.seriesData.get(cfg.id)
      }))
    });
    // 👇 Зумим в конец, чтобы видеть последние данные
    this.chart.dispatchAction({
      type: 'dataZoom',
      start: 95,
      end: 100
    });
  }
}
}