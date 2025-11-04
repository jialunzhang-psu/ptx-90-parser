//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b00, .b01, .b02, .b03, .b04, .b05, .b06, .b07,
//!                   .b10, .b11, .b12, .b13, .b14, .b15, .b16, .b17,
//!                   .b20, .b21, .b22, .b23, .b24, .b25, .b26, .b27,
//!                   .b30, .b31, .b32, .b33, .b34, .b35, .b36, .b37,
//!                   .b40, .b41, .b42, .b43, .b44, .b45, .b46, .b47,
//!                   .b50, .b51, .b52, .b53, .b54, .b55, .b56, .b57,
//!                   .b60, .b61, .b62, .b63, .b64, .b65, .b66, .b67,
//!                   .b70, .b71, .b72, .b73, .b74, .b75, .b76, .b77
//!                   } // where x,y,z,w are from { 0, ..., 7 };
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Asel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B00
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b00").is_ok() {
                    return Ok(Asel::B00);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B01
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b01").is_ok() {
                    return Ok(Asel::B01);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B02
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b02").is_ok() {
                    return Ok(Asel::B02);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B03
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b03").is_ok() {
                    return Ok(Asel::B03);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B04
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b04").is_ok() {
                    return Ok(Asel::B04);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B05
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b05").is_ok() {
                    return Ok(Asel::B05);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B06
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b06").is_ok() {
                    return Ok(Asel::B06);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B07
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b07").is_ok() {
                    return Ok(Asel::B07);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b10").is_ok() {
                    return Ok(Asel::B10);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B11
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b11").is_ok() {
                    return Ok(Asel::B11);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B12
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b12").is_ok() {
                    return Ok(Asel::B12);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B13
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b13").is_ok() {
                    return Ok(Asel::B13);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B14
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b14").is_ok() {
                    return Ok(Asel::B14);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B15
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b15").is_ok() {
                    return Ok(Asel::B15);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Asel::B16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B17
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b17").is_ok() {
                    return Ok(Asel::B17);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b20").is_ok() {
                    return Ok(Asel::B20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b21").is_ok() {
                    return Ok(Asel::B21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B22
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b22").is_ok() {
                    return Ok(Asel::B22);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B23
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b23").is_ok() {
                    return Ok(Asel::B23);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B24
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b24").is_ok() {
                    return Ok(Asel::B24);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B25
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b25").is_ok() {
                    return Ok(Asel::B25);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B26
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b26").is_ok() {
                    return Ok(Asel::B26);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B27
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b27").is_ok() {
                    return Ok(Asel::B27);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b30").is_ok() {
                    return Ok(Asel::B30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b31").is_ok() {
                    return Ok(Asel::B31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Asel::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B33
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b33").is_ok() {
                    return Ok(Asel::B33);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B34
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b34").is_ok() {
                    return Ok(Asel::B34);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B35
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b35").is_ok() {
                    return Ok(Asel::B35);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B36
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b36").is_ok() {
                    return Ok(Asel::B36);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B37
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b37").is_ok() {
                    return Ok(Asel::B37);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B40
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b40").is_ok() {
                    return Ok(Asel::B40);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B41
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b41").is_ok() {
                    return Ok(Asel::B41);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B42
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b42").is_ok() {
                    return Ok(Asel::B42);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B43
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b43").is_ok() {
                    return Ok(Asel::B43);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B44
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b44").is_ok() {
                    return Ok(Asel::B44);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B45
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b45").is_ok() {
                    return Ok(Asel::B45);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B46
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b46").is_ok() {
                    return Ok(Asel::B46);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B47
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b47").is_ok() {
                    return Ok(Asel::B47);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B50
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b50").is_ok() {
                    return Ok(Asel::B50);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B51
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b51").is_ok() {
                    return Ok(Asel::B51);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B52
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b52").is_ok() {
                    return Ok(Asel::B52);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B53
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b53").is_ok() {
                    return Ok(Asel::B53);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B54
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b54").is_ok() {
                    return Ok(Asel::B54);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B55
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b55").is_ok() {
                    return Ok(Asel::B55);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B56
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b56").is_ok() {
                    return Ok(Asel::B56);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B57
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b57").is_ok() {
                    return Ok(Asel::B57);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B60
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b60").is_ok() {
                    return Ok(Asel::B60);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B61
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b61").is_ok() {
                    return Ok(Asel::B61);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B62
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b62").is_ok() {
                    return Ok(Asel::B62);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B63
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b63").is_ok() {
                    return Ok(Asel::B63);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Asel::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B65
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b65").is_ok() {
                    return Ok(Asel::B65);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B66
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b66").is_ok() {
                    return Ok(Asel::B66);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B67
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b67").is_ok() {
                    return Ok(Asel::B67);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B70
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b70").is_ok() {
                    return Ok(Asel::B70);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B71
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b71").is_ok() {
                    return Ok(Asel::B71);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B72
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b72").is_ok() {
                    return Ok(Asel::B72);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B73
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b73").is_ok() {
                    return Ok(Asel::B73);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B74
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b74").is_ok() {
                    return Ok(Asel::B74);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B75
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b75").is_ok() {
                    return Ok(Asel::B75);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B76
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b76").is_ok() {
                    return Ok(Asel::B76);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B77
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b77").is_ok() {
                    return Ok(Asel::B77);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b00", ".b01", ".b02", ".b03", ".b04", ".b05", ".b06", ".b07", ".b10", ".b11", ".b12", ".b13", ".b14", ".b15", ".b16", ".b17", ".b20", ".b21", ".b22", ".b23", ".b24", ".b25", ".b26", ".b27", ".b30", ".b31", ".b32", ".b33", ".b34", ".b35", ".b36", ".b37", ".b40", ".b41", ".b42", ".b43", ".b44", ".b45", ".b46", ".b47", ".b50", ".b51", ".b52", ".b53", ".b54", ".b55", ".b56", ".b57", ".b60", ".b61", ".b62", ".b63", ".b64", ".b65", ".b66", ".b67", ".b70", ".b71", ".b72", ".b73", ".b74", ".b75", ".b76", ".b77"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Bsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B00
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b00").is_ok() {
                    return Ok(Bsel::B00);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B01
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b01").is_ok() {
                    return Ok(Bsel::B01);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B02
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b02").is_ok() {
                    return Ok(Bsel::B02);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B03
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b03").is_ok() {
                    return Ok(Bsel::B03);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B04
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b04").is_ok() {
                    return Ok(Bsel::B04);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B05
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b05").is_ok() {
                    return Ok(Bsel::B05);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B06
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b06").is_ok() {
                    return Ok(Bsel::B06);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B07
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b07").is_ok() {
                    return Ok(Bsel::B07);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b10").is_ok() {
                    return Ok(Bsel::B10);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B11
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b11").is_ok() {
                    return Ok(Bsel::B11);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B12
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b12").is_ok() {
                    return Ok(Bsel::B12);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B13
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b13").is_ok() {
                    return Ok(Bsel::B13);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B14
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b14").is_ok() {
                    return Ok(Bsel::B14);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B15
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b15").is_ok() {
                    return Ok(Bsel::B15);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Bsel::B16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B17
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b17").is_ok() {
                    return Ok(Bsel::B17);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b20").is_ok() {
                    return Ok(Bsel::B20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b21").is_ok() {
                    return Ok(Bsel::B21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B22
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b22").is_ok() {
                    return Ok(Bsel::B22);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B23
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b23").is_ok() {
                    return Ok(Bsel::B23);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B24
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b24").is_ok() {
                    return Ok(Bsel::B24);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B25
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b25").is_ok() {
                    return Ok(Bsel::B25);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B26
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b26").is_ok() {
                    return Ok(Bsel::B26);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B27
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b27").is_ok() {
                    return Ok(Bsel::B27);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b30").is_ok() {
                    return Ok(Bsel::B30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b31").is_ok() {
                    return Ok(Bsel::B31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Bsel::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B33
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b33").is_ok() {
                    return Ok(Bsel::B33);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B34
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b34").is_ok() {
                    return Ok(Bsel::B34);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B35
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b35").is_ok() {
                    return Ok(Bsel::B35);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B36
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b36").is_ok() {
                    return Ok(Bsel::B36);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B37
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b37").is_ok() {
                    return Ok(Bsel::B37);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B40
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b40").is_ok() {
                    return Ok(Bsel::B40);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B41
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b41").is_ok() {
                    return Ok(Bsel::B41);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B42
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b42").is_ok() {
                    return Ok(Bsel::B42);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B43
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b43").is_ok() {
                    return Ok(Bsel::B43);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B44
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b44").is_ok() {
                    return Ok(Bsel::B44);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B45
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b45").is_ok() {
                    return Ok(Bsel::B45);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B46
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b46").is_ok() {
                    return Ok(Bsel::B46);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B47
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b47").is_ok() {
                    return Ok(Bsel::B47);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B50
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b50").is_ok() {
                    return Ok(Bsel::B50);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B51
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b51").is_ok() {
                    return Ok(Bsel::B51);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B52
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b52").is_ok() {
                    return Ok(Bsel::B52);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B53
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b53").is_ok() {
                    return Ok(Bsel::B53);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B54
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b54").is_ok() {
                    return Ok(Bsel::B54);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B55
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b55").is_ok() {
                    return Ok(Bsel::B55);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B56
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b56").is_ok() {
                    return Ok(Bsel::B56);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B57
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b57").is_ok() {
                    return Ok(Bsel::B57);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B60
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b60").is_ok() {
                    return Ok(Bsel::B60);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B61
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b61").is_ok() {
                    return Ok(Bsel::B61);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B62
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b62").is_ok() {
                    return Ok(Bsel::B62);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B63
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b63").is_ok() {
                    return Ok(Bsel::B63);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Bsel::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B65
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b65").is_ok() {
                    return Ok(Bsel::B65);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B66
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b66").is_ok() {
                    return Ok(Bsel::B66);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B67
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b67").is_ok() {
                    return Ok(Bsel::B67);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B70
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b70").is_ok() {
                    return Ok(Bsel::B70);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B71
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b71").is_ok() {
                    return Ok(Bsel::B71);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B72
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b72").is_ok() {
                    return Ok(Bsel::B72);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B73
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b73").is_ok() {
                    return Ok(Bsel::B73);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B74
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b74").is_ok() {
                    return Ok(Bsel::B74);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B75
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b75").is_ok() {
                    return Ok(Bsel::B75);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B76
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b76").is_ok() {
                    return Ok(Bsel::B76);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B77
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b77").is_ok() {
                    return Ok(Bsel::B77);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b00", ".b01", ".b02", ".b03", ".b04", ".b05", ".b06", ".b07", ".b10", ".b11", ".b12", ".b13", ".b14", ".b15", ".b16", ".b17", ".b20", ".b21", ".b22", ".b23", ".b24", ".b25", ".b26", ".b27", ".b30", ".b31", ".b32", ".b33", ".b34", ".b35", ".b36", ".b37", ".b40", ".b41", ".b42", ".b43", ".b44", ".b45", ".b46", ".b47", ".b50", ".b51", ".b52", ".b53", ".b54", ".b55", ".b56", ".b57", ".b60", ".b61", ".b62", ".b63", ".b64", ".b65", ".b66", ".b67", ".b70", ".b71", ".b72", ".b73", ".b74", ".b75", ".b76", ".b77"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Mask {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Mask::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Mask::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B10B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b10.b2").is_ok() {
                    return Ok(Mask::B10B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b20").is_ok() {
                    return Ok(Mask::B20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b21").is_ok() {
                    return Ok(Mask::B21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B210
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b210").is_ok() {
                    return Ok(Mask::B210);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3").is_ok() {
                    return Ok(Mask::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b30").is_ok() {
                    return Ok(Mask::B30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b31").is_ok() {
                    return Ok(Mask::B31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B310
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b310").is_ok() {
                    return Ok(Mask::B310);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Mask::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B320
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b320").is_ok() {
                    return Ok(Mask::B320);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B321
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b321").is_ok() {
                    return Ok(Mask::B321);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3210
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3210").is_ok() {
                    return Ok(Mask::B3210);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b0", ".b1", ".b10.b2", ".b20", ".b21", ".b210", ".b3", ".b30", ".b31", ".b310", ".b32", ".b320", ".b321", ".b3210"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Atype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Atype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Dtype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Dtype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Btype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Btype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vop4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop4")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(Vop4DtypeAtypeBtypeSat {
                dtype,
                atype,
                btype,
                sat,
                d,
                mask,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


    impl PtxParser for Vop4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop4")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".add")?;
            let add = ();
            let d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(Vop4DtypeAtypeBtypeAdd {
                dtype,
                atype,
                btype,
                add,
                d,
                mask,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


}

