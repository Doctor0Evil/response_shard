#![forbid(unsafe_code)]

use nanoswarm_safety_kernel::routing::run_safety_loop;
use nanoswarm_safety_kernel::kernel::KernelParams;
use nanoswarm_safety_kernel::types::CorridorBands;

struct PhoenixBoard { /* embedded-hal impl fields */ }

impl nanoswarm_safety_kernel::board_hal::NanoswarmBoard for PhoenixBoard {
    type Adc = ();
    type DutyPin = ();

    fn adc(&mut self) -> &mut Self::Adc { unimplemented!() }
    fn duty_pin(&mut self) -> &mut Self::DutyPin { unimplemented!() }

    fn read_tdi(&mut self) -> f32 { 0.0 }  // hook real sensors
    fn read_mbi(&mut self) -> f32 { 1.0 }
    fn read_eis(&mut self) -> f32 { 0.0 }
    fn read_rad_index(&mut self) -> f32 { 0.0 }

    fn apply_duty(&mut self, _duty: f32) {
        // write PWM / GPIO
    }
}

#[async_std::main]
async fn main() {
    let bands_tdi = CorridorBands {
        var_id: "TDI",
        units: "ºC_dev",
        safe: 0.05,
        gold: 0.10,
        hard: 0.20,
        weight: 0.3,
        lyap_channel: 0,
    };
    let bands_mbi = CorridorBands {
        var_id: "MBI",
        units: "unitless",
        safe: 0.8,
        gold: 0.7,
        hard: 0.5,
        weight: 0.3,
        lyap_channel: 1,
    };
    let bands_eis = CorridorBands {
        var_id: "EIS",
        units: "unitless",
        safe: 0.10,
        gold: 0.20,
        hard: 0.40,
        weight: 0.2,
        lyap_channel: 2,
    };
    let bands_rad = CorridorBands {
        var_id: "RAD",
        units: "norm",
        safe: 0.10,
        gold: 0.20,
        hard: 0.40,
        weight: 0.2,
        lyap_channel: 3,
    };

    let params = KernelParams {
        eta_mass: 1.0,
        eta_eco: 1.0,
        eta_bee: 1.0,
    };

    let board = PhoenixBoard { /* ... */ };

    run_safety_loop(board, params, bands_tdi, bands_mbi, bands_eis, bands_rad).await;
}
