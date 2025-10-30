use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::RegisterOperand, instruction::or::*},
};

#[test]
fn parses_or_predicate() {
    assert_eq!(
        parse::<Or>("or.pred %p0, %p1, %p2;"),
        Or {
            data_type: DataType::Pred,
            destination: RegisterOperand::Single("%p0".into()),
            a: RegisterOperand::Single("%p1".into()),
            b: RegisterOperand::Single("%p2".into()),
        }
    );
    assert_roundtrip::<Or>("or.pred %p0, %p1, %p2;");
}

#[test]
fn parses_or_bitwise() {
    assert_eq!(
        parse::<Or>("or.b32 %r3, %r1, %r2;"),
        Or {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Or>("or.b32 %r3, %r1, %r2;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Or>("or.u32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Or>("and.b32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Or>("or.b16 %r0, %r1, %r2")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
