import {
  LineTelemetrySnapshot,
  TelemetryShard
} from "../domain/telemetry";

export function snapshotToShard(
  snapshot: LineTelemetrySnapshot,
  shardId: string
): TelemetryShard {
  const { energyKWh, waterLiters, materialKg, traysProduced } =
    snapshot;
  if (traysProduced <= 0) {
    throw new Error("traysProduced must be positive");
  }
  return {
    shardId,
    lineId: snapshot.lineId,
    timestamp: snapshot.timestamp,
    payload: {
      energyKWhPerTray: energyKWh / traysProduced,
      waterLitersPerTray: waterLiters / traysProduced,
      materialKgPerTray: materialKg / traysProduced
    }
  };
}
