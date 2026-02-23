use ecosafety_core::{Residual, CorridorDecision};
use cyboquatic_safety_kernel::{AquaticState, FlowControl, CyboquaticSafetyKernel};

pub trait SafeController<K>
where
    K: CyboquaticSafetyKernel,
{
    fn kernel(&self) -> &K;

    fn step_with_safety(
        &mut self,
        eco_state: &AquaticState,
        prev_residual: &Residual,
    ) -> Result<(FlowControl, Residual), &'static str> {
        let candidate = self.propose_control(eco_state);
        let (decision, next_residual) = self.kernel().check_step(eco_state, &candidate, prev_residual);
        match decision {
            CorridorDecision::Ok    => Ok((candidate, next_residual)),
            CorridorDecision::Derate => Err("derate"),
            CorridorDecision::Stop   => Err("stop"),
        }
    }

    fn propose_control(&mut self, eco_state: &AquaticState) -> FlowControl;
}
