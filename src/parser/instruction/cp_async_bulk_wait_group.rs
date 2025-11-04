//! Original PTX specification:
//!
//! cp.async.bulk.wait_group{.read} N;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_wait_group::section_0::*;

    impl PtxParser for CpAsyncBulkWaitGroupRead {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_string(".wait_group")?;
            let wait_group = ();
            let saved_pos = stream.position();
            let read = stream.expect_string(".read").is_ok();
            if !read {
                stream.set_position(saved_pos);
            }
            let n = Operand::parse(stream)?;
            Ok(CpAsyncBulkWaitGroupRead {
                async_,
                bulk,
                wait_group,
                read,
                n,
            })
        }
    }


}

