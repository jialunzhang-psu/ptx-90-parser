//! Original PTX specification:
//!
//! setmaxnreg.action.sync.aligned.u32 imm-reg-count;
//! .action = { .inc, .dec };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::setmaxnreg::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Action {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Inc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".inc").is_ok() {
                    return Ok(Action::Inc);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Dec
            {
                let saved_pos = stream.position();
                if stream.expect_string(".dec").is_ok() {
                    return Ok(Action::Dec);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".inc", ".dec"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SetmaxnregActionSyncAlignedU32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("setmaxnreg")?;
            let action = Action::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let imm_reg_count = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SetmaxnregActionSyncAlignedU32 {
                action,
                sync,
                aligned,
                u32,
                imm_reg_count,
            })
        }
    }
}
