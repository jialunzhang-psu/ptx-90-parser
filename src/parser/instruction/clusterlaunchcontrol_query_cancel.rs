//! Original PTX specification:
//!
//! clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 pred, try_cancel_response;
//! clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;
//! clusterlaunchcontrol.query_cancel{.get_first_ctaid::dimension}.b32.b128 reg, try_cancel_response;
//! .get_first_ctaid::dimension = { .get_first_ctaid::x, .get_first_ctaid::y, .get_first_ctaid::z };

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for GetFirstCtaidDimension {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".get_first_ctaid::x"), |_, _span| {
                    GetFirstCtaidDimension::GetFirstCtaidX
                }),
                map(string_p(".get_first_ctaid::y"), |_, _span| {
                    GetFirstCtaidDimension::GetFirstCtaidY
                }),
                map(string_p(".get_first_ctaid::z"), |_, _span| {
                    GetFirstCtaidDimension::GetFirstCtaidZ
                })
            )
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("clusterlaunchcontrol"),
                    string_p(".query_cancel"),
                    string_p(".is_canceled"),
                    string_p(".pred"),
                    string_p(".b128"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, query_cancel, is_canceled, pred, b128, pred2, _, try_cancel_response, _),
                 span| {
                    ok!(ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
                        query_cancel = query_cancel,
                        is_canceled = is_canceled,
                        pred = pred,
                        b128 = b128,
                        pred2 = pred2,
                        try_cancel_response = try_cancel_response,

                    })
                },
            )
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("clusterlaunchcontrol"),
                    string_p(".query_cancel"),
                    string_p(".get_first_ctaid"),
                    string_p(".v4"),
                    string_p(".b32"),
                    string_p(".b128"),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    query_cancel,
                    get_first_ctaid,
                    v4,
                    b32,
                    b128,
                    xdim,
                    _,
                    try_cancel_response,
                    _,
                ),
                 span| {
                    ok!(ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
                        query_cancel = query_cancel,
                        get_first_ctaid = get_first_ctaid,
                        v4 = v4,
                        b32 = b32,
                        b128 = b128,
                        xdim = xdim,
                        try_cancel_response = try_cancel_response,

                    })
                },
            )
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("clusterlaunchcontrol"),
                    string_p(".query_cancel"),
                    optional(GetFirstCtaidDimension::parse()),
                    string_p(".b32"),
                    string_p(".b128"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    query_cancel,
                    get_first_ctaid_dimension,
                    b32,
                    b128,
                    reg,
                    _,
                    try_cancel_response,
                    _,
                ),
                 span| {
                    ok!(ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
                        query_cancel = query_cancel,
                        get_first_ctaid_dimension = get_first_ctaid_dimension,
                        b32 = b32,
                        b128 = b128,
                        reg = reg,
                        try_cancel_response = try_cancel_response,

                    })
                },
            )
        }
    }
}
