use crate::types::{CorridorBands, RiskCoord, Residual, CorridorDecision, KerScore};

#[derive(Clone, Debug)]
pub struct MarShard {
    pub mar_id: String,
    pub basin_id: String,
    pub lat: f64,
    pub lon: f64,
    pub aquifer_unit: String,
    pub climate_class: String,

    // Hydraulics
    pub hlr_current: f64,
    pub hlr_bands: CorridorBands, // var_id="HLR"
    pub q_in_m3d: f64,
    pub q_out_m3d: f64,
    pub res_time_d: f64,
    pub surcharge_count: u64,
    pub r_surcharge: RiskCoord,

    // CECs and nutrients
    pub c_pfas_in_ngl: f64,
    pub c_pfas_out_ngl: f64,
    pub r_pfas: RiskCoord,

    pub c_pharma_in_ngl: f64,
    pub c_pharma_out_ngl: f64,
    pub r_pharma: RiskCoord,

    pub c_n_in_mgl: f64,
    pub c_n_out_mgl: f64,
    pub r_n: RiskCoord,

    pub c_p_in_mgl: f64,
    pub c_p_out_mgl: f64,
    pub r_p: RiskCoord,

    // Thermal and redox
    pub t_plume_c: f64,
    pub t_bands: CorridorBands, // var_id="TEMP"
    pub r_thermal: RiskCoord,
    pub redox_state_mv: f64,
    pub r_redox: RiskCoord,

    // Fouling
    pub fouling_index: f64,
    pub fouling_bands: CorridorBands, // var_id="FOUL"
    pub cleaning_dose_eq: f64,
    pub r_foul: RiskCoord,

    // KER scores
    pub ker: KerScore,
}

/// Ensure all critical corridors are present and consistent.
/// This should be called in CI; if it returns false, build fails.
pub fn corridor_present(shard: &MarShard) -> bool {
    // Check HLr, PFAS, TEMP, FOUL bands and risk coords are consistent.
    let mut ok = true;

    ok &= shard.hlr_bands.var_id == "HLR";
    ok &= shard.t_bands.var_id == "TEMP";
    ok &= shard.fouling_bands.var_id == "FOUL";

    ok &= (shard.r_pfas.bands.var_id == "PFAS");
    ok &= (shard.r_pharma.bands.var_id == "PHARMA");
    ok &= (shard.r_n.bands.var_id == "N");
    ok &= (shard.r_p.bands.var_id == "P");
    ok &= (shard.r_thermal.bands.var_id == "TEMP");
    ok &= (shard.r_redox.bands.var_id == "REDOX");
    ok &= (shard.r_foul.bands.var_id == "FOUL");
    ok &= (shard.r_surcharge.bands.var_id == "SURCHARGE");

    ok
}

/// Normalize a raw metric into r_x in [0,1] using corridor bands.
pub fn normalize_metric(x: f64, bands: &CorridorBands) -> f64 {
    if x <= bands.safe {
        0.0
    } else if x >= bands.hard {
        1.0
    } else {
        (x - bands.safe) / (bands.hard - bands.safe)
    }
}

/// Compute V(t) = Σ w_j r_j(t)^2 using weights from bands.
pub fn compute_residual(risks: &[RiskCoord]) -> Residual {
    let mut vt = 0.0;
    let mut w = Vec::with_capacity(risks.len());
    for r in risks {
        let wj = r.bands.weight_w;
        vt += wj * r.value * r.value;
        w.push(wj);
    }
    Residual { vt, w, rx: risks.to_vec() }
}

/// Enforce "no corridor no build" and "violated corridor derate/stop".
pub fn safe_step(prev: &Residual, next: &Residual) -> CorridorDecision {
    // Any r_x >= 1.0 is a hard breach.
    if next.rx.iter().any(|r| r.value >= 1.0) {
        return CorridorDecision {
            derate: true,
            stop: true,
            reason: "hard corridor breach: r_x >= 1.0".to_string(),
        };
    }

    // Lyapunov non-increase: V(t+1) <= V(t) inside safe interior.
    if next.vt > prev.vt {
        return CorridorDecision {
            derate: true,
            stop: true,
            reason: "Lyapunov residual increased".to_string(),
        };
    }

    CorridorDecision {
        derate: false,
        stop: false,
        reason: "within corridors".to_string(),
    }
}

/// MAR-specific guard: SAT-safe, fouling-safe, and surcharge-safe.
pub fn sat_ok(shard: &MarShard) -> bool {
    // HLR within gold band, surcharge risk and fouling inside safe corridor.
    let r_hlr = normalize_metric(shard.hlr_current, &shard.hlr_bands);
    (r_hlr < 1.0)
        && (shard.r_surcharge.value < 1.0)
        && (shard.r_foul.value < 1.0)
}

/// Update KER scores from new evidence (placeholder logic).
pub fn update_ker(mut ker: KerScore, new_evidence_weight: f64) -> KerScore {
    // Example: increase knowledge and eco-impact slightly, reduce risk proportionally.
    let alpha = new_evidence_weight.clamp(0.0, 1.0);
    ker.knowledge_k = (ker.knowledge_k + alpha * (1.0 - ker.knowledge_k)).clamp(0.0, 1.0);
    ker.eco_impact_e = (ker.eco_impact_e + alpha * (1.0 - ker.eco_impact_e)).clamp(0.0, 1.0);
    ker.risk_r = (ker.risk_r * (1.0 - alpha)).clamp(0.0, 1.0);
    ker
}
