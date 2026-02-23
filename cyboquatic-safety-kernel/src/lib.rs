use ecosafety_core::{Residual, CorridorDecision};

#[derive(Clone, Copy)]
pub struct AquaticState {
    pub temp_c: f32,
    pub do_mg_l: f32,
    pub ph: f32,
    pub turbidity_ntu: f32,
    pub fish_scalar: f32,  // dimensionless health index
}

#[derive(Clone, Copy)]
pub struct FlowControl {
    pub flow_l_s: f32,
    pub aeration_power_w: f32,
}

pub trait CyboquaticSafetyKernel {
    fn check_step(
        &self,
        eco_state: &AquaticState,
        proposed: &FlowControl,
        prev: &Residual,
    ) -> (CorridorDecision, Residual);
}
