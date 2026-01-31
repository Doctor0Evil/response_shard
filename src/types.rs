#[derive(Clone, Debug)]
pub struct CorridorBands {
    pub var_id: String,    // e.g. "HLR", "PFAS", "TEMP"
    pub units: String,     // "m/d", "ng/L", "degC"
    pub safe: f64,         // inner "comfortable" band
    pub gold: f64,         // regulatory / science "gold" limit
    pub hard: f64,         // absolute never-exceed limit
    pub weight_w: f64,     // weight in V(t)
    pub lyap_channel: u16, // channel index for residual decomposition
}

#[derive(Clone, Debug)]
pub struct RiskCoord {
    pub value: f64,        // normalized r_x in [0,1]
    pub bands: CorridorBands,
    pub sigma: f64,        // uncertainty
}

#[derive(Clone, Debug)]
pub struct Residual {
    pub vt: f64,           // Lyapunov-style residual
    pub w: Vec<f64>,       // weights
    pub rx: Vec<RiskCoord> // risk coordinates in fixed order
}

#[derive(Clone, Debug)]
pub struct CorridorDecision {
    pub derate: bool,
    pub stop: bool,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct KerScore {
    pub knowledge_k: f64,  // 0–1
    pub eco_impact_e: f64, // 0–1
    pub risk_r: f64,       // 0–1
}
