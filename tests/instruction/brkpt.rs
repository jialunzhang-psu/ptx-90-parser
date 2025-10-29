use crate::util::{parse, parse_result};
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::brkpt::Brkpt};

#[test]
fn parses_brkpt_instruction() {
    assert_eq!(parse::<Brkpt>("brkpt;"), Brkpt);
}

#[test]
fn rejects_unexpected_opcode() {
    let err = parse_result::<Brkpt>("trap;").expect_err("parser should reject non-brkpt opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Brkpt>("brkpt").expect_err("parser should reject missing semicolon");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
