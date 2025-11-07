//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem],  a-desc,  b-desc,  idesc,
//! enable-input-d {, zero-column-mask-desc };
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc };
//! .kind = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! ----------------------------------------------------------------------------------
//! // 2. Integer type:
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem],  a-desc,  b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .collector_usage = { .collector::buffer::op };
//! ::buffer = { ::b0, ::b1, ::b2, ::b3 };
//! ::op   = { ::fill, ::use, ::lastuse, ::discard};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma_ws::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindF8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f8f6f4").is_ok() {
                    return Ok(Kind::KindF8f6f4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try KindTf32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::tf32").is_ok() {
                    return Ok(Kind::KindTf32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try KindF16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f16").is_ok() {
                    return Ok(Kind::KindF16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::f8f6f4", ".kind::tf32", ".kind::f16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaWsCtaGroup1KindCollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_complete()?;
            stream.expect_string(".ws")?;
            let ws = ();
            stream.expect_complete()?;
            stream.expect_string(".cta_group::1")?;
            let cta_group_1 = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let collector_usage = stream.expect_string(".collector_usage").is_ok();
            if !collector_usage {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let zero_column_mask_desc = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05MmaWsCtaGroup1KindCollectorUsage {
                mma,
                ws,
                cta_group_1,
                kind,
                collector_usage,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                enable_input_d,
                zero_column_mask_desc,
            })
        }
    }


    impl PtxParser for Tcgen05MmaWsCtaGroup1KindCollectorUsage1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_complete()?;
            stream.expect_string(".ws")?;
            let ws = ();
            stream.expect_complete()?;
            stream.expect_string(".cta_group::1")?;
            let cta_group_1 = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let collector_usage = stream.expect_string(".collector_usage").is_ok();
            if !collector_usage {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let zero_column_mask_desc = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05MmaWsCtaGroup1KindCollectorUsage1 {
                mma,
                ws,
                cta_group_1,
                kind,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                enable_input_d,
                zero_column_mask_desc,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma_ws::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Buffer {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string("::b0").is_ok() {
                    return Ok(Buffer::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string("::b1").is_ok() {
                    return Ok(Buffer::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B2
            {
                let saved_pos = stream.position();
                if stream.expect_string("::b2").is_ok() {
                    return Ok(Buffer::B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string("::b3").is_ok() {
                    return Ok(Buffer::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::b0", "::b1", "::b2", "::b3"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CollectorBufferOp
            {
                let saved_seq_pos = stream.position();
                match (|| -> Result<_, PtxParseError> {
            stream.expect_string(".collector")?;
            let collector = ();
            let buffer = Buffer::parse(stream)?;
            let op = Op::parse(stream)?;
                    Ok((collector, buffer, op))
                })() {
                    Ok((collector, buffer, op)) => {
                        return Ok(CollectorUsage::CollectorBufferOp(collector, buffer, op));
                    }
                    Err(_) => {
                        stream.set_position(saved_seq_pos);
                    }
                }
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["<complex>"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Lastuse
            {
                let saved_pos = stream.position();
                if stream.expect_string("::lastuse").is_ok() {
                    return Ok(Op::Lastuse);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Discard
            {
                let saved_pos = stream.position();
                if stream.expect_string("::discard").is_ok() {
                    return Ok(Op::Discard);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Fill
            {
                let saved_pos = stream.position();
                if stream.expect_string("::fill").is_ok() {
                    return Ok(Op::Fill);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Use
            {
                let saved_pos = stream.position();
                if stream.expect_string("::use").is_ok() {
                    return Ok(Op::Use);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::lastuse", "::discard", "::fill", "::use"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaWsCtaGroup1KindI8CollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_complete()?;
            stream.expect_string(".ws")?;
            let ws = ();
            stream.expect_complete()?;
            stream.expect_string(".cta_group::1")?;
            let cta_group_1 = ();
            stream.expect_complete()?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let collector_usage = match CollectorUsage::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let zero_column_mask_desc = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05MmaWsCtaGroup1KindI8CollectorUsage {
                mma,
                ws,
                cta_group_1,
                kind_i8,
                collector_usage,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                enable_input_d,
                zero_column_mask_desc,
            })
        }
    }


    impl PtxParser for Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_complete()?;
            stream.expect_string(".ws")?;
            let ws = ();
            stream.expect_complete()?;
            stream.expect_string(".cta_group::1")?;
            let cta_group_1 = ();
            stream.expect_complete()?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let collector_usage = match CollectorUsage::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let zero_column_mask_desc = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 {
                mma,
                ws,
                cta_group_1,
                kind_i8,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                enable_input_d,
                zero_column_mask_desc,
            })
        }
    }


}

