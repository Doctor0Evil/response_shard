// SPDX-License-Identifier: MIT
// Fog eco-biome kernels for r_tot and V_t, consistent with ecosafety grammar.

use crate::types::{FogNodeShard, FogPanelMaterialEvidence};

/// Corridor bands for normalization (Phoenix defaults).
pub struct ToxicityCorridors {
    pub erc50_safe_mg_l: f64,     // e.g., 100.0
    pub erc50_hard_mg_l: f64,     // e.g., 10.0
    pub pfas_safe_ng_l: f64,      // e.g., 4.0
    pub pfas_hard_ng_l: f64,      // e.g., 70.0
    pub arom_safe_ng_l: f64,      // e.g., 100.0
    pub arom_hard_ng_l: f64,      // e.g., 1000.0
    pub r_tot_gate: f64,          // typically 0.10
}

/// Clamp helper 0–1.
fn clamp01(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

/// Normalize algal ecotoxicity from OECD 201 (ErC50 in mg/L).
pub fn compute_r_algae(erc50_mg_l: f64, c: &ToxicityCorridors) -> f64 {
    if erc50_mg_l >= c.erc50_safe_mg_l {
        0.0
    } else if erc50_mg_l <= c.erc50_hard_mg_l {
        1.0
    } else {
        let span = c.erc50_safe_mg_l - c.erc50_hard_mg_l;
        let delta = c.erc50_safe_mg_l - erc50_mg_l;
        clamp01(delta / span)
    }
}

/// Normalize PFAS band (using max of PFBS/PFOS/GenX) relative to safe/hard bands.
pub fn compute_r_pfas(max_pfas_ng_l: f64, c: &ToxicityCorridors) -> f64 {
    if max_pfas_ng_l <= c.pfas_safe_ng_l {
        0.0
    } else if max_pfas_ng_l >= c.pfas_hard_ng_l {
        1.0
    } else {
        let span = c.pfas_hard_ng_l - c.pfas_safe_ng_l;
        let delta = max_pfas_ng_l - c.pfas_safe_ng_l;
        clamp01(delta / span)
    }
}

/// Normalize aromatics (or VOC sum) to corridor bands.
pub fn compute_r_arom(arom_sum_ng_l: f64, c: &ToxicityCorridors) -> f64 {
    if arom_sum_ng_l <= c.arom_safe_ng_l {
        0.0
    } else if arom_sum_ng_l >= c.arom_hard_ng_l {
        1.0
    } else {
        let span = c.arom_hard_ng_l - c.arom_safe_ng_l;
        let delta = arom_sum_ng_l - c.arom_safe_ng_l;
        clamp01(delta / span)
    }
}

/// Normalize biodegradation shortfall (ISO 14851) into r_deg.
/// Here we treat any failure to hit 60% at 28d or 90% at 180d as rising r_deg.
pub fn compute_r_deg(thod28d_pct: f64, thod180d_pct: f64) -> f64 {
    let target_28 = 60.0;
    let target_180 = 90.0;

    let short_28 = if thod28d_pct >= target_28 {
        0.0
    } else {
        (target_28 - thod28d_pct) / target_28
    };

    let short_180 = if thod180d_pct >= target_180 {
        0.0
    } else {
        (target_180 - thod180d_pct) / target_180
    };

    clamp01(0.5 * short_28 + 0.5 * short_180)
}

/// Compute total toxicity index r_tot from component r's.
/// Weights emphasize toxicity; no coordinate can be fully canceled.
pub fn compute_r_tot(r_algae: f64, r_pfas: f64, r_arom: f64) -> f64 {
    let w_algae = 0.5;
    let w_pfas = 0.4;
    let w_arom = 0.1;
    clamp01(w_algae * r_algae + w_pfas * r_pfas + w_arom * r_arom)
}

/// Fill FogPanelMaterialEvidence risk fields and biosurface_ok flag.
pub fn enrich_material_evidence(
    mut ev: FogPanelMaterialEvidence,
    corridors: &ToxicityCorridors,
) -> FogPanelMaterialEvidence {
    let max_pfas = ev
        .lcms_pfas_pfbs_ng_l
        .max(ev.lcms_pfas_pfos_ng_l)
        .max(ev.lcms_pfas_genx_ng_l);

    ev.r_deg = compute_r_deg(ev.iso14851_thod28d_pct, ev.iso14851_thod180d_pct);
    ev.r_algae = compute_r_algae(ev.oecd201_erc50_mg_l, corridors);
    ev.r_pfas = compute_r_pfas(max_pfas, corridors);
    ev.r_arom = compute_r_arom(ev.lcms_aromatics_sum_ng_l, corridors);
    ev.r_tot = compute_r_tot(ev.r_algae, ev.r_pfas, ev.r_arom);

    ev.biosurface_ok = ev.r_tot <= corridors.r_tot_gate;
    ev
}

/// Compute Lyapunov residual V_t from a minimal toxicity slice.
/// In integration, you will extend this to include hydraulic/thermal channels too.
pub fn compute_v_t(r_tot: f64, r_deg: f64, r_algae: f64, r_pfas: f64) -> f64 {
    // Example simple weighted sum (sign and offsets can be tuned).
    let w_tot = 0.5;
    let w_deg = 0.2;
    let w_algae = 0.2;
    let w_pfas = 0.1;
    let raw = w_tot * r_tot + w_deg * r_deg + w_algae * r_algae + w_pfas * r_pfas;
    // Store as log10 scale residual; V_t <= 0 is "good".
    (raw + 1e-6).log10()
}

/// Update a FogNodeShard with r_tot, V_t, and invariant booleans, given evidence.
pub fn hydrate_fog_node_from_evidence(
    mut node: FogNodeShard,
    ev: &FogPanelMaterialEvidence,
    corridors: &ToxicityCorridors,
    prev_v_t: Option<f64>,
) -> FogNodeShard {
    // Copy ISO/OECD and r-fields from evidence
    node.iso14851_thod28d_pct = ev.iso14851_thod28d_pct;
    node.iso14851_thod180d_pct = ev.iso14851_thod180d_pct;
    node.oecd201_erc50_mg_l = ev.oecd201_erc50_mg_l;
    node.oecd201_noec_mg_l = ev.oecd201_noec_mg_l;
    node.r_deg = ev.r_deg;
    node.r_algae = ev.r_algae;
    node.r_pfas = ev.r_pfas;
    node.r_arom = ev.r_arom;
    node.r_tot = ev.r_tot;

    node.biosurfaceok = ev.biosurface_ok;

    node.v_t = compute_v_t(node.r_tot, node.r_deg, node.r_algae, node.r_pfas);

    // Lyapunov ok if either no previous residual, or non-increasing outside safe interior.
    let safe_interior = 0.0; // can be tuned; e.g., V_t <= -1e-3
    node.v_t_ok = if let Some(prev) = prev_v_t {
        if prev <= safe_interior && node.v_t <= safe_interior {
            true
        } else {
            node.v_t <= prev
        }
    } else {
        true
    };

    node.lyapunovok = node.v_t_ok;

    node
}
