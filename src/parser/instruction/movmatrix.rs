use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::movmatrix::{DataType, Movmatrix, Shape},
    },
};

impl PtxParser for Shape {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "m8n8" => Ok(Shape::M8N8),
            other => Err(unexpected_value(span, &[".m8n8"], format!(".{other}"))),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b16" => Ok(DataType::B16),
            other => Err(unexpected_value(span, &[".b16"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Movmatrix {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "movmatrix" {
            return Err(unexpected_value(span, &["movmatrix"], opcode));
        }

        let (sync, sync_span) = stream.expect_directive()?;
        if sync != "sync" {
            return Err(unexpected_value(sync_span, &[".sync"], format!(".{sync}")));
        }

        let (aligned, aligned_span) = stream.expect_directive()?;
        if aligned != "aligned" {
            return Err(unexpected_value(
                aligned_span,
                &[".aligned"],
                format!(".{aligned}"),
            ));
        }

        let shape = Shape::parse(stream)?;

        let (trans, trans_span) = stream.expect_directive()?;
        if trans != "trans" {
            return Err(unexpected_value(
                trans_span,
                &[".trans"],
                format!(".{trans}"),
            ));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Movmatrix {
            shape,
            data_type,
            destination,
            source,
        })
    }
}
