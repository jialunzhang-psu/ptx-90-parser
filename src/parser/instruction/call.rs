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

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::call::section_0::*;

    impl PtxParser for CallUni {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect(&PtxToken::LParen)?;
            let ret_param = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let func = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni {
                uni,
                ret_param,
                func,
                param_list,
            })
        }
    }


    impl PtxParser for CallUni1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let func = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni1 {
                uni,
                func,
                param_list,
            })
        }
    }


    impl PtxParser for CallUni2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let func = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni2 {
                uni,
                func,
            })
        }
    }


    impl PtxParser for CallUni3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect(&PtxToken::LParen)?;
            let ret_param = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let flist = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni3 {
                uni,
                ret_param,
                fptr,
                param_list,
                flist,
            })
        }
    }


    impl PtxParser for CallUni4 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let flist = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni4 {
                uni,
                fptr,
                param_list,
                flist,
            })
        }
    }


    impl PtxParser for CallUni5 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let flist = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni5 {
                uni,
                fptr,
                flist,
            })
        }
    }


    impl PtxParser for CallUni6 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect(&PtxToken::LParen)?;
            let ret_param = GeneralOperand::parse(stream)?;
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let fproto = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni6 {
                uni,
                ret_param,
                fptr,
                param_list,
                fproto,
            })
        }
    }


    impl PtxParser for CallUni7 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect(&PtxToken::LParen)?;
            let mut param_list = Vec::new();
            // Parse comma-separated operands
            loop {
                // Try to parse an operand
                let saved_pos = stream.position();
                match GeneralOperand::parse(stream) {
                    Ok(operand) => {
                        param_list.push(operand);
                        // Check for comma
                        if stream.expect(&PtxToken::Comma).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        stream.set_position(saved_pos);
                        break;
                    }
                }
            }
            stream.expect(&PtxToken::RParen)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let fproto = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni7 {
                uni,
                fptr,
                param_list,
                fproto,
            })
        }
    }


    impl PtxParser for CallUni8 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("call")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let fptr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let fproto = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CallUni8 {
                uni,
                fptr,
                fproto,
            })
        }
    }


}

