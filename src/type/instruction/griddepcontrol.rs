//! Original PTX specification:
//!
//! griddepcontrol.action;
//! .action   = { .launch_dependents, .wait };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Action {
        LaunchDependents, // .launch_dependents
        Wait, // .wait
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct GriddepcontrolAction {
        pub action: Action, // .action
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::GriddepcontrolAction;
pub use section_0::Action as Action0;
