//! Original PTX specification:
//!
//! ex2.approx{.ftz}.f32  d, a;
//! 
//! ex2.approx.atype     d, a;
//! ex2.approx.ftz.btype d, a;
//! .atype = { .f16,  .f16x2};
//! .btype = { .bf16, .bf16x2};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ex2::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16x2").is_ok() {
                    return Ok(Atype::F16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Atype::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16x2", ".f16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16x2").is_ok() {
                    return Ok(Btype::Bf16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Btype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16x2", ".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ex2ApproxFtzF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ex2")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Ex2ApproxFtzF32 {
                approx,
                ftz,
                f32,
                d,
                a,
            })
        }
    }


    impl PtxParser for Ex2ApproxAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ex2")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Ex2ApproxAtype {
                approx,
                atype,
                d,
                a,
            })
        }
    }


    impl PtxParser for Ex2ApproxFtzBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ex2")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_complete()?;
            stream.expect_string(".ftz")?;
            let ftz = ();
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Ex2ApproxFtzBtype {
                approx,
                ftz,
                btype,
                d,
                a,
            })
        }
    }


}

