//! Original PTX specification:
//!
//! setp.CmpOp{.ftz}.type         p{|q}, a, b;
//! setp.CmpOp.BoolOp{.ftz}.type  p{|q}, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .type   = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 };
//! --------------------------------------------------------------
//! setp.CmpOp{.ftz}.f16           p, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16    p, a, b, {!}c;
//! setp.CmpOp{.ftz}.f16x2         p|q, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16x2  p|q, a, b, {!}c;
//! setp.CmpOp.bf16                p, a, b;
//! setp.CmpOp.BoolOp.bf16         p, a, b, {!}c;
//! setp.CmpOp.bf16x2              p|q, a, b;
//! setp.CmpOp.BoolOp.bf16x2       p|q, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::setp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Boolop::And);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Boolop::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Boolop::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and", ".xor", ".or"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Cmpop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Equ
            {
                let saved_pos = stream.position();
                if stream.expect_string(".equ").is_ok() {
                    return Ok(Cmpop::Equ);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Neu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".neu").is_ok() {
                    return Ok(Cmpop::Neu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ltu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ltu").is_ok() {
                    return Ok(Cmpop::Ltu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Leu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".leu").is_ok() {
                    return Ok(Cmpop::Leu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gtu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gtu").is_ok() {
                    return Ok(Cmpop::Gtu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Geu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".geu").is_ok() {
                    return Ok(Cmpop::Geu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Num
            {
                let saved_pos = stream.position();
                if stream.expect_string(".num").is_ok() {
                    return Ok(Cmpop::Num);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Nan
            {
                let saved_pos = stream.position();
                if stream.expect_string(".nan").is_ok() {
                    return Ok(Cmpop::Nan);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Eq
            {
                let saved_pos = stream.position();
                if stream.expect_string(".eq").is_ok() {
                    return Ok(Cmpop::Eq);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ne
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ne").is_ok() {
                    return Ok(Cmpop::Ne);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Lt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".lt").is_ok() {
                    return Ok(Cmpop::Lt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Le
            {
                let saved_pos = stream.position();
                if stream.expect_string(".le").is_ok() {
                    return Ok(Cmpop::Le);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gt").is_ok() {
                    return Ok(Cmpop::Gt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ge
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ge").is_ok() {
                    return Ok(Cmpop::Ge);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Lo
            {
                let saved_pos = stream.position();
                if stream.expect_string(".lo").is_ok() {
                    return Ok(Cmpop::Lo);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ls
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ls").is_ok() {
                    return Ok(Cmpop::Ls);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Hi
            {
                let saved_pos = stream.position();
                if stream.expect_string(".hi").is_ok() {
                    return Ok(Cmpop::Hi);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Hs
            {
                let saved_pos = stream.position();
                if stream.expect_string(".hs").is_ok() {
                    return Ok(Cmpop::Hs);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".equ", ".neu", ".ltu", ".leu", ".gtu", ".geu", ".num", ".nan", ".eq", ".ne",
                ".lt", ".le", ".gt", ".ge", ".lo", ".ls", ".hi", ".hs",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Type::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Type::U16);
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
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Type::S16);
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f32",
                ".f64",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SetpCmpopFtzType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let q = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(GeneralOperand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopFtzType {
                cmpop,
                ftz,
                type_,
                p,
                q,
                a,
                b,
            })
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let q = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(GeneralOperand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBoolopFtzType {
                cmpop,
                boolop,
                ftz,
                type_,
                p,
                q,
                a,
                b,
                c_op,
                c,
            })
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::setp::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Boolop::And);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Boolop::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Boolop::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and", ".xor", ".or"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Cmpop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Equ
            {
                let saved_pos = stream.position();
                if stream.expect_string(".equ").is_ok() {
                    return Ok(Cmpop::Equ);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Neu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".neu").is_ok() {
                    return Ok(Cmpop::Neu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ltu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ltu").is_ok() {
                    return Ok(Cmpop::Ltu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Leu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".leu").is_ok() {
                    return Ok(Cmpop::Leu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gtu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gtu").is_ok() {
                    return Ok(Cmpop::Gtu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Geu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".geu").is_ok() {
                    return Ok(Cmpop::Geu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Num
            {
                let saved_pos = stream.position();
                if stream.expect_string(".num").is_ok() {
                    return Ok(Cmpop::Num);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Nan
            {
                let saved_pos = stream.position();
                if stream.expect_string(".nan").is_ok() {
                    return Ok(Cmpop::Nan);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Eq
            {
                let saved_pos = stream.position();
                if stream.expect_string(".eq").is_ok() {
                    return Ok(Cmpop::Eq);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ne
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ne").is_ok() {
                    return Ok(Cmpop::Ne);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Lt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".lt").is_ok() {
                    return Ok(Cmpop::Lt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Le
            {
                let saved_pos = stream.position();
                if stream.expect_string(".le").is_ok() {
                    return Ok(Cmpop::Le);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gt").is_ok() {
                    return Ok(Cmpop::Gt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ge
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ge").is_ok() {
                    return Ok(Cmpop::Ge);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".equ", ".neu", ".ltu", ".leu", ".gtu", ".geu", ".num", ".nan", ".eq", ".ne",
                ".lt", ".le", ".gt", ".ge",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SetpCmpopFtzF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopFtzF16 {
                cmpop,
                ftz,
                f16,
                p,
                a,
                b,
            })
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBoolopFtzF16 {
                cmpop,
                boolop,
                ftz,
                f16,
                p,
                a,
                b,
                c_op,
                c,
            })
        }
    }

    impl PtxParser for SetpCmpopFtzF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let q = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopFtzF16x2 {
                cmpop,
                ftz,
                f16x2,
                p,
                q,
                a,
                b,
            })
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let q = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBoolopFtzF16x2 {
                cmpop,
                boolop,
                ftz,
                f16x2,
                p,
                q,
                a,
                b,
                c_op,
                c,
            })
        }
    }

    impl PtxParser for SetpCmpopBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBf16 {
                cmpop,
                bf16,
                p,
                a,
                b,
            })
        }
    }

    impl PtxParser for SetpCmpopBoolopBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBoolopBf16 {
                cmpop,
                boolop,
                bf16,
                p,
                a,
                b,
                c_op,
                c,
            })
        }
    }

    impl PtxParser for SetpCmpopBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let q = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBf16x2 {
                cmpop,
                bf16x2,
                p,
                q,
                a,
                b,
            })
        }
    }

    impl PtxParser for SetpCmpopBoolopBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setp")?;
            let cmpop = Cmpop::parse(stream)?;
            stream.expect_complete()?;
            let boolop = Boolop::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let q = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetpCmpopBoolopBf16x2 {
                cmpop,
                boolop,
                bf16x2,
                p,
                q,
                a,
                b,
                c_op,
                c,
            })
        }
    }
}
