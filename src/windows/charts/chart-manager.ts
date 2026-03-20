import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import { GridComponent, TooltipComponent, LegendComponent, DataZoomComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

// Регистрируем только нужные компоненты. Не тащим всю библиотеку, следим за памятью.
echarts.use([
  LineChart,
  GridComponent,
  TooltipComponent,
  LegendComponent,
  DataZoomComponent,
  CanvasRenderer
]);

export class ChartManager {
    private chart: echarts.ECharts | null = null;
    private container: HTMLElement;
    private resizeObserver: ResizeObserver | null = null;

    private data1: [number, number][] = [];
    private data2: [number, number][] = [];

  constructor(containerId: string) {
    const el = document.getElementById(containerId);
    if (!el) {
      throw new Error(`Элемент с id "${containerId}" не найден.`);
    }
    this.container = el;
    this.init();
  }

  private init() {
    this.chart = echarts.init(this.container);
    this.chart.setOption(this.getInitialOption());
    this.observeResize();
  }

  private getInitialOption() {
    return {
      tooltip: { trigger: 'axis' },
      legend: { data: ['Сигнал 1', 'Сигнал 2'] },
      grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
      xAxis: { type: 'time', boundaryGap: false },
      yAxis: { type: 'value' },
      dataZoom: [{ type: 'inside' }, { type: 'slider' }],
      series: [
        { 
          name: 'Сигнал 1', 
          type: 'line', 
          smooth: true, 
          showSymbol: false,
          data: [] 
        },
        { 
          name: 'Сигнал 2', 
          type: 'line', 
          smooth: true, 
          showSymbol: false,
          data: [] 
        }
      ]
    };
  }

  // Метод для обновления данных. Вызываем это из цикла main.ts.
public updateData(point1: [number, number], point2: [number, number]) {
  if (!this.chart) return;

  this.data1.push(point1);
  this.data2.push(point2);

  const MAX_POINTS = 100;

  if (this.data1.length > MAX_POINTS) this.data1.shift();
  if (this.data2.length > MAX_POINTS) this.data2.shift();

  this.chart.setOption({
    series: [
      { data: this.data1 },
      { data: this.data2 }
    ]
  });
}

  private observeResize() {
    if (typeof ResizeObserver !== 'undefined') {
      this.resizeObserver = new ResizeObserver(() => {
        this.chart?.resize();
      });
      this.resizeObserver.observe(this.container);
    } else {
      window.addEventListener('resize', () => this.chart?.resize());
    }
  }

  // Обязательно диспозим
  public destroy() {
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
    }
    this.chart?.dispose();
    this.chart = null;
  }
}