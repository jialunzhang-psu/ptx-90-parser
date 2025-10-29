use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::setmaxnreg::*},
};

impl PtxParser for Action {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "inc" => Ok(Action::Inc),
            "dec" => Ok(Action::Dec),
            other => Err(unexpected_value(
                span,
                &[".inc", ".dec"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Setmaxnreg {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "setmaxnreg" {
            return Err(unexpected_value(span, &["setmaxnreg"], opcode));
        }

        let action = Action::parse(stream)?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;
        expect_directive_value(stream, "u32")?;

        let register_count = Immediate::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Setmaxnreg {
            action,
            register_count,
        })
    }
}
