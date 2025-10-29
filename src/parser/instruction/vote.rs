use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::vote::*},
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "all" => Ok(Mode::All),
            "any" => Ok(Mode::Any),
            "uni" => Ok(Mode::Uni),
            other => Err(unexpected_value(
                span,
                &[".all", ".any", ".uni"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for PredicateOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let negated = stream
            .consume_if(|token| matches!(token, PtxToken::Exclaim))
            .is_some();
        let register = PredicateRegister::parse(stream)?;

        Ok(Self { register, negated })
    }
}

impl PtxParser for Vote {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "vote" {
            return Err(unexpected_value(span, &["vote"], opcode));
        }

        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "ballot" => {
                expect_directive_value(stream, "b32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let source = PredicateOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Vote::Ballot(Ballot {
                    destination,
                    source,
                }))
            }
            "all" | "any" | "uni" => {
                let mode = match modifier.as_str() {
                    "all" => Mode::All,
                    "any" => Mode::Any,
                    "uni" => Mode::Uni,
                    _ => unreachable!(),
                };

                expect_directive_value(stream, "pred")?;
                let destination = PredicateRegister::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let source = PredicateOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Vote::Predicate(Predicate {
                    mode,
                    destination,
                    source,
                }))
            }
            other => Err(unexpected_value(
                span,
                &[".all", ".any", ".uni", ".ballot"],
                format!(".{other}"),
            )),
        }
    }
}
