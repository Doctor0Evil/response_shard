use serde::{Deserialize, Serialize};

pub mod aln_invariants;

/// Knowledge-factor K, Eco-impact E, Risk-of-harm R. [file:6]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Triad {
    pub knowledge: f64,   // K in [0,1]
    pub eco_impact: f64,  // E in [0,1]
    pub risk_of_harm: f64 // R in [0,1]
}

/// Single normalized risk coordinate r_x ∈ [0,1] plus metadata. [file:7]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCoord {
    pub var_id: String,
    pub value: f64,
    pub safe: f64,
    pub gold: f64,
    pub hard: f64,
    pub weight: f64,
}

/// Lyapunov-style residual V_t = Σ w_j r_j. [file:6]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Residual {
    pub vt: f64,
    pub coords: Vec<RiskCoord>,
}

impl Residual {
    pub fn from_coords(coords: Vec<RiskCoord>) -> Self {
        let vt = coords.iter().map(|c| c.weight * c.value).sum();
        Self { vt, coords }
    }
}

/// Minimal shard for a single response turn. [file:6]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseShard {
    pub user_did: String,
    pub topic: String,
    pub triad: Triad,
    pub residual: Residual,
    pub evidence: Vec<String>,
    pub corridor_tags: Vec<String>,
}

impl ResponseShard {
    /// Computes a simple “tightening” signal:
    /// - K must not fall, E must not fall, R must not rise, and V_t must not rise. [file:6]
    pub fn improves_over(&self, previous: &ResponseShard) -> bool {
        let k_ok = self.triad.knowledge >= previous.triad.knowledge;
        let e_ok = self.triad.eco_impact >= previous.triad.eco_impact;
        let r_ok = self.triad.risk_of_harm <= previous.triad.risk_of_harm;
        let v_ok = self.residual.vt <= previous.residual.vt;
        k_ok && e_ok && r_ok && v_ok
    }
}

/// Draft input from an upstream system (e.g., AI answer + MAR proposal). [file:6]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftAssessment {
    pub user_did: String,
    pub topic: String,
    pub base_triads: TriadInputs,
    pub base_coords: Vec<RiskCoord>,
    pub evidence: Vec<String>,
    pub corridor_tags: Vec<String>,
}

/// Explicit K/E/R values for a draft. [file:6]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TriadInputs {
    pub k: f64,
    pub e: f64,
    pub r: f64,
}

/// Wrap a draft into a ResponseShard.
pub fn evaluate_draft(input: DraftAssessment) -> ResponseShard {
    let residual = Residual::from_coords(input.base_coords);
    let triad = Triad {
        knowledge: input.base_triads.k,
        eco_impact: input.base_triads.e,
        risk_of_harm: input.base_triads.r,
    };
    ResponseShard {
        user_did: input.user_did,
        topic: input.topic,
        triad,
        residual,
        evidence: input.evidence,
        corridor_tags: input.corridor_tags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tightening_logic_works() {
        let prev = ResponseShard {
            user_did: "bostrom18...".into(),
            topic: "phoenix-mar-sat".into(),
            triad: Triad { knowledge: 0.90, eco_impact: 0.88, risk_of_harm: 0.15 },
            residual: Residual { vt: 0.25, coords: vec![] },
            evidence: vec![],
            corridor_tags: vec!["mar".into()],
        };
        let next = ResponseShard {
            user_did: "bostrom18...".into(),
            topic: "phoenix-mar-sat".into(),
            triad: Triad { knowledge: 0.93, eco_impact: 0.90, risk_of_harm: 0.13 },
            residual: Residual { vt: 0.22, coords: vec![] },
            evidence: vec![],
            corridor_tags: vec!["mar".into()],
        };
        assert!(next.improves_over(&prev));
    }
}
