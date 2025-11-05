//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind   [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind   [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! ------------------------------------------------------------------
//! // 2. Floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .kind = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .cta_group      = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! ------------------------------------------------------------------
//! // 3. Convolution MMA for floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind.collector_usage [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1,   .cta_group::2 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 4. Activation Stationary MMA for floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize  = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! .kind            = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 5. Integer type:
//! tcgen05.mma.cta_group.kind::i8  [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8  [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group = { .cta_group::1,   .cta_group::2  };
//! ------------------------------------------------------------------
//! // 6. Convolution MMA for integer type:
//! tcgen05.mma.cta_group.kind::i8.collector_usage          [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2  };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindF16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f16").is_ok() {
                    return Ok(Kind::KindF16);
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
            // Try KindF8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f8f6f4").is_ok() {
                    return Ok(Kind::KindF8f6f4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::f16", ".kind::tf32", ".kind::f8f6f4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let scale_input_d = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05MmaCtaGroupKind {
                mma,
                cta_group,
                kind,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
                scale_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKind1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let scale_input_d = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05MmaCtaGroupKind1 {
                mma,
                cta_group,
                kind,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
                scale_input_d,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for ScaleVectorsize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ScaleVec1x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::1X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec1x);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ScaleVec2x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::2X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec2x);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ScaleVec4x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::4X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec4x);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Block16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".block16").is_ok() {
                    return Ok(ScaleVectorsize::Block16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Block32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".block32").is_ok() {
                    return Ok(ScaleVectorsize::Block32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".scale_vec::1X", ".scale_vec::2X", ".scale_vec::4X", ".block16", ".block32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindMxf8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf8f6f4").is_ok() {
                    return Ok(Kind::KindMxf8f6f4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try KindMxf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4").is_ok() {
                    return Ok(Kind::KindMxf4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try KindMxf4nvf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4nvf4").is_ok() {
                    return Ok(Kind::KindMxf4nvf4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::mxf8f6f4", ".kind::mxf4", ".kind::mxf4nvf4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            let saved_pos = stream.position();
            let scale_vectorsize = match ScaleVectorsize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize {
                mma,
                cta_group,
                kind,
                block_scale,
                scale_vectorsize,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                scale_a_tmem,
                scale_b_tmem,
                enable_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            let saved_pos = stream.position();
            let scale_vectorsize = match ScaleVectorsize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 {
                mma,
                cta_group,
                kind,
                block_scale,
                scale_vectorsize,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                scale_a_tmem,
                scale_b_tmem,
                enable_input_d,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Fill
            {
                let saved_pos = stream.position();
                if stream.expect_string("::fill").is_ok() {
                    return Ok(Op::Fill);
                }
                stream.set_position(saved_pos);
            }
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
            let saved_pos = stream.position();
            // Try Lastuse
            {
                let saved_pos = stream.position();
                if stream.expect_string("::lastuse").is_ok() {
                    return Ok(Op::Lastuse);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Discard
            {
                let saved_pos = stream.position();
                if stream.expect_string("::discard*").is_ok() {
                    return Ok(Op::Discard);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::fill", "::use", "::lastuse", "::discard*"];
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

    impl PtxParser for Buffer {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try A
            {
                let saved_pos = stream.position();
                if stream.expect_string("::a").is_ok() {
                    return Ok(Buffer::A);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::a"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindF16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f16").is_ok() {
                    return Ok(Kind::KindF16);
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
            // Try KindF8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f8f6f4").is_ok() {
                    return Ok(Kind::KindF8f6f4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::f16", ".kind::tf32", ".kind::f8f6f4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindCollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let scale_input_d = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05MmaCtaGroupKindCollectorUsage {
                mma,
                cta_group,
                kind,
                collector_usage,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
                scale_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            let saved_pos = stream.position();
            let ashift = stream.expect_string(".ashift").is_ok();
            if !ashift {
                stream.set_position(saved_pos);
            }
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let scale_input_d = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05MmaCtaGroupKindAshiftCollectorUsage {
                mma,
                cta_group,
                kind,
                ashift,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
                scale_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            stream.expect_string(".ashift")?;
            let ashift = ();
            let saved_pos = stream.position();
            let collector_usage = match CollectorUsage::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let scale_input_d = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 {
                mma,
                cta_group,
                kind,
                ashift,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
                scale_input_d,
            })
        }
    }


}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for ScaleVectorsize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ScaleVec1x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::1X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec1x);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ScaleVec2x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::2X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec2x);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ScaleVec4x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::4X").is_ok() {
                    return Ok(ScaleVectorsize::ScaleVec4x);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Block16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".block16").is_ok() {
                    return Ok(ScaleVectorsize::Block16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Block32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".block32").is_ok() {
                    return Ok(ScaleVectorsize::Block32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".scale_vec::1X", ".scale_vec::2X", ".scale_vec::4X", ".block16", ".block32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Fill
            {
                let saved_pos = stream.position();
                if stream.expect_string("::fill").is_ok() {
                    return Ok(Op::Fill);
                }
                stream.set_position(saved_pos);
            }
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
            let saved_pos = stream.position();
            // Try Lastuse
            {
                let saved_pos = stream.position();
                if stream.expect_string("::lastuse").is_ok() {
                    return Ok(Op::Lastuse);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Discard
            {
                let saved_pos = stream.position();
                if stream.expect_string("::discard*").is_ok() {
                    return Ok(Op::Discard);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::fill", "::use", "::lastuse", "::discard*"];
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

    impl PtxParser for Buffer {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try A
            {
                let saved_pos = stream.position();
                if stream.expect_string("::a").is_ok() {
                    return Ok(Buffer::A);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::a"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindMxf8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf8f6f4").is_ok() {
                    return Ok(Kind::KindMxf8f6f4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try KindMxf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4").is_ok() {
                    return Ok(Kind::KindMxf4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try KindMxf4nvf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4nvf4").is_ok() {
                    return Ok(Kind::KindMxf4nvf4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::mxf8f6f4", ".kind::mxf4", ".kind::mxf4nvf4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            let saved_pos = stream.position();
            let scale_vectorsize = match ScaleVectorsize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
                mma,
                cta_group,
                kind,
                block_scale,
                scale_vectorsize,
                collector_usage,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                scale_a_tmem,
                scale_b_tmem,
                enable_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            let kind = Kind::parse(stream)?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            let saved_pos = stream.position();
            let scale_vectorsize = match ScaleVectorsize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
                mma,
                cta_group,
                kind,
                block_scale,
                scale_vectorsize,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                scale_a_tmem,
                scale_b_tmem,
                enable_input_d,
            })
        }
    }


}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindI8 {
                mma,
                cta_group,
                kind_i8,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindI81 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindI81 {
                mma,
                cta_group,
                kind_i8,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
            })
        }
    }


}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Buffer {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try A
            {
                let saved_pos = stream.position();
                if stream.expect_string("::a").is_ok() {
                    return Ok(Buffer::A);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::a"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Fill
            {
                let saved_pos = stream.position();
                if stream.expect_string("::fill").is_ok() {
                    return Ok(Op::Fill);
                }
                stream.set_position(saved_pos);
            }
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
            let saved_pos = stream.position();
            // Try Lastuse
            {
                let saved_pos = stream.position();
                if stream.expect_string("::lastuse").is_ok() {
                    return Ok(Op::Lastuse);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Discard
            {
                let saved_pos = stream.position();
                if stream.expect_string("::discard*").is_ok() {
                    return Ok(Op::Discard);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["::fill", "::use", "::lastuse", "::discard*"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8CollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindI8CollectorUsage {
                mma,
                cta_group,
                kind_i8,
                collector_usage,
                d_tmem,
                a_desc,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            stream.expect_string(".ashift")?;
            let ashift = ();
            let saved_pos = stream.position();
            let collector_usage = match CollectorUsage::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage {
                mma,
                cta_group,
                kind_i8,
                ashift,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
            })
        }
    }


    impl PtxParser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_string(".kind::i8")?;
            let kind_i8 = ();
            let saved_pos = stream.position();
            let ashift = stream.expect_string(".ashift").is_ok();
            if !ashift {
                stream.set_position(saved_pos);
            }
            let collector_usage = CollectorUsage::parse(stream)?;
            let d_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a_tmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let idesc = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let disable_output_lane = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let enable_input_d = Operand::parse(stream)?;
            Ok(Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 {
                mma,
                cta_group,
                kind_i8,
                ashift,
                collector_usage,
                d_tmem,
                a_tmem,
                b_desc,
                idesc,
                disable_output_lane,
                enable_input_d,
            })
        }
    }


}

