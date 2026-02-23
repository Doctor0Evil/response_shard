pub struct LegacyCommand {
    pub rpm: u16,
}

pub trait LegacyAdapter {
    fn to_safe_intent(&self, legacy: &LegacyCommand) -> FlowControl;
}

pub struct EdgeNode<K, A> {
    pub ctrl: Box<dyn SafeController<K>>,
    pub adapter: A,
}

impl<K, A> EdgeNode<K, A>
where
    K: CyboquaticSafetyKernel,
    A: LegacyAdapter,
{
    pub fn tick(
        &mut self,
        legacy_cmd: &LegacyCommand,
        eco_state: &AquaticState,
        residual: &Residual,
    ) -> Result<(FlowControl, Residual), &'static str> {
        let safe_intent = self.adapter.to_safe_intent(legacy_cmd);
        // optional: override controller proposal with legacy-mapped safe_intent
        self.ctrl.step_with_safety(eco_state, residual)
    }
}
