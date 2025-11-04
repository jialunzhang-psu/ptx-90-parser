//! Original PTX specification:
//!
//! ret{.uni};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct RetUni {
        pub uni: bool, // {.uni}
    }

}
