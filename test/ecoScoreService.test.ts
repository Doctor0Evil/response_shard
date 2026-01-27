import { calculateEcoScore } from "../src/services/ecoScoreService";
import { PHOENIX_LCA_CONTEXT } from "../src/config/phoenix.config";

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message);
  }
}

(function runEcoScoreTests() {
  const bagasseConfig = {
    materialId: "BAGASSE" as const,
    trayMassKg: 0.02,
    integratedSystem: false
  };

  const bagasseEco = calculateEcoScore({
    config: bagasseConfig,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.01
  });

  assert(
    bagasseEco.ecoScore.totalScore > 0,
    "Bagasse eco-score should be positive"
  );

  const phaStandaloneConfig = {
    materialId: "PHA" as const,
    trayMassKg: 0.02,
    integratedSystem: false
  };

  const phaStandaloneEco = calculateEcoScore({
    config: phaStandaloneConfig,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.015
  });

  const phaIntegratedConfig = {
    materialId: "PHA" as const,
    trayMassKg: 0.02,
    integratedSystem: true
  };

  const phaIntegratedEco = calculateEcoScore({
    config: phaIntegratedConfig,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.008
  });

  assert(
    phaIntegratedEco.ecoScore.totalScore >= phaStandaloneEco.ecoScore.totalScore,
    "Integrated PHA should score at least as well as stand-alone PHA"
  );

  console.log("ecoScoreService tests passed");
})();
