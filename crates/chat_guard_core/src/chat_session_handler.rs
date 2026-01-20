use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct KerTriple {
    pub k: f32, // knowledge-factor 0–1
    pub e: f32, // eco-impact 0–1
    pub r: f32, // risk-of-harm 0–1
}

#[derive(Clone, Debug)]
pub struct CorridorVector {
    // Normalized risk coordinates rx in [0,1]
    pub r_heat: f32,
    pub r_toxic: f32,
    pub r_neuro: f32,
    pub r_governance: f32,
}

#[derive(Clone, Debug)]
pub struct ResidualState {
    pub vt_before: f32,
    pub vt_after: f32,
}

#[derive(Clone, Debug)]
pub struct CorridorFlags {
    pub corridor_ok: bool, // all rx < 1
    pub legal_ok: bool,    // legal corridors satisfied
    pub gold_ok: bool,     // science/ethics gold bands satisfied (for bonus / scale-up)
}

#[derive(Clone, Debug)]
pub struct ChatSessionParticle {
    pub session_id: String,
    pub creator_did: String,
    pub vault_id: String,
    pub artifact_refs: Vec<(String, String)>, // (shard_id, role)
    pub chat_quanta_used: u64,
    pub ker: KerTriple,
    pub rx: CorridorVector,
    pub residual: ResidualState,
    pub corridor_flags: CorridorFlags,
    pub dev_tunnel_label: String,
    pub hex_stamp: String,
    pub created_at: SystemTime,
}

#[derive(Clone, Debug)]
pub struct InboundRequestContext {
    pub caller_did: String,
    pub vault_id: String,
    pub topic_tags: Vec<String>,
    pub artifact_refs: Vec<(String, String)>,
    pub prior_vt: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionDecision {
    Allow,
    Derate,
    Reject,
}

#[derive(Clone, Debug)]
pub struct SessionDecisionOutcome {
    pub decision: SessionDecision,
    pub particle: ChatSessionParticle,
}
