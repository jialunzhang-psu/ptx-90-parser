//! Original PTX specification:
//!
//! clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .space = { .shared::cta };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::clusterlaunchcontrol_try_cancel::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierCompleteTxBytes
            {
                let saved_pos = stream.position();
                if stream
                    .expect_string(".mbarrier::complete_tx::bytes")
                    .is_ok()
                {
                    return Ok(CompletionMechanism::MbarrierCompleteTxBytes);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::complete_tx::bytes"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Space {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Space::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser
        for ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128
    {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("clusterlaunchcontrol")?;
            stream.expect_string(".try_cancel")?;
            let try_cancel = ();
            stream.expect_complete()?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let multicast_cluster_all = stream.expect_string(".multicast::cluster::all").is_ok();
            if !multicast_cluster_all {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let mbar = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 {
                    try_cancel,
                    async_,
                    space,
                    completion_mechanism,
                    multicast_cluster_all,
                    b128,
                    addr,
                    mbar,
                },
            )
        }
    }
}
