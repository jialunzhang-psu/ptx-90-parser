//! Original PTX specification:
//!
//! // Integer type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op.type      d, [a];
//! multimem.ld_reduce.weak{.ss}.op.type                 d, [a];
//! multimem.st{.stsem}{.scope}{.ss}.type                [a], b;
//! multimem.st.weak{.ss}.type                           [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.op.type           [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add, .and, .or, .xor };
//! .type =     { .b32, .b64,  .u32, .u64, .s32, .s64 };
//! ------------------------------------------------------------------
//! // Floating point type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op{.acc_prec}{.vec}.type    d, [a];
//! multimem.ld_reduce.weak{.ss}.op{.acc_prec}{.vec}.type               d, [a];
//! multimem.st{.stsem}{.scope}{.ss}{.vec}.type                         [a], b;
//! multimem.st.weak{.ss}{.vec}.type                                    [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.redop{.vec}.redtype              [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add };
//! .redop  =   { .add };
//! .acc_prec = { .acc::f32, .acc::f16 };
//! .vec =      { .v2, .v4, .v8 };
//! .type=      { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64, .e5m2, .e5m2x2, .e5m2x4, .e4m3, .e4m3x2, .e4m3x4 };
//! .redtype =  { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::multimem_ld_reduce::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ldsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Ldsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Acquire
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acquire").is_ok() {
                    return Ok(Ldsem::Acquire);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".acquire"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Op::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Op::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".min", ".max", ".add", ".and", ".xor", ".or"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Redsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Redsem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".release"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Stsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Stsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Stsem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".release"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Type::B32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Type::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Type::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Type::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".b64", ".u32", ".u64", ".s32", ".s64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MultimemLdReduceLdsemScopeSsOpType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".ld_reduce")?;
            let ld_reduce = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ldsem = match Ldsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemLdReduceLdsemScopeSsOpType {
                ld_reduce,
                ldsem,
                scope,
                ss,
                op,
                type_,
                d,
                a,
            })
        }
    }


    impl PtxParser for MultimemLdReduceWeakSsOpType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".ld_reduce")?;
            let ld_reduce = ();
            stream.expect_complete()?;
            stream.expect_string(".weak")?;
            let weak = ();
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemLdReduceWeakSsOpType {
                ld_reduce,
                weak,
                ss,
                op,
                type_,
                d,
                a,
            })
        }
    }


    impl PtxParser for MultimemStStsemScopeSsType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let stsem = match Stsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemStStsemScopeSsType {
                st,
                stsem,
                scope,
                ss,
                type_,
                a,
                b,
            })
        }
    }


    impl PtxParser for MultimemStWeakSsType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_complete()?;
            stream.expect_string(".weak")?;
            let weak = ();
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
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemStWeakSsType {
                st,
                weak,
                ss,
                type_,
                a,
                b,
            })
        }
    }


    impl PtxParser for MultimemRedRedsemScopeSsOpType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let redsem = match Redsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemRedRedsemScopeSsOpType {
                red,
                redsem,
                scope,
                ss,
                op,
                type_,
                a,
                b,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::multimem_ld_reduce::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for AccPrec {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try AccF32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acc::f32").is_ok() {
                    return Ok(AccPrec::AccF32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try AccF16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acc::f16").is_ok() {
                    return Ok(AccPrec::AccF16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".acc::f32", ".acc::f16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ldsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Ldsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Acquire
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acquire").is_ok() {
                    return Ok(Ldsem::Acquire);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".acquire"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".min", ".max", ".add"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Redop::Add);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".add"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Redsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Redsem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".release"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16x2").is_ok() {
                    return Ok(Redtype::Bf16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16x2").is_ok() {
                    return Ok(Redtype::F16x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Redtype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Redtype::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Redtype::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Redtype::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16x2", ".f16x2", ".bf16", ".f16", ".f32", ".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Stsem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Stsem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Stsem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".release"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16x2").is_ok() {
                    return Ok(Type::Bf16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2x2").is_ok() {
                    return Ok(Type::E5m2x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E5m2x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2x4").is_ok() {
                    return Ok(Type::E5m2x4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E4m3x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3x2").is_ok() {
                    return Ok(Type::E4m3x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E4m3x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3x4").is_ok() {
                    return Ok(Type::E4m3x4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16x2").is_ok() {
                    return Ok(Type::F16x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Type::Bf16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(Type::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(Type::E4m3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Type::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Type::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".bf16x2", ".e5m2x2", ".e5m2x4", ".e4m3x2", ".e4m3x4", ".f16x2", ".bf16", ".e5m2", ".e4m3", ".f16", ".f32", ".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vec {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try V2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v2").is_ok() {
                    return Ok(Vec::V2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try V4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v4").is_ok() {
                    return Ok(Vec::V4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try V8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v8").is_ok() {
                    return Ok(Vec::V8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".v2", ".v4", ".v8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".ld_reduce")?;
            let ld_reduce = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ldsem = match Ldsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let acc_prec = match AccPrec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
                ld_reduce,
                ldsem,
                scope,
                ss,
                op,
                acc_prec,
                vec,
                type_,
                d,
                a,
            })
        }
    }


    impl PtxParser for MultimemLdReduceWeakSsOpAccPrecVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".ld_reduce")?;
            let ld_reduce = ();
            stream.expect_complete()?;
            stream.expect_string(".weak")?;
            let weak = ();
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let acc_prec = match AccPrec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemLdReduceWeakSsOpAccPrecVecType {
                ld_reduce,
                weak,
                ss,
                op,
                acc_prec,
                vec,
                type_,
                d,
                a,
            })
        }
    }


    impl PtxParser for MultimemStStsemScopeSsVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let stsem = match Stsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemStStsemScopeSsVecType {
                st,
                stsem,
                scope,
                ss,
                vec,
                type_,
                a,
                b,
            })
        }
    }


    impl PtxParser for MultimemStWeakSsVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_complete()?;
            stream.expect_string(".weak")?;
            let weak = ();
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
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemStWeakSsVecType {
                st,
                weak,
                ss,
                vec,
                type_,
                a,
                b,
            })
        }
    }


    impl PtxParser for MultimemRedRedsemScopeSsRedopVecRedtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("multimem")?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let redsem = match Redsem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let redop = Redop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let redtype = Redtype::parse(stream)?;
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MultimemRedRedsemScopeSsRedopVecRedtype {
                red,
                redsem,
                scope,
                ss,
                redop,
                vec,
                redtype,
                a,
                b,
            })
        }
    }


}

