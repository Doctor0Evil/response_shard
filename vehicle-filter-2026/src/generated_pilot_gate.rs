// Auto-generated from aln/VehicleFilterPilotGate2026v1.aln
// Pilot-Gate bindings for VehicleFilter2026v1 shards.
// Hex-stamp (placeholder) for bostrom-aligned authorship anchoring:
// 0x4a2b1c3d5e6f7890a1b2c3d4e5f67890abcdef1234567890fedcba9876543210

#![forbid(unsafe_code)]

use crate::contracts::{CorridorBands, Residual, CorridorDecision};
use crate::contracts::{riskcoord_leq_one, safestep};
use crate::shard::{VehicleFilterShard, RiskCoordEntry};

/// Convenience: required corridor variable IDs for VehicleFilter2026v1.
/// These must match canonical IDs in qpudatashards and ALN:
///   "pm", "nox", "hc", "co", "backpressure", "substratetemp"
pub const REQUIRED_VARS: [&str; 6] = [
    "pm",
    "nox",
    "hc",
    "co",
    "backpressure",
    "substratetemp",
];

/// Check that a shard has all required corridors and a valid DID signature.
/// Implements **no corridor, no deployment**.
pub fn has_all_corridors(shard: &VehicleFilterShard) -> bool {
    // DID signature check – wire this to your actual signer/validator.
    if !shard.header.didsignature_valid {
        return false;
    }

    for required in REQUIRED_VARS.iter() {
        let found = shard
            .corridors
            .iter()
            .any(|c| c.varid.as_str() == *required);
        if !found {
            return false;
        }
    }

    true
}

/// Normalize risk coordinates and compute residual Vt for a shard.
///
/// Assumes each corridor entry holds:
///   - measured (physical value),
///   - bands (CorridorBands with safe, gold, hard, weight),
/// so Vt can be reconstructed exactly from the shard.
pub fn compute_residual(shard: &VehicleFilterShard) -> Option<Residual> {
    if shard.corridors.is_empty() {
        return None;
    }

    let mut coords: Vec<RiskCoordEntry> = Vec::with_capacity(shard.corridors.len());

    for c in shard.corridors.iter() {
        let bands: &CorridorBands = &c.bands;
        let measured = c.measured;

        // tor_j-style normalization:
        //   r = 0  if measured <= safe
        //   r = 1  if measured >= hard
        //   linear between safe and hard otherwise
        let r_value = if measured <= bands.safe {
            0.0
        } else if measured >= bands.hard {
            1.0
        } else {
            (measured - bands.safe) / (bands.hard - bands.safe)
        };

        let mut entry = c.clone();
        entry.rx = r_value;
        coords.push(entry);
    }

    // Hard bound: all rx <= 1.0
    if !coords.iter().all(|e| riskcoord_leq_one(e.rx)) {
        return None;
    }

    // Lyapunov-style residual Vt = Σ w_j * r_j
    let vt = coords
        .iter()
        .map(|e| e.bands.weight_w * e.rx)
        .sum::<f64>();

    Some(Residual { vt, coords })
}

/// Safestep invariant for time-series shards.
///
/// Semantics:
/// - If all rx are inside their safe interior (rx <= safe_rx), Vt is allowed to float.
/// - Otherwise, we require V_{t+1} <= V_t (Lyapunov decrease outside safe interior).
pub fn safestep_shard(prev: &VehicleFilterShard, next: &VehicleFilterShard) -> bool {
    let prev_res = match compute_residual(prev) {
        Some(r) => r,
        None => return false,
    };
    let next_res = match compute_residual(next) {
        Some(r) => r,
        None => return false,
    };

    // Safe interior: rx <= bands.safe_rx (often 0.0 or a small tolerance).
    let all_safe = next_res
        .coords
        .iter()
        .all(|e| e.rx <= e.bands.safe_rx);

    let lyapunov_ok = if all_safe {
        // Inside safe interior, we do not enforce strict decrease.
        true
    } else {
        // Outside safe interior, require V_{t+1} <= V_t.
        next_res.vt <= prev_res.vt
    };

    // Time ordering: strictly increasing timestamps.
    let time_ok = prev.header.timestamp < next.header.timestamp;

    lyapunov_ok && time_ok
}

/// High-level Pilot-Gate decisions for a single shard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PilotGateDecision {
    Approve,
    Derate,
    Stop,
}

