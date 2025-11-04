//! Original PTX specification:
//!
//! setmaxnreg.action.sync.aligned.u32 imm-reg-count;
//! .action = { .inc, .dec };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Action {
        Inc, // .inc
        Dec, // .dec
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetmaxnregActionSyncAlignedU32 {
        pub action: Action, // .action
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub u32: (), // .u32
        pub imm_reg_count: Operand, // imm-reg-count
    }

}
