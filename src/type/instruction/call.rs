//! Original PTX specification:
//!
//! // direct call to named function, func is a symbol
//! call{.uni} (ret-param), func, (param-list);
//! call{.uni} func, (param-list);
//! call{.uni} func;
//! // indirect call via pointer, with full list of call targets
//! call{.uni} (ret-param), fptr, (param-list), flist;
//! call{.uni} fptr, (param-list), flist;
//! call{.uni} fptr, flist;
//! // indirect call via pointer, with no knowledge of call targets
//! call{.uni} (ret-param), fptr, (param-list), fproto;
//! call{.uni} fptr, (param-list), fproto;
//! call{.uni} fptr, fproto;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni {
        pub uni: bool, // {.uni}
        pub ret_param: Operand, // (ret-param)
        pub func: Operand, // func
        pub param_list: Vec<Operand>, // (param-list)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni1 {
        pub uni: bool, // {.uni}
        pub func: Operand, // func
        pub param_list: Vec<Operand>, // (param-list)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni2 {
        pub uni: bool, // {.uni}
        pub func: Operand, // func
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni3 {
        pub uni: bool, // {.uni}
        pub ret_param: Operand, // (ret-param)
        pub fptr: Operand, // fptr
        pub param_list: Vec<Operand>, // (param-list)
        pub flist: Operand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni4 {
        pub uni: bool, // {.uni}
        pub fptr: Operand, // fptr
        pub param_list: Vec<Operand>, // (param-list)
        pub flist: Operand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni5 {
        pub uni: bool, // {.uni}
        pub fptr: Operand, // fptr
        pub flist: Operand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni6 {
        pub uni: bool, // {.uni}
        pub ret_param: Operand, // (ret-param)
        pub fptr: Operand, // fptr
        pub param_list: Vec<Operand>, // (param-list)
        pub fproto: Operand, // fproto
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni7 {
        pub uni: bool, // {.uni}
        pub fptr: Operand, // fptr
        pub param_list: Vec<Operand>, // (param-list)
        pub fproto: Operand, // fproto
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni8 {
        pub uni: bool, // {.uni}
        pub fptr: Operand, // fptr
        pub fproto: Operand, // fproto
    }

}
