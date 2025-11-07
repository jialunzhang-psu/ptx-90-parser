//! Original PTX specification:
//!
//! tcgen05.wait_operation.sync.aligned;
//! .wait_operation = { .wait::ld, .wait::st }

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_wait::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for WaitOperation {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try WaitLd
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wait::ld").is_ok() {
                    return Ok(WaitOperation::WaitLd);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try WaitSt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wait::st").is_ok() {
                    return Ok(WaitOperation::WaitSt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".wait::ld", ".wait::st"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05WaitOperationSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            let wait_operation = WaitOperation::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05WaitOperationSyncAligned {
                wait_operation,
                sync,
                aligned,
            })
        }
    }


}

