//! Original PTX specification:
//!
//! testp.op.type  p, a;  // result is .pred
//! .op   = { .finite, .infinite,
//! .number, .notanumber,
//! .normal, .subnormal };
//! .type = { .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::testp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Finite
            {
                let saved_pos = stream.position();
                if stream.expect_string(".finite").is_ok() {
                    return Ok(Op::Finite);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Infinite
            {
                let saved_pos = stream.position();
                if stream.expect_string(".infinite").is_ok() {
                    return Ok(Op::Infinite);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Number
            {
                let saved_pos = stream.position();
                if stream.expect_string(".number").is_ok() {
                    return Ok(Op::Number);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Notanumber
            {
                let saved_pos = stream.position();
                if stream.expect_string(".notanumber").is_ok() {
                    return Ok(Op::Notanumber);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Normal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".normal").is_ok() {
                    return Ok(Op::Normal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Subnormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".subnormal").is_ok() {
                    return Ok(Op::Subnormal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".finite", ".infinite", ".number", ".notanumber", ".normal", ".subnormal"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Type::F32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Type::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f32", ".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TestpOpType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("testp")?;
            let op = Op::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let p = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            Ok(TestpOpType {
                op,
                type_,
                p,
                a,
            })
        }
    }


}

