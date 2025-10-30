use crate::util::*;
use ptx_parser::r#type::common::RegisterOperand;
use ptx_parser::r#type::instruction::xor::DataType;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::xor::Xor};

#[test]
fn parses_xor_pred() {
    assert_eq!(
        parse::<Xor>("xor.pred %p0, %p1, %p2;"),
        Xor {
            data_type: DataType::Pred,
            destination: RegisterOperand::Single("%p0".into()),
            a: RegisterOperand::Single("%p1".into()),
            b: RegisterOperand::Single("%p2".into()),
        }
    );
    assert_roundtrip::<Xor>("xor.pred %p0, %p1, %p2;");
}

#[test]
fn parses_xor_bitwise() {
    assert_eq!(
        parse::<Xor>("xor.b32 %r3, %r1, %r2;"),
        Xor {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Xor>("xor.b32 %r3, %r1, %r2;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Xor>("xor.u32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Xor>("and.b32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Xor>("xor.b16 %r0, %r1, %r2")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
