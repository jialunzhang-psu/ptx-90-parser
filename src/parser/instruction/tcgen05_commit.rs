//! Original PTX specification:
//!
//! tcgen05.commit.cta_group.completion_mechanism{.shared::cluster}{.multicast}.b64
//! [mbar] {, ctaMask};
//! .completion_mechanism = { .mbarrier::arrive::one };
//! .cta_group            = { .cta_group::1, .cta_group::2 };
//! .multicast            = { .multicast::cluster };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_commit::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Multicast {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MulticastCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".multicast::cluster").is_ok() {
                    return Ok(Multicast::MulticastCluster);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".multicast::cluster"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierArriveOne
            {
                let saved_pos = stream.position();
                if stream.expect_string(".mbarrier::arrive::one").is_ok() {
                    return Ok(CompletionMechanism::MbarrierArriveOne);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::arrive::one"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".commit")?;
            let commit = ();
            let cta_group = CtaGroup::parse(stream)?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            let saved_pos = stream.position();
            let shared_cluster = stream.expect_string(".shared::cluster").is_ok();
            if !shared_cluster {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let multicast = match Multicast::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".b64")?;
            let b64 = ();
            let mbar = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let ctamask = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
                commit,
                cta_group,
                completion_mechanism,
                shared_cluster,
                multicast,
                b64,
                mbar,
                ctamask,
            })
        }
    }


}

