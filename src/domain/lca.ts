import { MaterialId } from "./materials";

export interface LCAContext {
  gridEmissionFactorKgPerKWh: number;
  landfillMethaneKgCO2ePerKgOrganic: number;
  compostMethaneKgCO2ePerKgOrganic: number;
  fossilPlasticGwpKgPerKg: number;
}

export interface TrayConfiguration {
  materialId: MaterialId;
  trayMassKg: number;
  integratedSystem: boolean;
}

export interface DisplacementScore {
  kgPlasticAvoided: number;
  kgCO2eAvoidedFromPlastic: number;
}

export interface MethaneAvoidanceScore {
  kgCH4EqAvoidedInCO2e: number;
}

export interface EcoScore {
  climateScore: number;
  fossilScore: number;
  microplasticScore: number;
  totalScore: number;
}

export function computeDisplacement(
  config: TrayConfiguration,
  ctx: LCAContext
): DisplacementScore {
  const kgPlasticAvoided = config.trayMassKg;
  const kgCO2eAvoidedFromPlastic =
    kgPlasticAvoided * ctx.fossilPlasticGwpKgPerKg;
  return { kgPlasticAvoided, kgCO2eAvoidedFromPlastic };
}

export function computeMethaneAvoidance(
  config: TrayConfiguration,
  ctx: LCAContext,
  landfillFractionBaseline: number,
  compostFractionScenario: number
): MethaneAvoidanceScore {
  const organicKg = config.trayMassKg;
  const baselineMethane =
    organicKg *
    landfillFractionBaseline *
    ctx.landfillMethaneKgCO2ePerKgOrganic;
  const scenarioMethane =
    organicKg *
    compostFractionScenario *
    ctx.compostMethaneKgCO2ePerKgOrganic;
  const kgCH4EqAvoidedInCO2e = baselineMethane - scenarioMethane;
  return { kgCH4EqAvoidedInCO2e };
}

export function computeEcoScore(
  displacement: DisplacementScore,
  methane: MethaneAvoidanceScore,
  gridKWhPerTray: number,
  ctx: LCAContext,
  materialId: MaterialId,
  integratedSystem: boolean
): EcoScore {
  const energyEmissions = gridKWhPerTray * ctx.gridEmissionFactorKgPerKWh;
  const climateBenefit =
    displacement.kgCO2eAvoidedFromPlastic +
    methane.kgCH4EqAvoidedInCO2e -
    energyEmissions;

  const fossilBenefitBase = displacement.kgPlasticAvoided;
  const fossilMultiplier = integratedSystem && materialId === "PHA" ? 1.5 : 1.0;
  const fossilScore = fossilBenefitBase * fossilMultiplier;

  const microplasticScore = materialId === "PHA" ? 1.0 : 0.9;

  const climateScore = climateBenefit;
  const totalScore =
    0.6 * climateScore + 0.3 * fossilScore + 0.1 * microplasticScore;

  return {
    climateScore,
    fossilScore,
    microplasticScore,
    totalScore
  };
}
