//@ requires prev.vt >= 0.0
//@ requires forall i: 0..prev.coords.len() => prev.coords[i].r >= 0.0 && prev.coords[i].r <= 1.0
//@ requires forall i: 0..next.coords.len() => next.coords[i].r >= 0.0
//@ ensures  result == CorridorDecision::Stop ==> exists j: 0..next.coords.len() => next.coords[j].r > 1.0
//@ ensures  result == CorridorDecision::Ok ==> next.vt <= prev.vt
pub fn safe_step(prev: &Residual, next: &Residual) -> CorridorDecision { ... }

pub enum CorridorDecision {
    Ok,
    Derate,
    Stop,
}

pub fn safe_step(prev: &Residual, next: &Residual) -> CorridorDecision {
    let any_hard = next.coords.iter().any(|c| c.r > 1.0);
    if any_hard {
        return CorridorDecision::Stop;
    }
    if next.vt > prev.vt {
        return CorridorDecision::Derate;
    }
    CorridorDecision::Ok
}
