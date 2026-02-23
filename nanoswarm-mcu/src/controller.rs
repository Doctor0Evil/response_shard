impl<K> SafeController<K> for MyAsyncCtrl<K>
where
    K: CyboquaticSafetyKernel,
{
    fn kernel(&self) -> &K { &self.kernel }

    fn propose_control(&mut self, state: &AquaticState) -> FlowControl {
        // compute candidate from PID / MPC / SNN-advised logic
    }
}
