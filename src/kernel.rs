use crate::types::{CorridorDecision, Residual, safe_step};
use crate::voxel::Lifeforce5DVoxel;

#[derive(Clone, Debug)]
pub struct NodeState {
    pub node_id: u32,
    pub duty_cycle: f32,          // 0..1
    pub voxel: Lifeforce5DVoxel,  // current 5D state
    pub residual: Residual,       // last computed
}

#[derive(Clone, Debug)]
pub struct KernelParams {
    pub eta_mass: f32,
    pub eta_eco: f32,
    pub eta_bee: f32,
}

#[derive(Clone, Debug)]
pub struct KernelDecision {
    pub safe_duty: f32,
    pub permitted: bool,
    pub decision: CorridorDecision,
}

pub fn evaluate_node(
    prev: &NodeState,
    mut proposed: NodeState,
    params: &KernelParams,
) -> KernelDecision {
    // Clamp proposed duty
    let mut duty = proposed.duty_cycle.clamp(0.0, 1.0);

    // Recompute residual for proposed state (voxel already updated by caller)
    let mut next_res = proposed.residual.clone();
    next_res.recompute();

    let dec = safe_step(&prev.residual, &next_res);

    if dec.stop {
        duty = 0.0;
    } else if dec.derate {
        duty *= 0.5;
    }

    KernelDecision {
        safe_duty: duty,
        permitted: !dec.stop,
        decision: dec,
    }
}
