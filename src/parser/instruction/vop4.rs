//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10,
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n };
//! .n = { 0, 1, 2, 3, 4, 5, 6, 7};
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

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
    use crate::r#type::instruction::vop4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Asel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                |stream| {
                    stream.try_with_span(|stream| {
                        stream.with_partial_token_mode(|stream| {
                            stream.expect_string(".b")?;
                            let part0 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part1 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part2 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part3 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            Ok(((), part0, part1, part2, part3))
                        })
                    })
                },
                |(b, n, n1, n2, n3), _span| Asel::BNNNN(b, n, n1, n2, n3)
            ))
        }
    }

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Atype::U32),
                map(string_p(".s32"), |_, _span| Atype::S32)
            )
        }
    }

    impl PtxParser for Bsel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                |stream| {
                    stream.try_with_span(|stream| {
                        stream.with_partial_token_mode(|stream| {
                            stream.expect_string(".b")?;
                            let part0 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part1 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part2 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            let part3 = match stream
                                .expect_strings(&["0", "1", "2", "3", "4", "5", "6", "7"])?
                            {
                                0 => N::_0,
                                1 => N::_1,
                                2 => N::_2,
                                3 => N::_3,
                                4 => N::_4,
                                5 => N::_5,
                                6 => N::_6,
                                7 => N::_7,
                                _ => unreachable!(),
                            };
                            Ok(((), part0, part1, part2, part3))
                        })
                    })
                },
                |(b, n, n1, n2, n3), _span| Bsel::BNNNN(b, n, n1, n2, n3)
            ))
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Btype::U32),
                map(string_p(".s32"), |_, _span| Btype::S32)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for Mask {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b3210"), |_, _span| Mask::B3210),
                map(string_p(".b210"), |_, _span| Mask::B210),
                map(string_p(".b310"), |_, _span| Mask::B310),
                map(string_p(".b320"), |_, _span| Mask::B320),
                map(string_p(".b321"), |_, _span| Mask::B321),
                map(string_p(".b10"), |_, _span| Mask::B10),
                map(string_p(".b20"), |_, _span| Mask::B20),
                map(string_p(".b21"), |_, _span| Mask::B21),
                map(string_p(".b30"), |_, _span| Mask::B30),
                map(string_p(".b31"), |_, _span| Mask::B31),
                map(string_p(".b32"), |_, _span| Mask::B32),
                map(string_p(".b0"), |_, _span| Mask::B0),
                map(string_p(".b1"), |_, _span| Mask::B1),
                map(string_p(".b2"), |_, _span| Mask::B2),
                map(string_p(".b3"), |_, _span| Mask::B3)
            )
        }
    }

    impl PtxParser for Vadd4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vadd4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vadd4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vsub4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vsub4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vsub4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vavrg4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vavrg4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vavrg4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vabsdiff4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vabsdiff4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vabsdiff4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vmin4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmin4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vmin4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vmax4DtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmax4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vmax4DtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vadd4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vadd4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vadd4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vsub4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vsub4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vsub4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vavrg4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vavrg4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vavrg4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vabsdiff4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vabsdiff4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vabsdiff4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vmin4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmin4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vmin4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for Vmax4DtypeAtypeBtypeAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmax4"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vmax4DtypeAtypeBtypeAdd {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        add = add,
                        d = d,
                        mask = mask,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }
}
