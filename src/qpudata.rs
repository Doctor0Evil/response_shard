use crate::types::Residual;

#[derive(Clone, Debug)]
pub struct QpuSafetyRow {
    pub node_id: u32,
    pub t_ms: u64,
    pub vt: f32,
    pub r_tdi: f32,
    pub r_mbi: f32,
    pub r_eis: f32,
    pub r_rad: f32,
    pub knowledge_factor: f32,
    pub eco_impact: f32,
    pub risk_of_harm: f32,
    pub hexstamp: &'static str,
}

pub fn residual_to_row(
    node_id: u32,
    t_ms: u64,
    res: &Residual,
    k: f32,
    e: f32,
    r: f32,
) -> QpuSafetyRow {
    let r_tdi = res.rx.get(0).map(|rc| rc.value).unwrap_or(0.0);
    let r_mbi = res.rx.get(1).map(|rc| rc.value).unwrap_or(0.0);
    let r_eis = res.rx.get(2).map(|rc| rc.value).unwrap_or(0.0);
    let r_rad = res.rx.get(3).map(|rc| rc.value).unwrap_or(0.0);

    QpuSafetyRow {
        node_id,
        t_ms,
        vt: res.vt,
        r_tdi,
        r_mbi,
        r_eis,
        r_rad,
        knowledge_factor: k,
        eco_impact: e,
        risk_of_harm: r,
        hexstamp: "0x6e616e6f737761726d5f7361666574795f6b65726e656c",
    }
}
