export type AxisMode = "auto_expand" | "manual";

export type ManualAxisRange = {
  min: number;
  max: number;
};

export type ManualAxisRanges = Record<number, ManualAxisRange>;

export type AxisManualSettings = {
  axisIndex: number;
  min: number;
  max: number;
};
