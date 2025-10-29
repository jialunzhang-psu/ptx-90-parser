use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::AddressOperand, instruction::clusterlaunchcontrol},
};

impl PtxParser for clusterlaunchcontrol::Space {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "shared" => {
                stream.expect_double_colon()?;
                let (identifier, identifier_span) = stream.expect_identifier()?;
                match identifier.as_str() {
                    "cta" => Ok(clusterlaunchcontrol::Space::SharedCta),
                    other => Err(unexpected_value(identifier_span, &["cta"], other)),
                }
            }
            other => Err(unexpected_value(
                span,
                &[".shared::cta"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for clusterlaunchcontrol::CompletionMechanism {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "mbarrier" => {
                stream.expect_double_colon()?;
                let (first, first_span) = stream.expect_identifier()?;
                if first != "complete_tx" {
                    return Err(unexpected_value(first_span, &["complete_tx"], first));
                }

                stream.expect_double_colon()?;
                let (second, second_span) = stream.expect_identifier()?;
                if second != "bytes" {
                    return Err(unexpected_value(second_span, &["bytes"], second));
                }

                Ok(clusterlaunchcontrol::CompletionMechanism::MbarrierCompleteTxBytes)
            }
            other => Err(unexpected_value(
                span,
                &[".mbarrier::complete_tx::bytes"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for clusterlaunchcontrol::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "b128")?;
        Ok(clusterlaunchcontrol::DataType::B128)
    }
}

impl PtxParser for clusterlaunchcontrol::Multicast {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "multicast" => {
                stream.expect_double_colon()?;
                let (scope, scope_span) = stream.expect_identifier()?;
                if scope != "cluster" {
                    return Err(unexpected_value(scope_span, &["cluster"], scope));
                }

                stream.expect_double_colon()?;
                let (target, target_span) = stream.expect_identifier()?;
                if target != "all" {
                    return Err(unexpected_value(target_span, &["all"], target));
                }

                Ok(clusterlaunchcontrol::Multicast::ClusterAll)
            }
            other => Err(unexpected_value(
                span,
                &[".multicast::cluster::all"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for clusterlaunchcontrol::TryCancel {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "clusterlaunchcontrol")?;
        expect_directive_value(stream, "try_cancel")?;
        expect_directive_value(stream, "async")?;

        let space = match peek_directive(stream)? {
            Some((directive, _)) if directive == "shared" => {
                Some(clusterlaunchcontrol::Space::parse(stream)?)
            }
            _ => None,
        };

        let completion_mechanism = clusterlaunchcontrol::CompletionMechanism::parse(stream)?;

        let multicast = match peek_directive(stream)? {
            Some((directive, _)) if directive == "multicast" => {
                Some(clusterlaunchcontrol::Multicast::parse(stream)?)
            }
            _ => None,
        };

        let data_type = clusterlaunchcontrol::DataType::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let mbarrier = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(clusterlaunchcontrol::TryCancel {
            space,
            completion_mechanism,
            multicast,
            data_type,
            address,
            mbarrier,
        })
    }
}
