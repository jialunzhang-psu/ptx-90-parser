//! Original PTX specification:
//!
//! barrier.cluster.arrive{.sem}{.aligned};
//! barrier.cluster.wait{.acquire}{.aligned};
//! .sem = {.release, .relaxed};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::barrier_cluster::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Sem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Sem::Release);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Sem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".release", ".relaxed"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for BarrierClusterArriveSemAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            stream.expect_string(".cluster")?;
            let cluster = ();
            stream.expect_complete()?;
            stream.expect_string(".arrive")?;
            let arrive = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierClusterArriveSemAligned {
                cluster,
                arrive,
                sem,
                aligned,
            })
        }
    }

    impl PtxParser for BarrierClusterWaitAcquireAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            stream.expect_string(".cluster")?;
            let cluster = ();
            stream.expect_complete()?;
            stream.expect_string(".wait")?;
            let wait = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let acquire = stream.expect_string(".acquire").is_ok();
            if !acquire {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierClusterWaitAcquireAligned {
                cluster,
                wait,
                acquire,
                aligned,
            })
        }
    }
}
