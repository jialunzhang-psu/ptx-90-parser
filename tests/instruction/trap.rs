use crate::util::*;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::trap::Trap};

#[test]
fn parses_simple_trap_instruction() {
    assert_eq!(parse::<Trap>("trap;"), Trap);
    assert_roundtrip::<Trap>("trap;");
}

#[test]
fn rejects_unexpected_opcode() {
    let err = parse_result::<Trap>("exit;").expect_err("parse should fail when opcode is not trap");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err =
        parse_result::<Trap>("trap").expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
