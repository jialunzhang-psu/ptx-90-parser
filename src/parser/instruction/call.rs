//! Original PTX specification:
//!
//! // direct call to named function, func is a symbol
//! call{.uni} (ret-param), func, (param-list);
//! call{.uni} func, (param-list);
//! call{.uni} func;
//! // indirect call via pointer, with full list of call targets
//! call{.uni} (ret-param), fptr, (param-list), flist;
//! call{.uni} fptr, (param-list), flist;
//! call{.uni} fptr, flist;
//! // indirect call via pointer, with no knowledge of call targets
//! call{.uni} (ret-param), fptr, (param-list), fproto;
//! call{.uni} fptr, (param-list), fproto;
//! call{.uni} fptr, fproto;

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
    use crate::r#type::instruction::call::section_0::*;

    impl PtxParser for CallUni {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    between(lparen_p(), rparen_p(), GeneralOperand::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    semicolon_p()
                ),
                |(_, uni, ret_param, _, func, _, param_list, _), span| {
                    ok!(CallUni {
                        uni = uni,
                        ret_param = ret_param,
                        func = func,
                        param_list = param_list,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    semicolon_p()
                ),
                |(_, uni, func, _, param_list, _), span| {
                    ok!(CallUni1 {
                        uni = uni,
                        func = func,
                        param_list = param_list,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, func, _), span| {
                    ok!(CallUni2 {
                        uni = uni,
                        func = func,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    between(lparen_p(), rparen_p(), GeneralOperand::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, ret_param, _, fptr, _, param_list, _, flist, _), span| {
                    ok!(CallUni3 {
                        uni = uni,
                        ret_param = ret_param,
                        fptr = fptr,
                        param_list = param_list,
                        flist = flist,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni4 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, fptr, _, param_list, _, flist, _), span| {
                    ok!(CallUni4 {
                        uni = uni,
                        fptr = fptr,
                        param_list = param_list,
                        flist = flist,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni5 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, fptr, _, flist, _), span| {
                    ok!(CallUni5 {
                        uni = uni,
                        fptr = fptr,
                        flist = flist,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni6 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    between(lparen_p(), rparen_p(), GeneralOperand::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, ret_param, _, fptr, _, param_list, _, fproto, _), span| {
                    ok!(CallUni6 {
                        uni = uni,
                        ret_param = ret_param,
                        fptr = fptr,
                        param_list = param_list,
                        fproto = fproto,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni7 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    between(
                        lparen_p(),
                        rparen_p(),
                        sep_by(GeneralOperand::parse(), comma_p())
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, fptr, _, param_list, _, fproto, _), span| {
                    ok!(CallUni7 {
                        uni = uni,
                        fptr = fptr,
                        param_list = param_list,
                        fproto = fproto,

                    })
                },
            )
        }
    }

    impl PtxParser for CallUni8 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("call"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, fptr, _, fproto, _), span| {
                    ok!(CallUni8 {
                        uni = uni,
                        fptr = fptr,
                        fproto = fproto,

                    })
                },
            )
        }
    }
}
