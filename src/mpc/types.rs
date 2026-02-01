pub struct MpcStateView<'a> {
    // Core physical / thermal
    pub wbgt_index: f64,
    pub ambient_temp_c: f64,
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,

    // Power / exergy / degradation
    pub current_power_watts: f64,
    pub exergy_destruction_joules: f64,
    pub degradation_accumulated: f64,

    // Queue / utilization
    pub queue_length: u32,
    pub cpu_utilization: f64,
    pub gpu_utilization: f64,
    pub net_latency_ms: f64,

    // KER / residuals
    pub r_vec: Vec<f64>, // normalized rx (0–1) pulled from shard corridors
    pub vt: f64,         // node residual
}
