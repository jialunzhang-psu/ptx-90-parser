use crate::util::*;
use ptx_parser::r#type::common::{Operand, RegisterOperand};
use ptx_parser::r#type::instruction::xor::Type;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::xor::Xor};

#[test]
fn parses_xor_pred() {
    assert_eq!(
        parse::<Xor>("xor.pred %p0, %p1, %p2;"),
        Xor {
            type_: Type::Pred,
            d: Operand::Register(RegisterOperand::Single("%p0".into())),
            a: Operand::Register(RegisterOperand::Single("%p1".into())),
            b: Operand::Register(RegisterOperand::Single("%p2".into())),
        }
    );
    assert_roundtrip::<Xor>("xor.pred %p0, %p1, %p2;");
}

#[test]
fn parses_xor_bitwise() {
    assert_eq!(
        parse::<Xor>("xor.b32 %r3, %r1, %r2;"),
        Xor {
            type_: Type::B32,
            d: Operand::Register(RegisterOperand::Single("%r3".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
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
