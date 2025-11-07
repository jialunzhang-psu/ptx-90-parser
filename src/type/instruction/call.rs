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
        pub ret_param: GeneralOperand, // (ret-param)
        pub func: GeneralOperand, // func
        pub param_list: Vec<GeneralOperand>, // (param-list)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni1 {
        pub uni: bool, // {.uni}
        pub func: GeneralOperand, // func
        pub param_list: Vec<GeneralOperand>, // (param-list)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni2 {
        pub uni: bool, // {.uni}
        pub func: GeneralOperand, // func
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni3 {
        pub uni: bool, // {.uni}
        pub ret_param: GeneralOperand, // (ret-param)
        pub fptr: GeneralOperand, // fptr
        pub param_list: Vec<GeneralOperand>, // (param-list)
        pub flist: GeneralOperand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni4 {
        pub uni: bool, // {.uni}
        pub fptr: GeneralOperand, // fptr
        pub param_list: Vec<GeneralOperand>, // (param-list)
        pub flist: GeneralOperand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni5 {
        pub uni: bool, // {.uni}
        pub fptr: GeneralOperand, // fptr
        pub flist: GeneralOperand, // flist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni6 {
        pub uni: bool, // {.uni}
        pub ret_param: GeneralOperand, // (ret-param)
        pub fptr: GeneralOperand, // fptr
        pub param_list: Vec<GeneralOperand>, // (param-list)
        pub fproto: GeneralOperand, // fproto
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni7 {
        pub uni: bool, // {.uni}
        pub fptr: GeneralOperand, // fptr
        pub param_list: Vec<GeneralOperand>, // (param-list)
        pub fproto: GeneralOperand, // fproto
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CallUni8 {
        pub uni: bool, // {.uni}
        pub fptr: GeneralOperand, // fptr
        pub fproto: GeneralOperand, // fproto
    }

}
