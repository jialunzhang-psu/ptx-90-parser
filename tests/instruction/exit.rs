use crate::util::{parse, parse_result};
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::exit::Exit};

#[test]
fn parses_simple_exit_instruction() {
    assert_eq!(parse::<Exit>("exit;"), Exit);
}

#[test]
fn rejects_unexpected_opcode() {
    let err = parse_result::<Exit>("ret;").expect_err("parse should fail when opcode is not exit");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err =
        parse_result::<Exit>("exit").expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
