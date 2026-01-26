use response_shard::{RiskCoord, Residual, Triad, DraftAssessment, evaluate_draft};
use thiserror::Error;

/// Simple mass-balance kernel for SAT cell eco-benefit. [file:14]
pub fn eco_benefit_kg_removed(nitrate_in_mg_l: f64, nitrate_out_mg_l: f64, flow_m3_d: f64) -> f64 {
    let delta_mg_l = (nitrate_in_mg_l - nitrate_out_mg_l).max(0.0);
    let kg_per_m3 = delta_mg_l * 1e-6;
    kg_per_m3 * flow_m3_d
}

/// Normalize a variable into r_x ∈ [0,1] given safe/gold/hard bands. [file:7]
fn normalize(value: f64, safe: f64, gold: f64, hard: f64) -> f64 {
    if value <= safe {
        0.0
    } else if value >= hard {
        1.0
    } else {
        (value - safe) / (hard - safe)
    }
}

/// Compute risk coordinates for a SAT scenario. [file:14]
pub fn sat_risk_coords(
    hlr_m_d: f64,
    pfas_ng_l: f64,
    temp_c: f64,
) -> Vec<RiskCoord> {
    let r_sat = normalize(hlr_m_d, 0.05, 0.15, 0.25);
    let r_pfas = normalize(pfas_ng_l, 5.0, 10.0, 20.0);
    let r_temp = normalize(temp_c, 15.0, 25.0, 30.0);

    vec![
        RiskCoord {
            var_id: "r_sat".into(),
            value: r_sat,
            safe: 0.0,
            gold: 0.7,
            hard: 1.0,
            weight: 0.4,
        },
        RiskCoord {
            var_id: "r_pfas".into(),
            value: r_pfas,
            safe: 0.0,
            gold: 0.7,
            hard: 1.0,
            weight: 0.4,
        },
        RiskCoord {
            var_id: "r_temp".into(),
            value: r_temp,
            safe: 0.0,
            gold: 0.7,
            hard: 1.0,
            weight: 0.2,
        },
    ]
}

#[derive(Debug, Error)]
pub enum SatEvalError {
    #[error("Previous shard missing or invalid")]
    MissingPrevious,
}

/// Evaluate whether a proposed configuration tightens the SAT pilot shard. [file:14]
pub fn evaluate_sat_scenario(
    user_did: &str,
    nitrate_in_mg_l: f64,
    nitrate_out_mg_l: f64,
    flow_m3_d: f64,
    hlr_m_d: f64,
    pfas_ng_l: f64,
    temp_c: f64,
    prev_shard: Option<response_shard::ResponseShard>,
) -> Result<(response_shard::ResponseShard, bool), SatEvalError> {
    let benefit = eco_benefit_kg_removed(nitrate_in_mg_l, nitrate_out_mg_l, flow_m3_d);
    let e = if benefit <= 0.0 {
        0.0
    } else if benefit >= 1000.0 {
        1.0
    } else {
        benefit / 1000.0
    };

    let coords = sat_risk_coords(hlr_m_d, pfas_ng_l, temp_c);
    let residual = response_shard::Residual::from_coords(coords.clone());

    // Simple illustrative K/R; in practice, use real corridor coverage and residual. [file:6][file:14]
    let k = 0.93;
    let r = residual.vt.min(0.3);

    let triad_inputs = response_shard::TriadInputs { k, e, r };

    let draft = DraftAssessment {
        user_did: user_did.to_string(),
        topic: "phoenix-mar-sat".into(),
        base_triads: triad_inputs,
        base_coords: coords,
        evidence: vec![
            format!("nitrate_in_mg_l={}", nitrate_in_mg_l),
            format!("nitrate_out_mg_l={}", nitrate_out_mg_l),
            format!("flow_m3_d={}", flow_m3_d),
        ],
        corridor_tags: vec!["mar".into(), "sat".into(), "phoenix".into()],
    };

    let shard = evaluate_draft(draft);

    let improves = if let Some(prev) = prev_shard {
        shard.improves_over(&prev)
    } else {
        false
    };

    Ok((shard, improves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eco_benefit_positive() {
        let kg = eco_benefit_kg_removed(10.0, 5.0, 1000.0);
        assert!(kg > 0.0);
    }

    #[test]
    fn sat_risk_coords_range() {
        let coords = sat_risk_coords(0.1, 8.0, 22.0);
        for c in coords {
            assert!(c.value >= 0.0 && c.value <= 1.0);
        }
    }
}
