use crate::grammar::{RiskCoord, CorridorBands, Residual, CorridorDecision};
use crate::contracts::{corridor_present, safe_step};

/// Phoenix-class MAR SAT shard header (simplified)
#[derive(Clone, Debug)]
pub struct PhoenixMarSatShard {
    pub shard_id: String,
    pub moduletype: String,      // e.g. "PhoenixMARCell.v1"
    pub region: String,          // e.g. "Phoenix-AZ"
    pub sim_or_live: String,     // "sim" | "live"
    pub timestamp_utc: String,   // ISO-8601
    pub did_signature: String,   // Bostrom DID / CHAT linked
    /// Corridor table: PFAS, pharma, SAT, temp, surcharge
    pub corridors: Vec<CorridorBands>,
    /// Current normalized risk coordinates and residual
    pub risk_state: Residual,
    /// K/E/R research-only scores for this MAR cell
    pub knowledge_factor: f64,   // e.g. 0.93
    pub eco_impact_value: f64,   // e.g. 0.92
    pub risk_of_harm: f64,       // e.g. 0.14
}

/// Convenience enum for the MAR SAT corridor IDs.
pub enum MarVarId {
    RSat,        // overall SAT health / loading
    RPfas,       // PFAS (PFBS etc.) breakthrough
    RPharma,     // pharmaceuticals breakthrough
    RTemp,       // recharge / aquifer temperature shift
    RSurcharge,  // hydraulic surcharge risk
}

impl MarVarId {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarVarId::RSat       => "rSAT",
            MarVarId::RPfas      => "rPFAS",
            MarVarId::RPharma    => "rPHARMA",
            MarVarId::RTemp      => "rTEMP",
            MarVarId::RSurcharge => "rSURCH",
        }
    }
}

/// Build MAR SAT corridor rows (bands are placeholders; pilot data should tighten).
pub fn default_mar_sat_corridors() -> Vec<CorridorBands> {
    vec![
        CorridorBands::new(
            MarVarId::RSat.as_str(),
            "dimensionless",
            0.30, 0.60, 0.95,   // safe, gold, hard
            0.30,               // weight in V_t
            0,                  // Lyapunov channel index
            true,               // mandatory
        ),
        CorridorBands::new(
            MarVarId::RPfas.as_str(),
            "dimensionless",
            0.20, 0.50, 0.95,
            0.25,
            0,
            true,
        ),
        CorridorBands::new(
            MarVarId::RPharma.as_str(),
            "dimensionless",
            0.20, 0.50, 0.95,
            0.15,
            0,
            true,
        ),
        CorridorBands::new(
            MarVarId::RTemp.as_str(),
            "dimensionless",
            0.20, 0.60, 0.98,   // narrow bands for thermal/geochemical drift
            0.15,
            1,                  // separate Lyapunov channel if desired
            true,
        ),
        CorridorBands::new(
            MarVarId::RSurcharge.as_str(),
            "dimensionless",
            0.10, 0.40, 0.90,   // conservative surcharge envelope
            0.15,
            2,
            true,
        ),
    ]
}

/// Normalize raw MAR metrics into RiskCoord values using shared kernels.
/// (Implement using the universal normalization functions already in your grammar crate.)
pub fn compute_mar_risk_state(
    bands: &[CorridorBands],
    r_sat_raw: f64,
    r_pfas_raw: f64,
    r_pharma_raw: f64,
    r_temp_raw: f64,
    r_surcharge_raw: f64,
) -> Residual {
    let mut coords: Vec<RiskCoord> = Vec::with_capacity(5);

    for (var_id, raw) in [
        (MarVarId::RSat.as_str(),       r_sat_raw),
        (MarVarId::RPfas.as_str(),      r_pfas_raw),
        (MarVarId::RPharma.as_str(),    r_pharma_raw),
        (MarVarId::RTemp.as_str(),      r_temp_raw),
        (MarVarId::RSurcharge.as_str(), r_surcharge_raw),
    ] {
        let band = bands
            .iter()
            .find(|b| b.varid == var_id)
            .expect("missing MAR corridor band");
        coords.push(RiskCoord::from_raw(raw, band));
    }

    Residual::from_coords(coords)
}

/// CI-time invariant: no corridor, no build for Phoenix MAR SAT cells.
pub fn invariant_mar_sat_corridor_complete(shard: &PhoenixMarSatShard) -> bool {
    corridor_present(&shard.corridors)
}

/// Runtime invariant wrapper: MAR-safe step with SAT / PFAS / pharma / temp / surcharge.
pub fn mar_sat_safe_step(
    prev: &Residual,
    next: &Residual,
    safe_interior_eps: f64,
) -> CorridorDecision {
    // Delegates core checks to the shared safestep contract:
    // 1. All rx < 1.0
    // 2. V_t+1 <= V_t outside the safe interior.
    safe_step(prev, next, safe_interior_eps)
}

/// Example: build a research-only Phoenix-class MAR shard from live telemetry.
pub fn build_live_mar_shard_from_telemetry(
    shard_id: String,
    region: String,
    did_signature: String,
    r_sat_raw: f64,
    r_pfas_raw: f64,
    r_pharma_raw: f64,
    r_temp_raw: f64,
    r_surcharge_raw: f64,
) -> PhoenixMarSatShard {
    let corridors = default_mar_sat_corridors();
    assert!(corridor_present(&corridors), "no corridor, no build");

    let risk_state =
        compute_mar_risk_state(&corridors, r_sat_raw, r_pfas_raw, r_pharma_raw, r_temp_raw, r_surcharge_raw);

    PhoenixMarSatShard {
        shard_id,
        moduletype: "PhoenixMARCell.v1".to_string(),
        region,
        sim_or_live: "live".to_string(),
        timestamp_utc: chrono::Utc::now().to_rfc3339(),
        did_signature,
        corridors,
        risk_state,
        // Research-only K/E/R; production promotion is gated elsewhere.
        knowledge_factor: 0.93,
        eco_impact_value: 0.92,
        risk_of_harm: 0.14,
    }
}
