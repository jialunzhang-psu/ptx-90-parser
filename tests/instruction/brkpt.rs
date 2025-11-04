use crate::util::*;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::brkpt::Brkpt};

#[test]
fn parses_brkpt_instruction() {
    assert_eq!(parse::<Brkpt>("brkpt;"), Brkpt {});
    assert_roundtrip::<Brkpt>("brkpt;");
}

#[test]
fn rejects_unexpected_opcode() {
    assert_roundtrip::<Brkpt>("brkpt;");
    let err = parse_result::<Brkpt>("trap;").expect_err("parser should reject non-brkpt opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    assert_roundtrip::<Brkpt>("brkpt;");
    let err = parse_result::<Brkpt>("brkpt").expect_err("parser should reject missing semicolon");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
