import {
  LCAContext,
  TrayConfiguration,
  DisplacementScore,
  MethaneAvoidanceScore,
  computeDisplacement,
  computeMethaneAvoidance
} from "../domain/lca";

export interface DisplacementInputs {
  config: TrayConfiguration;
  ctx: LCAContext;
  landfillFractionBaseline: number;
  compostFractionScenario: number;
}

export interface DisplacementMetrics {
  displacement: DisplacementScore;
  methane: MethaneAvoidanceScore;
}

export function computeDisplacementMetrics(
  inputs: DisplacementInputs
): DisplacementMetrics {
  const { config, ctx, landfillFractionBaseline, compostFractionScenario } =
    inputs;

  const displacement = computeDisplacement(config, ctx);
  const methane = computeMethaneAvoidance(
    config,
    ctx,
    landfillFractionBaseline,
    compostFractionScenario
  );

  return { displacement, methane };
}
