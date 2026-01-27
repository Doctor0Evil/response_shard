#!/usr/bin/env node
import { MATERIALS } from "../domain/materials";
import { PHOENIX_LCA_CONTEXT } from "../config/phoenix.config";
import { calculateEcoScore } from "../services/ecoScoreService";
import { estimateLocalDecay } from "../services/decayService";

function printUsage() {
  console.log("PHA–Bagasse Eco CLI");
  console.log("Usage:");
  console.log("  eco-cli scenario phoenix-bagasse");
  console.log("  eco-cli scenario phoenix-pha-standalone");
  console.log("  eco-cli scenario phoenix-pha-integrated");
}

function runScenarioPhoenixBagasse() {
  const trayMassKg = 0.02;
  const config = {
    materialId: "BAGASSE" as const,
    trayMassKg,
    integratedSystem: false
  };
  const eco = calculateEcoScore({
    config,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.01
  });
  const decay = estimateLocalDecay(
    MATERIALS.BAGASSE,
    "home_compost",
    90
  );

  console.log("Phoenix bagasse tray scenario:");
  console.log(JSON.stringify({ eco, decay }, null, 2));
}

function runScenarioPhoenixPhaStandalone() {
  const trayMassKg = 0.02;
  const config = {
    materialId: "PHA" as const,
    trayMassKg,
    integratedSystem: false
  };
  const eco = calculateEcoScore({
    config,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.015
  });
  const decay = estimateLocalDecay(MATERIALS.PHA, "soil", 365);

  console.log("Phoenix PHA tray scenario (stand-alone PHA line):");
  console.log(JSON.stringify({ eco, decay }, null, 2));
}

function runScenarioPhoenixPhaIntegrated() {
  const trayMassKg = 0.02;
  const config = {
    materialId: "PHA" as const,
    trayMassKg,
    integratedSystem: true
  };
  const eco = calculateEcoScore({
    config,
    ctx: PHOENIX_LCA_CONTEXT,
    landfillFractionBaseline: 0.9,
    compostFractionScenario: 1.0,
    gridKWhPerTray: 0.008
  });
  const decay = estimateLocalDecay(MATERIALS.PHA, "soil", 365);

  console.log("Phoenix PHA tray scenario (integrated waste–water–energy system):");
  console.log(JSON.stringify({ eco, decay }, null, 2));
}

function main() {
  const [, , cmd, arg1] = process.argv;
  if (cmd !== "scenario") {
    printUsage();
    process.exit(1);
  }
  switch (arg1) {
    case "phoenix-bagasse":
      runScenarioPhoenixBagasse();
      break;
    case "phoenix-pha-standalone":
      runScenarioPhoenixPhaStandalone();
      break;
    case "phoenix-pha-integrated":
      runScenarioPhoenixPhaIntegrated();
      break;
    default:
      printUsage();
      process.exit(1);
  }
}

main();