/// Approve predicate.
///
/// Conditions mirror your ALN sketch:
/// - All required corridors present and DID-signed.
/// - Residual Vt below a gold-like bound (here 0.5).
/// - Eco-impact E >= 0.9.
/// - Risk-of-harm R <= 0.15.
pub fn pilot_gate_approve(shard: &VehicleFilterShard) -> Option<PilotGateDecision> {
    if !has_all_corridors(shard) {
        return None;
    }

    let res = compute_residual(shard)?;
    if res.vt >= 0.5 {
        return None;
    }

    let ker = &shard.ker;
    if ker.ecoimpact_value < 0.9 {
        return None;
    }
    if ker.risk_of_harm > 0.15 {
        return None;
    }

    Some(PilotGateDecision::Approve)
}

/// Derate predicate: Vt in [0.5, 1.0).
pub fn pilot_gate_derate(shard: &VehicleFilterShard) -> Option<PilotGateDecision> {
    let res = compute_residual(shard)?;
    if res.vt >= 0.5 && res.vt < 1.0 {
        Some(PilotGateDecision::Derate)
    } else {
        None
    }
}

/// Stop predicate: Vt >= 1.0 (hard corridor violation).
pub fn pilot_gate_stop(shard: &VehicleFilterShard) -> Option<PilotGateDecision> {
    let res = compute_residual(shard)?;
    if res.vt >= 1.0 {
        Some(PilotGateDecision::Stop)
    } else {
        None
    }
}

/// Governance over a chain of shards (one filter over time).
///
/// Enforces:
/// - `safestep_shard` for every consecutive pair (Lyapunov + time).
/// - All shards must have valid DID signatures.
/// - Decision on final shard with priority: Stop > Derate > Approve.
pub fn govern_vehicle_filter_chain(chain: &[VehicleFilterShard]) -> Option<PilotGateDecision> {
    if chain.is_empty() {
        return None;
    }

    // Time-series invariants over the chain.
    for window in chain.windows(2) {
        let prev = &window[0];
        let next = &window[1];

        if !safestep_shard(prev, next) {
            return Some(PilotGateDecision::Stop);
        }

        if !prev.header.didsignature_valid || !next.header.didsignature_valid {
            return Some(PilotGateDecision::Stop);
        }
    }

    let last = chain.last().unwrap();

    // Priority: Stop, then Derate, then Approve.
    if let Some(dec) = pilot_gate_stop(last) {
        return Some(dec);
    }
    if let Some(dec) = pilot_gate_derate(last) {
        return Some(dec);
    }
    if let Some(dec) = pilot_gate_approve(last) {
        return Some(dec);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shard::{VehicleFilterShardHeader, KerFields};

    fn dummy_bands() -> CorridorBands {
        CorridorBands {
            safe: 0.0,
            gold: 0.5,
            hard: 1.0,
            safe_rx: 0.0,
            weight_w: 1.0,
            lyap_channel: 0,
        }
    }

    fn dummy_shard_with_vt(vt_target: f64) -> VehicleFilterShard {
        // Minimal dummy shard with 2 equal-weight coords.
        // We back-compute rx so that Vt = vt_target.
        let bands = dummy_bands();
        let rx = vt_target / 2.0;

        let entry1 = RiskCoordEntry {
            varid: "pm".to_string(),
            measured: rx, // purely illustrative
            bands: bands.clone(),
            rx,
        };
        let entry2 = RiskCoordEntry {
            varid: "backpressure".to_string(),
            measured: rx,
            bands: bands.clone(),
            rx,
        };

        VehicleFilterShard {
            header: VehicleFilterShardHeader {
                didsignature_valid: true,
                timestamp: 0,
                ..Default::default()
            },
            corridors: vec![entry1, entry2],
            ker: KerFields {
                ecoimpact_value: 0.95,
                risk_of_harm: 0.10,
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_approve_derate_stop_thresholds() {
        let mut s_approve = dummy_shard_with_vt(0.4);
        s_approve.header.timestamp = 1;

        let mut s_derate = dummy_shard_with_vt(0.7);
        s_derate.header.timestamp = 2;

        let mut s_stop = dummy_shard_with_vt(1.1);
        s_stop.header.timestamp = 3;

        assert_eq!(
            pilot_gate_approve(&s_approve),
            Some(PilotGateDecision::Approve)
        );
        assert_eq!(
            pilot_gate_derate(&s_derate),
            Some(PilotGateDecision::Derate)
        );
        assert_eq!(pilot_gate_stop(&s_stop), Some(PilotGateDecision::Stop));
    }

    #[test]
    fn test_govern_vehicle_filter_priority() {
        let mut s1 = dummy_shard_with_vt(0.4);
        s1.header.timestamp = 1;
        let mut s2 = dummy_shard_with_vt(0.7);
        s2.header.timestamp = 2;
        let mut s3 = dummy_shard_with_vt(1.1);
        s3.header.timestamp = 3;

        let chain = vec![s1, s2, s3];
        let decision = govern_vehicle_filter_chain(&chain);
        assert_eq!(decision, Some(PilotGateDecision::Stop));
    }
}
