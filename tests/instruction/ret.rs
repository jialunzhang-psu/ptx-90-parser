use crate::util::{parse, parse_result};
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::ret::Ret};

#[test]
fn parses_default_return_instruction() {
    assert_eq!(parse::<Ret>("ret;"), Ret::Default);
}

#[test]
fn parses_uniform_return_instruction() {
    assert_eq!(parse::<Ret>("ret.uni;"), Ret::Uniform);
}

#[test]
fn rejects_unexpected_opcode() {
    let err = parse_result::<Ret>("exit;").expect_err("parse should fail when opcode is not ret");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Ret>("ret").expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
