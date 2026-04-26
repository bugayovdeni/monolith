import type {
  AxisMode,
  ManualAxisRanges,
  AxisManualSettings,
} from "./axis-types";

export class AxisState {
  private mode: AxisMode = "auto_expand";
  private manualRanges: ManualAxisRanges = {};

  getMode(): AxisMode {
    return this.mode;
  }

  setMode(mode: AxisMode) {
    this.mode = mode;
  }

  isAutoExpand(): boolean {
    return this.mode === "auto_expand";
  }

  isManual(): boolean {
    return this.mode === "manual";
  }

  setManualRange(axisIndex: number, min: number, max: number) {
    this.manualRanges[axisIndex] = { min, max };
    this.mode = "manual";
  }

  getManualRange(axisIndex: number) {
    return this.manualRanges[axisIndex];
  }

  clearManualRanges() {
    this.manualRanges = {};
  }

  setManualSettings(settings: AxisManualSettings) {
    this.manualRanges[settings.axisIndex] = {
      min: settings.min,
      max: settings.max,
    };

    this.mode = "manual";
  }
}
