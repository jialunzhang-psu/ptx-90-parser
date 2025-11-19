//! Original PTX specification:
//!
//! clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 pred, try_cancel_response;
//! clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;
//! clusterlaunchcontrol.query_cancel{.get_first_ctaid::dimension}.b32.b128 reg, try_cancel_response;
//! .get_first_ctaid::dimension = { .get_first_ctaid::x, .get_first_ctaid::y, .get_first_ctaid::z };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum GetFirstCtaidDimension {
        GetFirstCtaidX, // .get_first_ctaid::x
        GetFirstCtaidY, // .get_first_ctaid::y
        GetFirstCtaidZ, // .get_first_ctaid::z
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
        pub query_cancel: (),                    // .query_cancel
        pub is_canceled: (),                     // .is_canceled
        pub pred: (),                            // .pred
        pub b128: (),                            // .b128
        pub pred2: GeneralOperand,               // pred
        pub try_cancel_response: GeneralOperand, // try_cancel_response
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
        pub query_cancel: (),                    // .query_cancel
        pub get_first_ctaid: (),                 // .get_first_ctaid
        pub v4: (),                              // .v4
        pub b32: (),                             // .b32
        pub b128: (),                            // .b128
        pub xdim: VectorOperand,                 // {xdim, ydim, zdim, _}
        pub try_cancel_response: GeneralOperand, // try_cancel_response
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
        pub query_cancel: (),                                          // .query_cancel
        pub get_first_ctaid_dimension: Option<GetFirstCtaidDimension>, // {.get_first_ctaid::dimension}
        pub b32: (),                                                   // .b32
        pub b128: (),                                                  // .b128
        pub reg: GeneralOperand,                                       // reg
        pub try_cancel_response: GeneralOperand,                       // try_cancel_response
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128;
pub use section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128;
pub use section_0::ClusterlaunchcontrolQueryCancelIsCanceledPredB128;
pub use section_0::GetFirstCtaidDimension as GetFirstCtaidDimension0;
