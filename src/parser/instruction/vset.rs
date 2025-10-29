use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::RegisterOperand, instruction::vset::*},
};

impl PtxParser for Vset {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "vset")?;

        let atype = DataType::parse(stream)?;
        let btype = DataType::parse(stream)?;
        let cmp = Compare::parse(stream)?;

        if is_secondary_directive(stream) {
            let op2 = SecondaryOperation::parse(stream)?;
            let d = RegisterOperand::parse(stream)?;
            ensure_no_selection_after_destination(stream)?;

            stream.expect(&PtxToken::Comma)?;
            let a = Source::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Source::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Vset::ScalarWithSecondary(ScalarWithSecondary {
                atype,
                btype,
                cmp,
                op2,
                d,
                a,
                b,
                c,
            }))
        } else {
            let d = RegisterOperand::parse(stream)?;

            if is_selection_directive(stream) {
                let dsel = Selection::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = Source::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = Source::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let c = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Vset::DataMerge(DataMerge {
                    atype,
                    btype,
                    cmp,
                    d,
                    dsel,
                    a,
                    b,
                    c,
                }))
            } else {
                stream.expect(&PtxToken::Comma)?;
                let a = Source::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = Source::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Vset::Scalar(Scalar {
                    atype,
                    btype,
                    cmp,
                    d,
                    a,
                    b,
                }))
            }
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "u32" => Ok(DataType::U32),
            "s32" => Ok(DataType::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Compare {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "eq" => Ok(Compare::Eq),
            "ne" => Ok(Compare::Ne),
            "lt" => Ok(Compare::Lt),
            "le" => Ok(Compare::Le),
            "gt" => Ok(Compare::Gt),
            "ge" => Ok(Compare::Ge),
            other => Err(unexpected_value(
                span,
                &[".eq", ".ne", ".lt", ".le", ".gt", ".ge"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for SecondaryOperation {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "add" => Ok(SecondaryOperation::Add),
            "min" => Ok(SecondaryOperation::Min),
            "max" => Ok(SecondaryOperation::Max),
            other => Err(unexpected_value(
                span,
                &[".add", ".min", ".max"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Selection {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "b0" => Ok(Selection::B0),
            "b1" => Ok(Selection::B1),
            "b2" => Ok(Selection::B2),
            "b3" => Ok(Selection::B3),
            "h0" => Ok(Selection::H0),
            "h1" => Ok(Selection::H1),
            other => Err(unexpected_value(
                span,
                &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Source {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selection = if is_selection_directive(stream) {
            Some(Selection::parse(stream)?)
        } else {
            None
        };

        Ok(Source {
            register,
            selection,
        })
    }
}

fn is_selection_directive(stream: &mut PtxTokenStream) -> bool {
    stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name)
                if matches!(
                    name.as_str(),
                    "b0" | "b1" | "b2" | "b3" | "h0" | "h1"
                )
        )
    })
}

fn is_secondary_directive(stream: &mut PtxTokenStream) -> bool {
    stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if matches!(name.as_str(), "add" | "min" | "max")
        )
    })
}

fn ensure_no_selection_after_destination(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    if is_selection_directive(stream) {
        let (token, span) = stream.peek()?;
        return Err(unexpected_value(
            span.clone(),
            &["destination without selector"],
            format!("{token:?}"),
        ));
    }
    Ok(())
}
