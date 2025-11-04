//! Original PTX specification:
//!
//! cp.async.wait_group N;
//! cp.async.wait_all ;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_wait_group::section_0::*;

    impl PtxParser for CpAsyncWaitGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".wait_group")?;
            let wait_group = ();
            let n = Operand::parse(stream)?;
            Ok(CpAsyncWaitGroup {
                async_,
                wait_group,
                n,
            })
        }
    }


    impl PtxParser for CpAsyncWaitAll {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".wait_all")?;
            let wait_all = ();
            Ok(CpAsyncWaitAll {
                async_,
                wait_all,
            })
        }
    }


}

