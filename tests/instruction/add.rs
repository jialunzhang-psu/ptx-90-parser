use crate::util::*;
use ptx_parser::r#type::common::{Immediate, Operand, RegisterOperand};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::add::{Add1, Add2, Type},
};

#[test]
fn parses_add_with_unsigned_type() {
    assert_eq!(
        parse::<Add1>("add.u32 %r0, %r1, %r2;"),
        Add1 {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Add1>("add.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_add_with_saturate_and_immediate_operand() {
    assert_eq!(
        parse::<Add2>("add.sat.s32 %r3, %r4, 1;"),
        Add2 {
            sat: true,
            s32: (),
            d: Operand::Register(RegisterOperand::Single("%r3".into())),
            a: Operand::Register(RegisterOperand::Single("%r4".into())),
            b: Operand::Immediate(Immediate("1".into())),
        }
    );
    assert_roundtrip::<Add2>("add.sat.s32 %r3, %r4, 1;");
}

#[test]
fn rejects_add_with_saturate_on_unsigned_type() {
    let err = parse_result::<Add1>("add.sat.u32 %r0, %r1, %r2;")
        .expect_err("parse should fail when .sat is used with unsigned type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_add_with_invalid_type() {
    let err = parse_result::<Add1>("add.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_add_missing_semicolon() {
    let err = parse_result::<Add1>("add.u16 %r0, %r1, %r2")
        .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
