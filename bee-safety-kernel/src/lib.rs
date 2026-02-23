use ecosafety_core::{Residual, CorridorDecision};

#[derive(Clone, Copy)]
pub struct BeeEnvInputs {
    pub temp_c: f32,
    pub emf_mw_cm2: f32,
    pub chem_mg_m3: f32,
}

#[derive(Clone, Copy)]
pub struct BeeControl {
    // high-level, non-harmful intent (e.g., fan duty fraction)
    pub ventilation_duty: f32,
    pub heater_duty: f32,
}

pub trait BeeSafetyKernel {
    fn check_step(
        &self,
        eco_state: &BeeEnvInputs,
        proposed: &BeeControl,
        prev: &Residual,
    ) -> (CorridorDecision, Residual);
}
