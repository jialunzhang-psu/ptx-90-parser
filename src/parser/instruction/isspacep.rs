//! Original PTX specification:
//!
//! isspacep.space  p, a;    // result is .pred
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::isspacep::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Space {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cluster").is_ok() {
                    return Ok(Space::SharedCluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ParamEntry
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param::entry").is_ok() {
                    return Ok(Space::ParamEntry);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Space::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Space::Global);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Space::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Const
            {
                let saved_pos = stream.position();
                if stream.expect_string(".const").is_ok() {
                    return Ok(Space::Const);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Local
            {
                let saved_pos = stream.position();
                if stream.expect_string(".local").is_ok() {
                    return Ok(Space::Local);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Param
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param").is_ok() {
                    return Ok(Space::Param);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cluster", ".param::entry", ".shared::cta", ".global", ".shared", ".const", ".local", ".param"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for IsspacepSpace {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("isspacep")?;
            let space = Space::parse(stream)?;
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(IsspacepSpace {
                space,
                p,
                a,
            })
        }
    }


}

