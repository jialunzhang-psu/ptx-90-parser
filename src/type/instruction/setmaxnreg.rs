//! Original PTX specification:
//!
//! setmaxnreg.action.sync.aligned.u32 imm-reg-count;
//! .action = { .inc, .dec };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Action {
        Inc, // .inc
        Dec, // .dec
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct SetmaxnregActionSyncAlignedU32 {
        pub action: Action,                // .action
        pub sync: (),                      // .sync
        pub aligned: (),                   // .aligned
        pub u32: (),                       // .u32
        pub imm_reg_count: GeneralOperand, // imm-reg-count
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Action as Action0;
pub use section_0::SetmaxnregActionSyncAlignedU32;
