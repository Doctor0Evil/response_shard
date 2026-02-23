// BeeCorridorShard2026v1 – ecosafety shard for pollinator corridors.
// Compatible with existing RiskCoord, CorridorBands, Residual, KER, and DID meta.

use crate::contracts::{RiskCoord, CorridorBands, Residual};
use crate::ker::KerTriple; // { knowledge_factor, eco_impact_value, risk_of_harm }
use crate::did::DidSignature;

// High-level type for corridor-scale bee habitat band.
#[derive(Clone, Debug)]
pub struct BeeCorridorShard2026v1 {
    pub header: BeeCorridorHeader,
    pub corridors: Vec<CorridorBands>,   // one row per varid (width, nectar, etc.)
    pub risk_state: BeeCorridorRiskState,
    pub residual: Residual,              // V_t for this corridor segment / scenario
    pub ker: KerTriple,                  // K/E/R scores, same triad as other shards
}

#[derive(Clone, Debug)]
pub struct BeeCorridorHeader {
    pub shard_id: String,               // e.g. "BeeCorridorShard2026v1-SEG-000123"
    pub region: String,                 // e.g. "Phoenix-AZ-US"
    pub segment_id: String,             // local corridor segment ID
    pub sim_or_live: String,            // "sim" | "live"
    pub timestamp_utc: String,          // ISO 8601
    pub authordid_primary: String,      // e.g. "bostrom18sd2u..."
    pub authordid_alt: Option<String>,
    pub evidence_hex: String,           // Merkle/tx anchor for this shard
    pub did_signature: DidSignature,    // hex-encoded signature over shard contents
}

// Normalized risk coordinates for the bee corridor band.
#[derive(Clone, Debug, Default)]
pub struct BeeCorridorRiskState {
    // Geometry & continuity
    pub r_width_m: RiskCoord,              // corridor physical width vs bands
    pub r_continuity_index: RiskCoord,     // 0..1 connectivity metric
    pub r_flower_density: RiskCoord,       // flowers per m^2 vs target

    // Nectar / forage quality
    pub r_nectar_rich_fraction: RiskCoord, // fraction of area with high-nectar flora
    pub r_bloom_season_coverage: RiskCoord,// fraction of weeks with adequate bloom

    // Pesticides / toxics
    pub r_pesticide_load: RiskCoord,       // aggregate pesticide index vs safe bands
    pub r_drift_risk: RiskCoord,          // adjacency to sprayed fields / drift index

    // Thermal / microclimate
    pub r_thermal_refuge: RiskCoord,      // shaded / cool micro-habitat fraction
    pub r_drought_stress: RiskCoord,      // water stress index for flora

    // Collision / mortality risks
    pub r_roadkill_risk: RiskCoord,       // proximity to roads / traffic corridors
    pub r_light_pollution: RiskCoord,     // night-time light vs safe bands
    pub r_air_toxics: RiskCoord,          // NOx/O3/PM band for flight height

    // Aggregated “out-of-band” indicator
    pub r_out_of_band: RiskCoord,         // max over critical coordinates
}

// Canonical varids for CorridorBands rows, so the residual engine can
// reconstruct V_t from bands + BeeCorridorRiskState exactly as in other nodes.
// These strings become the qpudatashard varid IDs.
pub const BEE_VARID_WIDTH_M: &str               = "bee.width_m";
pub const BEE_VARID_CONTINUITY_INDEX: &str      = "bee.continuity_index";
pub const BEE_VARID_FLOWER_DENSITY: &str        = "bee.flower_density";
pub const BEE_VARID_NECTAR_RICH_FRACTION: &str  = "bee.nectar_rich_fraction";
pub const BEE_VARID_BLOOM_SEASON_COVERAGE: &str = "bee.bloom_season_coverage";
pub const BEE_VARID_PESTICIDE_LOAD: &str        = "bee.pesticide_load";
pub const BEE_VARID_DRIFT_RISK: &str            = "bee.drift_risk";
pub const BEE_VARID_THERMAL_REFUGE: &str        = "bee.thermal_refuge";
pub const BEE_VARID_DROUGHT_STRESS: &str        = "bee.drought_stress";
pub const BEE_VARID_ROADKILL_RISK: &str         = "bee.roadkill_risk";
pub const BEE_VARID_LIGHT_POLLUTION: &str       = "bee.light_pollution";
pub const BEE_VARID_AIR_TOXICS: &str            = "bee.air_toxics";
pub const BEE_VARID_OUT_OF_BAND: &str           = "bee.out_of_band";

// Minimal helper to construct CorridorBands rows for BeeCorridorBand variables.
// safe/gold/hard are in physical/native units (m, fractions, indices), but
// normalization kernels will map them into r_x in [0,1] as usual.
pub fn bee_corridor_band(
    varid: &str,
    units: &str,
    safe: f64,
    gold: f64,
    hard: f64,
    weight_w: f64,
    lyap_channel: u16,
    mandatory: bool,
) -> CorridorBands {
    CorridorBands {
        varid: varid.to_string(),
        units: units.to_string(),
        safe,
        gold,
        hard,
        weight_w,
        lyap_channel,
        mandatory,
    }
}

// Example: canonical CorridorBands set for a Phoenix-class urban bee corridor
// segment, ready to be embedded in a qpudatashard and used by the shared
// residual / safestep contracts.
pub fn default_bee_corridor_bands_phoenix() -> Vec<CorridorBands> {
    vec![
        // Geometry & continuity – hard minimum width and connectivity
        bee_corridor_band(BEE_VARID_WIDTH_M, "m", 10.0, 5.0, 2.0, 0.20, 1, true),
        bee_corridor_band(BEE_VARID_CONTINUITY_INDEX, "0-1", 0.85, 0.70, 0.50, 0.20, 1, true),
        bee_corridor_band(BEE_VARID_FLOWER_DENSITY, "flowers/m^2", 8.0, 5.0, 2.0, 0.10, 1, true),

        // Nectar / forage
        bee_corridor_band(BEE_VARID_NECTAR_RICH_FRACTION, "0-1", 0.70, 0.50, 0.30, 0.10, 2, true),
        bee_corridor_band(BEE_VARID_BLOOM_SEASON_COVERAGE, "0-1", 0.85, 0.65, 0.40, 0.10, 2, true),

        // Pesticides
        bee_corridor_band(BEE_VARID_PESTICIDE_LOAD, "index", 0.20, 0.40, 0.60, 0.10, 3, true),
        bee_corridor_band(BEE_VARID_DRIFT_RISK, "0-1", 0.20, 0.40, 0.60, 0.05, 3, true),

        // Thermal / drought
        bee_corridor_band(BEE_VARID_THERMAL_REFUGE, "0-1", 0.60, 0.40, 0.20, 0.05, 4, true),
        bee_corridor_band(BEE_VARID_DROUGHT_STRESS, "0-1", 0.20, 0.40, 0.60, 0.05, 4, true),

        // Mortality & disturbance
        bee_corridor_band(BEE_VARID_ROADKILL_RISK, "0-1", 0.20, 0.40, 0.60, 0.025, 5, true),
        bee_corridor_band(BEE_VARID_LIGHT_POLLUTION, "0-1", 0.20, 0.40, 0.60, 0.025, 5, true),
        bee_corridor_band(BEE_VARID_AIR_TOXICS, "0-1", 0.20, 0.40, 0.60, 0.05, 5, true),

        // Aggregated out-of-band, used in r_out_of_band and V_t
        bee_corridor_band(BEE_VARID_OUT_OF_BAND, "0-1", 0.30, 0.50, 0.70, 0.10, 6, true),
    ]
}
