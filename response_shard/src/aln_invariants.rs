//! ALN-style invariants as pure Rust functions. [file:7]

use crate::{Residual, RiskCoord};

/// Invariant 1: no_corridor_no_build – every required variable must have a corridor row. [file:7]
pub fn no_corridor_no_build(required_vars: &[&str], coords: &[RiskCoord]) -> bool {
    required_vars.iter().all(|var| coords.iter().any(|c| c.var_id == *var))
}

/// Decision returned by `safestep`. [file:7]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorridorDecision {
    Ok,
    Derate,
    Stop,
}

/// Invariant 2: safestep – reject moves that cross hard limits or increase V_t. [file:7]
pub fn safestep(prev: &Residual, next: &Residual) -> CorridorDecision {
    if next
        .coords
        .iter()
        .any(|c| c.value > 1.0 || c.value < 0.0)
    {
        return CorridorDecision::Stop;
    }

    if next.vt > prev.vt {
        return CorridorDecision::Derate;
    }

    CorridorDecision::Ok
}

/// Invariant 3: ker_delta – require non-degrading K/E/R against thresholds. [file:6]
pub fn ker_delta(
    prev_k: f64,
    prev_e: f64,
    prev_r: f64,
    next_k: f64,
    next_e: f64,
    next_r: f64,
    min_k: f64,
    min_e: f64,
    max_r: f64,
) -> bool {
    next_k >= prev_k
        && next_e >= prev_e
        && next_r <= prev_r
        && next_k >= min_k
        && next_e >= min_e
        && next_r <= max_r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_corridor_no_build_works() {
        let coords = vec![
            RiskCoord {
                var_id: "r_sat".into(),
                value: 0.3,
                safe: 0.3,
                gold: 0.7,
                hard: 1.0,
                weight: 1.0,
            },
        ];
        assert!(no_corridor_no_build(&["r_sat"], &coords));
        assert!(!no_corridor_no_build(&["r_sat", "r_pfas"], &coords));
    }

    #[test]
    fn safestep_derates_on_v_increase() {
        let prev = Residual { vt: 0.2, coords: vec![] };
        let next = Residual { vt: 0.25, coords: vec![] };
        assert_eq!(safestep(&prev, &next), CorridorDecision::Derate);
    }

    #[test]
    fn ker_delta_respects_thresholds() {
        assert!(ker_delta(0.90, 0.88, 0.15, 0.93, 0.90, 0.13, 0.90, 0.89, 0.13));
        assert!(!ker_delta(0.90, 0.88, 0.15, 0.89, 0.90, 0.13, 0.90, 0.89, 0.13));
    }
}
