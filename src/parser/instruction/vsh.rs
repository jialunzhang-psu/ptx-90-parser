use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::RegisterOperand, instruction::vsh::*},
};

impl PtxParser for Opcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "vshl" => Ok(Opcode::Vshl),
            "vshr" => Ok(Opcode::Vshr),
            other => Err(unexpected_value(span, &["vshl", "vshr"], other)),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_modifier()?;
        match directive.as_str() {
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

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_modifier()?;
        match directive.as_str() {
            "clamp" => Ok(Mode::Clamp),
            "wrap" => Ok(Mode::Wrap),
            other => Err(unexpected_value(
                span,
                &[".clamp", ".wrap"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Selection {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_modifier()?;
        match directive.as_str() {
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

impl PtxParser for SecondaryOp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_modifier()?;
        match directive.as_str() {
            "add" => Ok(SecondaryOp::Add),
            "min" => Ok(SecondaryOp::Min),
            "max" => Ok(SecondaryOp::Max),
            other => Err(unexpected_value(
                span,
                &[".add", ".min", ".max"],
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

impl PtxParser for MergeDestination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;

        if !is_selection_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["destination selector"],
                format!("{token:?}"),
            ));
        }

        let selection = Selection::parse(stream)?;

        Ok(MergeDestination {
            register,
            selection,
        })
    }
}

impl PtxParser for Scalar {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let opcode = Opcode::parse(stream)?;
        let dtype = DataType::parse(stream)?;
        let atype = DataType::parse(stream)?;
        expect_directive_value(stream, "u32")?;

        let saturate = consume_directive_if(stream, "sat");
        let mode = Mode::parse(stream)?;

        if is_secondary_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["omit .op2 for scalar form"],
                format!("{token:?}"),
            ));
        }

        let destination = RegisterOperand::parse(stream)?;
        if is_selection_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["destination without selector"],
                format!("{token:?}"),
            ));
        }

        stream.expect(&PtxToken::Comma)?;
        let a = Source::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Source::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Scalar {
            opcode,
            dtype,
            atype,
            saturate,
            mode,
            destination,
            a,
            b,
        })
    }
}

impl PtxParser for ScalarWithSecondary {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let opcode = Opcode::parse(stream)?;
        let dtype = DataType::parse(stream)?;
        let atype = DataType::parse(stream)?;
        expect_directive_value(stream, "u32")?;

        let saturate = consume_directive_if(stream, "sat");
        let mode = Mode::parse(stream)?;
        let secondary = SecondaryOp::parse(stream)?;

        let destination = RegisterOperand::parse(stream)?;
        if is_selection_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["destination without selector"],
                format!("{token:?}"),
            ));
        }

        stream.expect(&PtxToken::Comma)?;
        let a = Source::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Source::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(ScalarWithSecondary {
            opcode,
            dtype,
            atype,
            saturate,
            mode,
            secondary,
            destination,
            a,
            b,
            c,
        })
    }
}

impl PtxParser for DataMerge {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let opcode = Opcode::parse(stream)?;
        let dtype = DataType::parse(stream)?;
        let atype = DataType::parse(stream)?;
        expect_directive_value(stream, "u32")?;

        let saturate = consume_directive_if(stream, "sat");
        let mode = Mode::parse(stream)?;

        if is_secondary_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["omit .op2 for data merge form"],
                format!("{token:?}"),
            ));
        }

        let destination = MergeDestination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = Source::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Source::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(DataMerge {
            opcode,
            dtype,
            atype,
            saturate,
            mode,
            destination,
            a,
            b,
            c,
        })
    }
}

impl PtxParser for Vsh {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let pos = stream.position();
        if let Ok(scalar) = Scalar::parse(stream) {
            return Ok(Vsh::Scalar(scalar));
        }

        stream.set_position(pos);
        if let Ok(scalar_secondary) = ScalarWithSecondary::parse(stream) {
            return Ok(Vsh::ScalarWithSecondary(scalar_secondary));
        }

        stream.set_position(pos);
        let merge = DataMerge::parse(stream)?;
        Ok(Vsh::DataMerge(merge))
    }
}

fn is_selection_directive(stream: &PtxTokenStream) -> bool {
    stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name)
            if matches!(name.as_str(), "b0" | "b1" | "b2" | "b3" | "h0" | "h1")
        )
    })
}

fn is_secondary_directive(stream: &PtxTokenStream) -> bool {
    stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if matches!(name.as_str(), "add" | "min" | "max")
        )
    })
}
