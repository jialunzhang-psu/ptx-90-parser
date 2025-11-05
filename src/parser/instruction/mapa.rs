//! Original PTX specification:
//!
//! mapa{.space}.type          d, a, b;
//! // Maps shared memory address in register a into CTA b.
//! // mapa.shared::cluster.type  d, a, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh + imm, b;
//! // Maps generic address in register a into CTA b.
//! // mapa.type                  d, a, b;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mapa::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".u64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cluster"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MapaSpaceType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mapa")?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(MapaSpaceType {
                space,
                type_,
                d,
                a,
                b,
            })
        }
    }


}

