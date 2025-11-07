//! Original PTX specification:
//!
//! stmatrix.sync.aligned.shape.num{.trans}{.ss}.type [p], r;
//! .shape  = {.m8n8, .m16n8};
//! .num    = {.x1, .x2, .x4};
//! .ss     = {.shared, .shared::cta};
//! .type   = {.b16, .b8};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::stmatrix::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Num {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try X1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x1").is_ok() {
                    return Ok(Num::X1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try X2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x2").is_ok() {
                    return Ok(Num::X2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x4").is_ok() {
                    return Ok(Num::X4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".x1", ".x2", ".x4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8").is_ok() {
                    return Ok(Shape::M16n8);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M8n8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8").is_ok() {
                    return Ok(Shape::M8n8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8", ".m8n8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Ss::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta", ".shared"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Type::B16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Type::B8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b16", ".b8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for StmatrixSyncAlignedShapeNumTransSsType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("stmatrix")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let num = Num::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let trans = stream.expect_string(".trans").is_ok();
            if !trans {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let p = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let r = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(StmatrixSyncAlignedShapeNumTransSsType {
                sync,
                aligned,
                shape,
                num,
                trans,
                ss,
                type_,
                p,
                r,
            })
        }
    }


}

