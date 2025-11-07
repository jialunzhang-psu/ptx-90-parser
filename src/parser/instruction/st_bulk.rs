//! Original PTX specification:
//!
//! st.bulk{.weak}{.shared::cta}  [a], size, initval; // initval must be zero

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::st_bulk::section_0::*;

    impl PtxParser for StBulkWeakSharedCta {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("st")?;
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let weak = stream.expect_string(".weak").is_ok();
            if !weak {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let shared_cta = stream.expect_string(".shared::cta").is_ok();
            if !shared_cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let size = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let initval = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(StBulkWeakSharedCta {
                bulk,
                weak,
                shared_cta,
                a,
                size,
                initval,
            })
        }
    }


}

