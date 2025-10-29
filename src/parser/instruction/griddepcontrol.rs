#[allow(unused_imports)]
use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, expect_identifier_value, unexpected_value},
    r#type::{common::*, instruction::griddepcontrol::*},
};

impl PtxParser for Griddepcontrol {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "griddepcontrol")?;

        let (modifier, span) = stream.expect_directive()?;
        let instruction = match modifier.as_str() {
            "launch_dependents" => Griddepcontrol::LaunchDependents,
            "wait" => Griddepcontrol::Wait,
            other => {
                return Err(unexpected_value(
                    span,
                    &[".launch_dependents", ".wait"],
                    format!(".{other}"),
                ));
            }
        };

        stream.expect(&PtxToken::Semicolon)?;
        Ok(instruction)
    }
}
