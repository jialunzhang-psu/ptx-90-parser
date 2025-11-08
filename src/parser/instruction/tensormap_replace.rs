//! Original PTX specification:
//!
//! tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
//! tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
//! tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;
//! .mode    = { .tile };
//! .field1  = { .global_address, .rank };
//! .field2  = { .box_dim, .global_dim, .global_stride, .element_stride  };
//! .field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode };
//! .ss      = { .global, .shared::cta };
//! .type    = { .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tensormap_replace::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Field1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try GlobalAddress
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global_address").is_ok() {
                    return Ok(Field1::GlobalAddress);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rank
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rank").is_ok() {
                    return Ok(Field1::Rank);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global_address", ".rank"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Field2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ElementStride
            {
                let saved_pos = stream.position();
                if stream.expect_string(".element_stride").is_ok() {
                    return Ok(Field2::ElementStride);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try GlobalStride
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global_stride").is_ok() {
                    return Ok(Field2::GlobalStride);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try GlobalDim
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global_dim").is_ok() {
                    return Ok(Field2::GlobalDim);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try BoxDim
            {
                let saved_pos = stream.position();
                if stream.expect_string(".box_dim").is_ok() {
                    return Ok(Field2::BoxDim);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".element_stride",
                ".global_stride",
                ".global_dim",
                ".box_dim",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Field3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try InterleaveLayout
            {
                let saved_pos = stream.position();
                if stream.expect_string(".interleave_layout").is_ok() {
                    return Ok(Field3::InterleaveLayout);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try SwizzleAtomicity
            {
                let saved_pos = stream.position();
                if stream.expect_string(".swizzle_atomicity").is_ok() {
                    return Ok(Field3::SwizzleAtomicity);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try SwizzleMode
            {
                let saved_pos = stream.position();
                if stream.expect_string(".swizzle_mode").is_ok() {
                    return Ok(Field3::SwizzleMode);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try FillMode
            {
                let saved_pos = stream.position();
                if stream.expect_string(".fill_mode").is_ok() {
                    return Ok(Field3::FillMode);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Elemtype
            {
                let saved_pos = stream.position();
                if stream.expect_string(".elemtype").is_ok() {
                    return Ok(Field3::Elemtype);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".interleave_layout",
                ".swizzle_atomicity",
                ".swizzle_mode",
                ".fill_mode",
                ".elemtype",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Tile
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile").is_ok() {
                    return Ok(Mode::Tile);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tile"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta", ".global"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".b64"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TensormapReplaceModeField1SsB1024Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tensormap")?;
            stream.expect_string(".replace")?;
            let replace = ();
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let field1 = Field1::parse(stream)?;
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
            stream.expect_string(".b1024")?;
            let b1024 = ();
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let new_val = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(TensormapReplaceModeField1SsB1024Type {
                replace,
                mode,
                field1,
                ss,
                b1024,
                type_,
                addr,
                new_val,
            })
        }
    }

    impl PtxParser for TensormapReplaceModeField2SsB1024Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tensormap")?;
            stream.expect_string(".replace")?;
            let replace = ();
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let field2 = Field2::parse(stream)?;
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
            stream.expect_string(".b1024")?;
            let b1024 = ();
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let ord = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let new_val = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(TensormapReplaceModeField2SsB1024Type {
                replace,
                mode,
                field2,
                ss,
                b1024,
                type_,
                addr,
                ord,
                new_val,
            })
        }
    }

    impl PtxParser for TensormapReplaceModeField3SsB1024Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tensormap")?;
            stream.expect_string(".replace")?;
            let replace = ();
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let field3 = Field3::parse(stream)?;
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
            stream.expect_string(".b1024")?;
            let b1024 = ();
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let new_val = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(TensormapReplaceModeField3SsB1024Type {
                replace,
                mode,
                field3,
                ss,
                b1024,
                type_,
                addr,
                new_val,
            })
        }
    }
}
