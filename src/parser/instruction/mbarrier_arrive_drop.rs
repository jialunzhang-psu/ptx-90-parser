//! Original PTX specification:
//!
//! mbarrier.arrive_drop{.sem}{.scope}{.state}.b64 state,           [addr]{, count};
//! mbarrier.arrive_drop{.sem}{.scope}{.shared::cluster}.b64           _,   [addr] {,count};
//! mbarrier.arrive_drop.expect_tx{.state}{.sem}{.scope}.b64 state, [addr], tx_count;
//! mbarrier.arrive_drop.expect_tx{.shared::cluster}{.sem}{.scope}.b64   _, [addr], tx_count;
//! mbarrier.arrive_drop.noComplete{.release}{.cta}{.state}.b64 state,  [addr], count;
//! .sem   = { .release, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_arrive_drop::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".release", ".relaxed"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for State {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(State::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(State::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta", ".shared"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MbarrierArriveDropSemScopeStateB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".arrive_drop")?;
            let arrive_drop = ();
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
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let state = match State::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let state2 = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let count = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MbarrierArriveDropSemScopeStateB64 {
                arrive_drop,
                sem,
                scope,
                state,
                b64,
                state2,
                addr,
                count,
            })
        }
    }


    impl PtxParser for MbarrierArriveDropSemScopeSharedClusterB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".arrive_drop")?;
            let arrive_drop = ();
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
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let shared_cluster = stream.expect_string(".shared::cluster").is_ok();
            if !shared_cluster {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let operand = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let count = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MbarrierArriveDropSemScopeSharedClusterB64 {
                arrive_drop,
                sem,
                scope,
                shared_cluster,
                b64,
                operand,
                addr,
                count,
            })
        }
    }


    impl PtxParser for MbarrierArriveDropExpectTxStateSemScopeB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".arrive_drop")?;
            let arrive_drop = ();
            stream.expect_complete()?;
            stream.expect_string(".expect_tx")?;
            let expect_tx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let state = match State::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let state2 = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tx_count = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MbarrierArriveDropExpectTxStateSemScopeB64 {
                arrive_drop,
                expect_tx,
                state,
                sem,
                scope,
                b64,
                state2,
                addr,
                tx_count,
            })
        }
    }


    impl PtxParser for MbarrierArriveDropExpectTxSharedClusterSemScopeB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".arrive_drop")?;
            let arrive_drop = ();
            stream.expect_complete()?;
            stream.expect_string(".expect_tx")?;
            let expect_tx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let shared_cluster = stream.expect_string(".shared::cluster").is_ok();
            if !shared_cluster {
                stream.set_position(saved_pos);
            }
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
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let operand = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tx_count = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MbarrierArriveDropExpectTxSharedClusterSemScopeB64 {
                arrive_drop,
                expect_tx,
                shared_cluster,
                sem,
                scope,
                b64,
                operand,
                addr,
                tx_count,
            })
        }
    }


    impl PtxParser for MbarrierArriveDropNocompleteReleaseCtaStateB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".arrive_drop")?;
            let arrive_drop = ();
            stream.expect_complete()?;
            stream.expect_string(".noComplete")?;
            let nocomplete = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let release = stream.expect_string(".release").is_ok();
            if !release {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let state = match State::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let state2 = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let count = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MbarrierArriveDropNocompleteReleaseCtaStateB64 {
                arrive_drop,
                nocomplete,
                release,
                cta,
                state,
                b64,
                state2,
                addr,
                count,
            })
        }
    }


}

