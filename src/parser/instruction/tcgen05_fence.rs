//! Original PTX specification:
//!
//! tcgen05.fence::before_thread_sync ;
//! tcgen05.fence::after_thread_sync  ;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_fence::section_0::*;

    impl PtxParser for Tcgen05FenceBeforeThreadSync {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".fence::before_thread_sync")?;
            let fence_before_thread_sync = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05FenceBeforeThreadSync {
                fence_before_thread_sync,
            })
        }
    }


    impl PtxParser for Tcgen05FenceAfterThreadSync {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".fence::after_thread_sync")?;
            let fence_after_thread_sync = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05FenceAfterThreadSync {
                fence_after_thread_sync,
            })
        }
    }


}

