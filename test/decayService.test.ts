import { MATERIALS } from "../src/domain/materials";
import { estimateLocalDecay } from "../src/services/decayService";

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message);
  }
}

(function runDecayTests() {
  const bagasse90 = estimateLocalDecay(
    MATERIALS.BAGASSE,
    "home_compost",
    90
  );
  assert(
    bagasse90.degradedFraction > 0.8,
    "Bagasse should be largely degraded by 90 days in home compost"
  );

  const pha365 = estimateLocalDecay(MATERIALS.PHA, "soil", 365);
  assert(
    pha365.degradedFraction > 0.7,
    "PHA should be mostly degraded by 365 days in soil"
  );

  console.log("decayService tests passed");
})();
