//! Original PTX specification:
//!
//! lop3.b32 d, a, b, c, immLut;
//! lop3.BoolOp.b32 d|p, a, b, c, immLut, q;
//! .BoolOp   = { .or , .and };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::lop3::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Boolop::Or);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Boolop::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".or", ".and"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Lop3B32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("lop3")?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immlut = Operand::parse(stream)?;
            Ok(Lop3B32 {
                b32,
                d,
                a,
                b,
                c,
                immlut,
            })
        }
    }


    impl PtxParser for Lop3BoolopB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("lop3")?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let p = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immlut = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let q = Operand::parse(stream)?;
            Ok(Lop3BoolopB32 {
                boolop,
                b32,
                d,
                p,
                a,
                b,
                c,
                immlut,
                q,
            })
        }
    }


}

