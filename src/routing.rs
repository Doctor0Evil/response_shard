use crate::board_hal::NanoswarmBoard;
use crate::kernel::{evaluate_node, KernelParams, NodeState};
use crate::types::CorridorBands;
use crate::voxel::{Lifeforce5DVoxel, LifeForm, SafetyEnvelope};
use async_std::task;
use core::time::Duration;

pub async fn run_safety_loop<B: NanoswarmBoard + Send>(
    mut board: B,
    params: KernelParams,
    bands_tdi: CorridorBands,
    bands_mbi: CorridorBands,
    bands_eis: CorridorBands,
    bands_rad: CorridorBands,
) -> ! {
    let env = SafetyEnvelope {
        lifeform: LifeForm::None,
        max_tdi: bands_tdi.safe,
        min_mbi: bands_mbi.safe,
        max_eis: bands_eis.safe,
        max_rad_index: bands_rad.safe,
        max_residence: 100,
    };

    let mut prev_state = {
        let voxel = Lifeforce5DVoxel {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdi: 0.0,
            mbi: 1.0,
            eis: 0.0,
            rad_index: 0.0,
            residence: 0,
            envelope: env,
        };
        let residual = voxel.to_risk_coords(&bands_tdi, &bands_mbi, &bands_eis, &bands_rad);
        NodeState {
            node_id: 0,
            duty_cycle: 0.0,
            voxel,
            residual,
        }
    };

    loop {
        let tdi = board.read_tdi();
        let mbi = board.read_mbi();
        let eis = board.read_eis();
        let rad = board.read_rad_index();

        let voxel = Lifeforce5DVoxel {
            x: prev_state.voxel.x,
            y: prev_state.voxel.y,
            z: prev_state.voxel.z,
            tdi,
            mbi,
            eis,
            rad_index: rad,
            residence: prev_state.voxel.residence.saturating_add(1),
            envelope: env,
        };

        let residual = voxel.to_risk_coords(&bands_tdi, &bands_mbi, &bands_eis, &bands_rad);

        let proposed = NodeState {
            node_id: prev_state.node_id,
            duty_cycle: prev_state.duty_cycle, // planner may adjust upstream
            voxel,
            residual,
        };

        let decision = evaluate_node(&prev_state, proposed.clone(), &params);

        board.apply_duty(decision.safe_duty);

        prev_state = NodeState {
            duty_cycle: decision.safe_duty,
            ..proposed
        };

        task::sleep(Duration::from_millis(100)).await;
    }
}
