use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::vop::{
            DataMerge, DataType, MergeDestination, Opcode, Operand, Scalar, ScalarWithSecondary,
            SecondaryOpcode, Selection, Vop,
        },
    },
};

impl PtxParser for Opcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "vadd" => Ok(Opcode::Vadd),
            "vsub" => Ok(Opcode::Vsub),
            "vabsdiff" => Ok(Opcode::Vabsdiff),
            "vmin" => Ok(Opcode::Vmin),
            "vmax" => Ok(Opcode::Vmax),
            other => Err(unexpected_value(
                span,
                &["vadd", "vsub", "vabsdiff", "vmin", "vmax"],
                other,
            )),
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

impl PtxParser for SecondaryOpcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "add" => Ok(SecondaryOpcode::Add),
            "min" => Ok(SecondaryOpcode::Min),
            "max" => Ok(SecondaryOpcode::Max),
            other => Err(unexpected_value(
                span,
                &[".add", ".min", ".max"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Operand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selection = if is_selection_directive(stream) {
            Some(Selection::parse(stream)?)
        } else {
            None
        };

        Ok(Operand {
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
                &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"],
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
        let btype = DataType::parse(stream)?;

        let mut saturate = false;
        if stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
            .is_some()
        {
            saturate = true;
        }

        if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if is_secondary_name(name)))
        {
            let (_, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["omit .op2 for scalar form"],
                format!("{:?}", stream.peek()?.0),
            ));
        }

        let d = RegisterOperand::parse(stream)?;
        if is_selection_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["destination without selector"],
                format!("{token:?}"),
            ));
        }

        stream.expect(&PtxToken::Comma)?;
        let a = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Scalar {
            opcode,
            dtype,
            atype,
            btype,
            saturate,
            d,
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
        let btype = DataType::parse(stream)?;

        let mut saturate = false;
        if stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
            .is_some()
        {
            saturate = true;
        }

        let op2 = SecondaryOpcode::parse(stream)?;
        let d = RegisterOperand::parse(stream)?;

        if is_selection_directive(stream) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["destination without selector"],
                format!("{token:?}"),
            ));
        }

        stream.expect(&PtxToken::Comma)?;
        let a = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(ScalarWithSecondary {
            opcode,
            dtype,
            atype,
            btype,
            saturate,
            op2,
            d,
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
        let btype = DataType::parse(stream)?;

        let mut saturate = false;
        if stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
            .is_some()
        {
            saturate = true;
        }

        if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if is_secondary_name(name)))
        {
            let (_, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["omit .op2 for data merge form"],
                format!("{:?}", stream.peek()?.0),
            ));
        }

        let d = MergeDestination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(DataMerge {
            opcode,
            dtype,
            atype,
            btype,
            saturate,
            d,
            a,
            b,
            c,
        })
    }
}

impl PtxParser for Vop {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let pos = stream.position();
        if let Ok(scalar) = Scalar::parse(stream) {
            return Ok(Vop::Scalar(scalar));
        }

        stream.set_position(pos);
        if let Ok(with_secondary) = ScalarWithSecondary::parse(stream) {
            return Ok(Vop::ScalarWithSecondary(with_secondary));
        }

        stream.set_position(pos);
        let merge = DataMerge::parse(stream)?;
        Ok(Vop::DataMerge(merge))
    }
}

fn is_selection_directive(stream: &PtxTokenStream) -> bool {
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

fn is_secondary_name(name: &str) -> bool {
    matches!(name, "add" | "min" | "max")
}
