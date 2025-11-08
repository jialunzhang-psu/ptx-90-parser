//! Original PTX specification:
//!
//! clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 pred, try_cancel_response;
//! clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;
//! clusterlaunchcontrol.query_cancel{.get_first_ctaid::dimension}.b32.b128 reg, try_cancel_response;
//! .get_first_ctaid::dimension = { .get_first_ctaid::x, .get_first_ctaid::y, .get_first_ctaid::z };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for GetFirstCtaidDimension {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try GetFirstCtaidX
            {
                let saved_pos = stream.position();
                if stream.expect_string(".get_first_ctaid::x").is_ok() {
                    return Ok(GetFirstCtaidDimension::GetFirstCtaidX);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try GetFirstCtaidY
            {
                let saved_pos = stream.position();
                if stream.expect_string(".get_first_ctaid::y").is_ok() {
                    return Ok(GetFirstCtaidDimension::GetFirstCtaidY);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try GetFirstCtaidZ
            {
                let saved_pos = stream.position();
                if stream.expect_string(".get_first_ctaid::z").is_ok() {
                    return Ok(GetFirstCtaidDimension::GetFirstCtaidZ);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".get_first_ctaid::x",
                ".get_first_ctaid::y",
                ".get_first_ctaid::z",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("clusterlaunchcontrol")?;
            stream.expect_string(".query_cancel")?;
            let query_cancel = ();
            stream.expect_complete()?;
            stream.expect_string(".is_canceled")?;
            let is_canceled = ();
            stream.expect_complete()?;
            stream.expect_string(".pred")?;
            let pred = ();
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let pred2 = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let try_cancel_response = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
                query_cancel,
                is_canceled,
                pred,
                b128,
                pred2,
                try_cancel_response,
            })
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("clusterlaunchcontrol")?;
            stream.expect_string(".query_cancel")?;
            let query_cancel = ();
            stream.expect_complete()?;
            stream.expect_string(".get_first_ctaid")?;
            let get_first_ctaid = ();
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let xdim = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let try_cancel_response = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
                query_cancel,
                get_first_ctaid,
                v4,
                b32,
                b128,
                xdim,
                try_cancel_response,
            })
        }
    }

    impl PtxParser for ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("clusterlaunchcontrol")?;
            stream.expect_string(".query_cancel")?;
            let query_cancel = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let get_first_ctaid_dimension = match GetFirstCtaidDimension::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let reg = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let try_cancel_response = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
                    query_cancel,
                    get_first_ctaid_dimension,
                    b32,
                    b128,
                    reg,
                    try_cancel_response,
                },
            )
        }
    }
}
