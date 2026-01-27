import { computeDisplacementMetrics } from "../src/services/displacementService";
import { PHOENIX_LCA_CONTEXT } from "../src/config/phoenix.config";

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message);
  }
}

(function runDisplacementTests() {
  const metrics = computeDisplacementMetrics({
    config: {
      materialId: "BAGASSE",
      trayMassKg: 0.02,
      integratedSystem: false
    },
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0
  });

  assert(
    metrics.displacement.kgPlasticAvoided === 0.02,
    "Plastic avoided should equal tray mass"
  );
  assert(
    metrics.methane.kgCH4EqAvoidedInCO2e > 0,
    "Methane avoidance should be positive when moving from landfill to compost"
  );

  console.log("displacementService tests passed");
})();
