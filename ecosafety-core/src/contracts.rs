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
