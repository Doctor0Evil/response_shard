use crate::{ChatResponseShardV1, ChatSessionParticleV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatKerCheck {
    Ok,
    SessionKerMalformed,
    ResponseKerMalformed,
    RiskExceedsSessionLimit,
}

pub fn check_response_against_session(
    session: &ChatSessionParticleV1,
    response: &ChatResponseShardV1,
) -> ChatKerCheck {
    if !session.is_well_formed() {
        return ChatKerCheck::SessionKerMalformed;
    }
    if !response.is_well_formed() {
        return ChatKerCheck::ResponseKerMalformed;
    }

    if response.ker.r > session.max_risk_of_harm_allowed + 1e-9 {
        return ChatKerCheck::RiskExceedsSessionLimit;
    }

    ChatKerCheck::Ok
}
