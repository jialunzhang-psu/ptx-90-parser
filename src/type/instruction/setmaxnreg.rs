use crate::r#type::common::Immediate;

/// `setmaxnreg.action.sync.aligned.u32 imm-reg-count;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setmaxnreg {
    /// `.action`
    pub action: Action,
    /// `imm-reg-count`
    pub register_count: Immediate,
}

/// `.action = { .inc, .dec };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// `.inc`
    Inc,
    /// `.dec`
    Dec,
}
