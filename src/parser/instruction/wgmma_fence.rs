//! Original PTX specification:
//!
//! wgmma.fence.sync.aligned;

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_fence::section_0::*;

    impl PtxParser for WgmmaFenceSyncAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".fence"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    semicolon_p()
                ),
                |(_, fence, sync, aligned, _), span| {
                    ok!(WgmmaFenceSyncAligned {
                        fence = fence,
                        sync = sync,
                        aligned = aligned,

                    })
                },
            )
        }
    }
}
