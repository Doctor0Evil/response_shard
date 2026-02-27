#![no_std]  // optional, but recommended for embedded
extern crate alloc;

use alloc::vec::Vec;
use core::ffi::c_double;
use core::ffi::c_int;
use core::ffi::c_uint;

use ecosafety_core::{
    CorridorBands, RiskCoord, Residual,
    MetricFields, CorridorDecision, ResidualCheck,
    normalize_metric, safe_step, residual_ok,
};

#[repr(C)]
pub struct FfiBands {
    pub safe: c_double,
    pub gold: c_double,
    pub hard: c_double,
    pub weight: c_double,
}

#[repr(C)]
pub struct FfiResidualInput {
    pub prev_vt: c_double,
    pub next_vt: c_double,
}

#[repr(C)]
pub struct FfiDecision {
    // 0 = Ok, 1 = Derate, 2 = Stop
    pub decision_code: c_int,
    // 0 = Ok, 1 = ViolatedAxis, 2 = IncreasedResidual, -1 = NotChecked
    pub residual_check: c_int,
}

/// Map C_int code back to CorridorDecision inside Rust if needed.
fn decision_to_code(d: CorridorDecision) -> c_int {
    match d {
        CorridorDecision::Ok => 0,
        CorridorDecision::Derate => 1,
        CorridorDecision::Stop => 2,
    }
}

fn residual_check_to_code(c: ResidualCheck) -> c_int {
    match c {
        ResidualCheck::Ok => 0,
        ResidualCheck::ViolatedAxis => 1,
        ResidualCheck::IncreasedResidual => 2,
    }
}

/// Pure helper: compute a single risk coordinate r in [0,1] from a value and bands.
/// Lua can call this to inspect rx without touching kernels.
#[no_mangle]
pub extern "C" fn ecosafety_normalize_metric(
    value: c_double,
    safe: c_double,
    gold: c_double,
    hard: c_double,
    weight: c_double,
) -> c_double {
    let bands = CorridorBands {
        var_id: "",
        units: "",
        safe,
        gold,
        hard,
        weight,
        lyap_channel: 0,
    };
    let rc: RiskCoord = normalize_metric(value, &bands);
    rc.r
}

/// Core gate: check a single-step transition using the shared safe_step kernel.
/// Lua passes current and proposed values already normalized to Residual.vt and rx.
#[no_mangle]
pub extern "C" fn ecosafety_safe_step_decision(
    prev_vt: c_double,
    next_vt: c_double,
    // number of coordinates, and pointers to their rx in [0,1]
    len: c_uint,
    prev_rx_ptr: *const c_double,
    next_rx_ptr: *const c_double,
) -> FfiDecision {
    // Safety: if pointers or length are invalid, fail closed.
    if prev_rx_ptr.is_null() || next_rx_ptr.is_null() || len == 0 {
        return FfiDecision { decision_code: 2, residual_check: 1 }; // Stop + ViolatedAxis
    }

    // Build minimal Residuals from rx only; bands are not needed for safe_step
    let mut prev_coords: Vec<RiskCoord> = Vec::new();
    let mut next_coords: Vec<RiskCoord> = Vec::new();

    for i in 0..len {
        unsafe {
            let pr = *prev_rx_ptr.add(i as usize);
            let nr = *next_rx_ptr.add(i as usize);

            // Hard defensive: if any rx is out of [0,1], treat as axis violation.
            if !(0.0..=1.0).contains(&pr) || !(0.0..=1.0).contains(&nr) {
                return FfiDecision {
                    decision_code: 2,          // Stop
                    residual_check: 1,         // ViolatedAxis
                };
            }

            let dummy_bands = CorridorBands {
                var_id: "",
                units: "",
                safe: 0.0,
                gold: 0.0,
                hard: 1.0,
                weight: 1.0,
                lyap_channel: 0,
            };
            prev_coords.push(RiskCoord { r: pr, sigma: 0.0, bands: dummy_bands });
            next_coords.push(RiskCoord { r: nr, sigma: 0.0, bands: dummy_bands });
        }
    }

    let prev = Residual { vt: prev_vt, coords: prev_coords };
    let next = Residual { vt: next_vt, coords: next_coords };

    let decision = safe_step(&prev, &next);
    FfiDecision { decision_code: decision_to_code(decision), residual_check: -1 }
}

/// Stronger gate: enforce both rx-axis limits and Lyapunov residual_ok on MetricFields.
/// This is what Lua should use for any “is this step allowed?” query.
#[no_mangle]
pub extern "C" fn ecosafety_residual_guard(
    // previous metrics
    prev_k: c_double,
    prev_e: c_double,
    prev_r: c_double,
    prev_vt: c_double,
    len: c_uint,
    prev_rx_ptr: *const c_double,
    // next metrics
    next_k: c_double,
    next_e: c_double,
    next_r: c_double,
    next_vt: c_double,
    next_rx_ptr: *const c_double,
) -> FfiDecision {
    if prev_rx_ptr.is_null() || next_rx_ptr.is_null() || len == 0 {
        return FfiDecision { decision_code: 2, residual_check: 1 };
    }

    unsafe {
        let mut prev_rx = Vec::with_capacity(len as usize);
        let mut next_rx = Vec::with_capacity(len as usize);

        for i in 0..len {
            let pr = *prev_rx_ptr.add(i as usize);
            let nr = *next_rx_ptr.add(i as usize);

            if !(0.0..=1.0).contains(&pr) || !(0.0..=1.0).contains(&nr) {
                return FfiDecision { decision_code: 2, residual_check: 1 };
            }
            prev_rx.push(pr);
            next_rx.push(nr);
        }

        let prev = MetricFields { k: prev_k, e: prev_e, r: prev_r, rx: prev_rx, vt: prev_vt };
        let next = MetricFields { k: next_k, e: next_e, r: next_r, rx: next_rx, vt: next_vt };

        if !prev.is_well_formed() || !next.is_well_formed() {
            return FfiDecision { decision_code: 2, residual_check: 1 };
        }

        let check = residual_ok(&prev, &next);
        let decision = match check {
            ResidualCheck::Ok => CorridorDecision::Ok,
            ResidualCheck::ViolatedAxis => CorridorDecision::Stop,
            ResidualCheck::IncreasedResidual => CorridorDecision::Derate,
        };

        FfiDecision {
            decision_code: decision_to_code(decision),
            residual_check: residual_check_to_code(check),
        }
    }
}
