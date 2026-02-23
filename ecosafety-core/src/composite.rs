use crate::{Residual, CorridorDecision};
use bee_safety_kernel::{BeeEnvInputs, BeeControl, BeeSafetyKernel};
use cyboquatic_safety_kernel::{AquaticState, FlowControl, CyboquaticSafetyKernel};

pub struct CompositeKernel<B, C> {
    pub bee: B,
    pub cybo: C,
}

pub enum CompositeDecision {
    Ok,
    Derate { bee: bool, cybo: bool },
    Stop  { bee: bool, cybo: bool },
}

impl<B, C> CompositeKernel<B, C>
where
    B: BeeSafetyKernel,
    C: CyboquaticSafetyKernel,
{
    pub fn step(
        &self,
        bee_state: &BeeEnvInputs,
        bee_ctl: &BeeControl,
        aquatic_state: &AquaticState,
        flow_ctl: &FlowControl,
        prev: &Residual,
    ) -> (CompositeDecision, Residual) {
        let (d_bee, res_bee) = self.bee.check_step(bee_state, bee_ctl, prev);
        let (d_cyb, res_cyb) = self.cybo.check_step(aquatic_state, flow_ctl, &res_bee);

        let decision = match (d_bee, d_cyb) {
            (CorridorDecision::Ok, CorridorDecision::Ok) => CompositeDecision::Ok,
            (CorridorDecision::Stop, _) | (_, CorridorDecision::Stop) =>
                CompositeDecision::Stop {
                    bee: matches!(d_bee, CorridorDecision::Stop),
                    cybo: matches!(d_cyb, CorridorDecision::Stop),
                },
            _ => CompositeDecision::Derate {
                bee: matches!(d_bee, CorridorDecision::Derate),
                cybo: matches!(d_cyb, CorridorDecision::Derate),
            },
        };
        (decision, res_cyb)
    }
}
