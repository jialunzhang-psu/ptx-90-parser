//! Original PTX specification:
//!
//! griddepcontrol.action;
//! .action   = { .launch_dependents, .wait };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::griddepcontrol::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Action {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try LaunchDependents
            {
                let saved_pos = stream.position();
                if stream.expect_string(".launch_dependents").is_ok() {
                    return Ok(Action::LaunchDependents);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Wait
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wait").is_ok() {
                    return Ok(Action::Wait);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".launch_dependents", ".wait"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for GriddepcontrolAction {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("griddepcontrol")?;
            let action = Action::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(GriddepcontrolAction { action })
        }
    }
}
