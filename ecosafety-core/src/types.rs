#[derive(Clone, Copy)]
pub struct CorridorBands {
    pub var_id: &'static str,
    pub units: &'static str,
    pub safe: f64,    // safe band upper edge (normalized)
    pub gold: f64,    // gold band
    pub hard: f64,    // hard limit (== 1.0)
    pub weight: f64,  // w_j in V_t = Σ w_j r_j
    pub lyap_channel: u8,
}

#[derive(Clone, Copy)]
pub struct RiskCoord {
    pub r: f64,             // r_j ∈ [0, 1]
    pub sigma: f64,         // uncertainty
    pub bands: CorridorBands,
}

pub struct Residual {
    pub vt: f64,
    pub coords: &'static [RiskCoord],
}

impl Residual {
    pub fn recompute(&mut self) {
        self.vt = self.coords.iter().map(|c| c.r * c.bands.weight).sum();
    }
}
