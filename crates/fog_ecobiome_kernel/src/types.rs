// SPDX-License-Identifier: MIT
// Fog eco-biome types for Phoenix-class FOG routing (non-Python, Rust-only)

use serde::{Deserialize, Serialize};

/// Phoenix-class ISO 14851 / OECD 201 evidence for a sorbent panel or media batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FogPanelMaterialEvidence {
    // Identity and provenance
    pub recipe_id: String,            // e.g., "FOG-PANEL-2026-01-A"
    pub batch_id: String,             // lab or production batch
    pub region: String,               // e.g., "Phoenix-AZ"
    pub matrix_profile: String,       // e.g., "phoenix_synthetic_canal_tds850mgL_ph8_4_30C"
    pub evidence_hex: String,         // hex stamp over evidence shard

    // ISO 14851 biodegradation metrics
    pub iso14851_thod28d_pct: f64,    // [%] ThOD at 28 days
    pub iso14851_thod180d_pct: f64,   // [%] ThOD at 180 days

    // OECD 201 algal toxicity (Phoenix matrix)
    pub oecd201_erc50_mg_l: f64,      // [mg/L] ErC50
    pub oecd201_noec_mg_l: f64,       // [mg/L] NOEC

    // LC-MS PFAS and co-contaminants (Phoenix canal / leachate)
    pub lcms_pfas_pfbs_ng_l: f64,     // [ng/L]
    pub lcms_pfas_pfos_ng_l: f64,     // [ng/L]
    pub lcms_pfas_genx_ng_l: f64,     // [ng/L]
    pub lcms_aromatics_sum_ng_l: f64, // [ng/L] sum of aromatic co-contaminants

    // Normalized risk coordinates (0–1), computed by kernels
    pub r_deg: f64,                   // biodegradation corridor shortfall
    pub r_algae: f64,                 // algal ecotoxicity
    pub r_pfas: f64,                  // PFAS band overshoot
    pub r_arom: f64,                  // aromatic / VOC overshoot
    pub r_tot: f64,                   // total organic toxicity index

    // Governance flags
    pub biosurface_ok: bool,          // r_tot <= 0.1 and no hard-band breach
    pub knowledge_factor: f64,        // 0–1, evidence coverage
}

/// Phoenix-class FOG node runtime shard for routing and FOG-panel coupling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FogNodeShard {
    // Identity
    pub nodeid: String,
    pub timestamputc: String,
    pub lat: f64,
    pub lon: f64,
    pub region: String,               // "Phoenix-AZ"

    // Environment (indoor / vault-adjacent air)
    pub matrix_profile: String,       // "phoenix_synthetic_canal_tds850mgL_ph8_4_30C"
    pub env_profile_id: String,       // e.g., "PHX-FOG-ENV-01"
    pub temp_c: f64,                  // [°C]
    pub rh_pct: f64,                  // [%]
    pub pm10_ug_m3: f64,              // [µg/m3]
    pub pfas_ng_l: f64,               // [ng/L] in condensate / leachate
    pub pfas_adsorption_ng_per_l: f64,// [ng/L] removed by panel
    pub hlr_max_l_m2_h: f64,          // [L/m2·h] design HLR
    pub dust_abrasion_pm10_ug_m3_threshold: f64, // [µg/m3] wear threshold

    // Material safety snapshot (derived from FogPanelMaterialEvidence)
    pub iso14851_thod28d_pct: f64,
    pub iso14851_thod180d_pct: f64,
    pub oecd201_erc50_mg_l: f64,
    pub oecd201_noec_mg_l: f64,
    pub r_deg: f64,
    pub r_algae: f64,
    pub r_pfas: f64,
    pub r_arom: f64,
    pub r_tot: f64,

    // Residual and invariants
    pub v_t: f64,                     // Lyapunov residual at t
    pub v_t_ok: bool,                 // V_{t+1} <= V_t outside safe interior
    pub biosurfaceok: bool,           // r_tot <= 0.1 and no hard limits reached
    pub lyapunovok: bool,             // contract-satisfied for last step

    // Governance scores
    pub ecoimpactscore: f64,          // 0–1
    pub karmadelta: f64,              // NanoKarmaBytes / similar
    pub evidencehex: String,          // hex stamp for this row
}
