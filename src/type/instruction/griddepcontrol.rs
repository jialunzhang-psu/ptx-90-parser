//! Original PTX specification:
//!
//! griddepcontrol.action;
//! .action   = { .launch_dependents, .wait };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Action {
        LaunchDependents, // .launch_dependents
        Wait, // .wait
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GriddepcontrolAction {
        pub action: Action, // .action
    }

}
