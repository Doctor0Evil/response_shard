export type Environment =
  | "home_compost"
  | "industrial_compost"
  | "soil"
  | "marine"
  | "landfill";

export interface DecayParams {
  halfLifeDays: number;
  environment: Environment;
}

export interface DecayResult {
  remainingFraction: number;
  degradedFraction: number;
}

export function halfLifeFromWindow(
  minDays: number,
  maxDays: number
): number {
  const target = (minDays + maxDays) / 2;
  return target / Math.log(2);
}

export function computeDecay(
  params: DecayParams,
  days: number
): DecayResult {
  const k = Math.log(2) / params.halfLifeDays;
  const remainingFraction = Math.exp(-k * days);
  return {
    remainingFraction,
    degradedFraction: 1 - remainingFraction
  };
}
