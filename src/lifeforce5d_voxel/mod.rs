#![forbid(unsafe_code)]

use std::time::Duration;

/// Dimensionless risk coordinate r_x ∈ [0,1] with corridor bands.
#[derive(Clone, Debug)]
pub struct RiskCoord {
    pub var_id:      String,   // e.g. "r_tdi", "r_bee", "r_mbi"
    pub value:       f64,      // 0 = fully safe, 1 = hard limit
    pub safe:        f64,
    pub gold:        f64,
    pub hard:        f64,
    pub weight:      f64,      // contribution into V_t
    pub lyap_channel: u16,     // residual channel
}

/// Aggregated Lyapunov-style residual V_t over all coordinates.
#[derive(Clone, Debug)]
pub struct Residual {
    pub vt:      f64,
    pub weights: Vec<f64>,
    pub coords:  Vec<RiskCoord>,
}

impl Residual {
    pub fn recompute(&mut self) {
        let mut v = 0.0;
        for (j, rc) in self.coords.iter().enumerate() {
            let w = self.weights.get(j).copied().unwrap_or(0.0);
            v += w * rc.value;
        }
        self.vt = v;
    }
}

/// Corridor decision: derate, stop, and reason.
#[derive(Clone, Debug)]
pub struct CorridorDecision {
    pub derate: bool,
    pub stop:   bool,
    pub reason: String,
}

/// Runtime invariant: “violated corridor ⇒ derate/stop” and
/// “V_{t+1} must not increase outside safe interior”.
pub fn enforce_safe_step(prev: &Residual, next: &Residual) -> CorridorDecision {
    if next.coords.iter().any(|rc| rc.value >= 1.0) {
        return CorridorDecision {
            derate: true,
            stop:   true,
            reason: "hard corridor limit exceeded".to_string(),
        };
    }
    if next.vt > prev.vt {
        return CorridorDecision {
            derate: true,
            stop:   false,
            reason: "Lyapunov residual increased".to_string(),
        };
    }
    CorridorDecision {
        derate: false,
        stop:   false,
        reason: "within ecosafety corridors".to_string(),
    }
}

/// 4D spacetime index plus discrete state for nanoswarm planning.
#[derive(Clone, Debug)]
pub struct Voxel5D {
    pub x_m: f64,
    pub y_m: f64,
    pub z_m: f64,
    pub t:   Duration,   // simulation or mission time
    pub state: VoxelState,
}

/// Ecological state of a voxel, including indices and envelopes.
#[derive(Clone, Debug)]
pub struct VoxelState {
    pub tdi: f64,              // Tree/Detrital Index or Terrain Disturbance Index
    pub mbi: f64,              // Macro-Benthic / Biotic Index
    pub eco_impact_score: f64, // normalized 0–1 eco-benefit
    pub radiation_index: f64,  // 0–1 normalized radiation hazard

    pub r_tdi: RiskCoord,
    pub r_mbi: RiskCoord,
    pub r_eco: RiskCoord,
    pub r_rad: RiskCoord,

    pub r_bee:   RiskCoord,    // bee wellness corridor
    pub r_marine: RiskCoord,   // marine / aquatic corridor
    pub r_human: RiskCoord,    // human WBGT / toxicity corridor

    pub life_envelope: LifeEnvelope,
}

/// LifeEnvelope expresses which envelopes must be respected.
#[derive(Clone, Debug)]
pub enum LifeEnvelope {
    BeeEnvelope(BeeEnvelope),
    AquaticEnvelope(AquaticEnvelope),
    HumanEnvelope(HumanEnvelope),
    Multi(Vec<LifeEnvelope>),
}

#[derive(Clone, Debug)]
pub struct BeeEnvelope {
    pub hb_score: f64,    // 0–1, like HB-rating
    pub r_heat:   RiskCoord,
    pub r_emf:    RiskCoord,
    pub r_chem:   RiskCoord,
}

#[derive(Clone, Debug)]
pub struct AquaticEnvelope {
    pub r_temp:    RiskCoord,
    pub r_do:      RiskCoord,
    pub r_nutrient: RiskCoord,
    pub r_toxic:   RiskCoord,
}

#[derive(Clone, Debug)]
pub struct HumanEnvelope {
    pub r_wbgt: RiskCoord,
    pub r_pm:   RiskCoord,
    pub r_noise: RiskCoord,
}
