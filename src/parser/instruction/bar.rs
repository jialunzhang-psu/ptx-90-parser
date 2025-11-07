//! Original PTX specification:
//!
//! barrier{.cta}.sync{.aligned}      a{, b};
//! barrier{.cta}.arrive{.aligned}    a, b;
//! barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
//! barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
//! bar{.cta}.sync      a{, b};
//! bar{.cta}.arrive    a, b;
//! bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
//! bar{.cta}.red.op.pred   p, a{, b}, {!}c;
//! .op = { .and, .or };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bar::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Op::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and", ".or"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for BarrierCtaSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierCtaSyncAligned {
                cta,
                sync,
                aligned,
                a,
                b,
            })
        }
    }


    impl PtxParser for BarrierCtaArriveAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".arrive")?;
            let arrive = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierCtaArriveAligned {
                cta,
                arrive,
                aligned,
                a,
                b,
            })
        }
    }


    impl PtxParser for BarrierCtaRedPopcAlignedU32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            stream.expect_string(".popc")?;
            let popc = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierCtaRedPopcAlignedU32 {
                cta,
                red,
                popc,
                aligned,
                u32,
                d,
                a,
                b,
                c_op,
                c,
            })
        }
    }


    impl PtxParser for BarrierCtaRedOpAlignedPred {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("barrier")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let aligned = stream.expect_string(".aligned").is_ok();
            if !aligned {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".pred")?;
            let pred = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarrierCtaRedOpAlignedPred {
                cta,
                red,
                op,
                aligned,
                pred,
                p,
                a,
                b,
                c_op,
                c,
            })
        }
    }


    impl PtxParser for BarCtaSync {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bar")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarCtaSync {
                cta,
                sync,
                a,
                b,
            })
        }
    }


    impl PtxParser for BarCtaArrive {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bar")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".arrive")?;
            let arrive = ();
            stream.expect_complete()?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarCtaArrive {
                cta,
                arrive,
                a,
                b,
            })
        }
    }


    impl PtxParser for BarCtaRedPopcU32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bar")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            stream.expect_string(".popc")?;
            let popc = ();
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarCtaRedPopcU32 {
                cta,
                red,
                popc,
                u32,
                d,
                a,
                b,
                c_op,
                c,
            })
        }
    }


    impl PtxParser for BarCtaRedOpPred {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bar")?;
            let saved_pos = stream.position();
            let cta = stream.expect_string(".cta").is_ok();
            if !cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".pred")?;
            let pred = ();
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let b = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c_op = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BarCtaRedOpPred {
                cta,
                red,
                op,
                pred,
                p,
                a,
                b,
                c_op,
                c,
            })
        }
    }


}

