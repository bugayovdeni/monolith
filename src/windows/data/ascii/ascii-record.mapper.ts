type DataPoint = [number, number];

export function mapAsciiRecord(
  raw: Record<string, number>,
  now: number,
): Partial<Record<string, DataPoint>> {
  return {
    recirc_density: [now, raw["Recirc Density"]],
    downhole_density: [now, raw["Downhole Density"]],
    mix_water_rate: [now, raw["Mix Water Rate"]],
    combo_rate: [now, raw["Combo Rate"]],
    ps_pressure: [now, raw["PS Pressure"]],
    ds_pressure: [now, raw["DS Pressure"]],
    mix_wtr_stg_ttl: [now, raw["Mix Wtr Stg Ttl"]],
    mix_wtr_job_ttl: [now, raw["Mix Wtr Job Ttl"]],
    combo_pump_stg_ttl: [now, raw["Combo Pump Stg Ttl"]],
    combo_pump_job_ttl: [now, raw["Combo Pump Job Ttl"]],
    cement_vlv_percent: [now, raw["Cement Vlv Percent"]],
    wtr_vlv_percent: [now, raw["Wtr Vlv Percent"]],
    ps_rate: [now, raw["PS Rate"]],
    ds_rate: [now, raw["DS Rate"]],
  };
}
