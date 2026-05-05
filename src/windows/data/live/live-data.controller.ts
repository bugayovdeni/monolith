import { listen } from "@tauri-apps/api/event";
import { mapAsciiRecord } from "../ascii/ascii-record.mapper";
import { ChartManager } from "../../charts/chart-manager";

export function initLiveData(
  chart: ChartManager,
  getDataMode: () => "idle" | "live" | "archive",
) {
  listen("ascii-record", (event) => {
    if (getDataMode() !== "live") return;

    const raw = event.payload as Record<string, number>;
    const now = Date.now();

    chart.updateData(mapAsciiRecord(raw, now));

    console.log("[FRONT] chart updated from ascii-record");
  });
}
