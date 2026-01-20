#[derive(Clone, Copy, Debug)]
pub struct KerMetrics {
    pub k: f64,  // knowledge-factor 0–1
    pub e: f64,  // eco-impact value 0–1
    pub r: f64,  // risk-of-harm 0–1
}

impl KerMetrics {
    pub fn is_well_formed(&self) -> bool {
        fn in_01(x: f64) -> bool {
            x.is_finite() && x >= 0.0 && x <= 1.0
        }
        in_01(self.k) && in_01(self.e) && in_01(self.r)
    }
}
