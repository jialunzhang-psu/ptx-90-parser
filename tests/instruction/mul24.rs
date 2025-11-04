use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::mul24::{Type as Mul24DataType, Mode as Mul24Mode, Mul24},
};

#[test]
fn parses_mul24_lo_with_unsigned_type() {
    assert_eq!(
        parse::<Mul24>("mul24.lo.u32 %r0, %r1, %r2;"),
        Mul24 {
            mode: Mul24Mode::Lo,
            type_: Mul24DataType::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Mul24>("mul24.lo.u32 %r0, %r1, %r2;");
}
#[test]
fn parses_mul24_hi_with_signed_type() {
    assert_eq!(
        parse::<Mul24>("mul24.hi.s32 %r6, %r7, %r8;"),
        Mul24 {
            mode: Mul24Mode::Hi,
            type_: Mul24DataType::S32,
            d: Operand::Register(RegisterOperand::Single("%r6".into())),
            a: Operand::Register(RegisterOperand::Single("%r7".into())),
            b: Operand::Register(RegisterOperand::Single("%r8".into())),
        }
    );
    assert_roundtrip::<Mul24>("mul24.hi.s32 %r6, %r7, %r8;");
}

#[test]
fn rejects_mul24_with_invalid_mode() {
    let err =
        parse_result::<Mul24>("mul24.wide.u32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mul24_with_invalid_type() {
    let err = parse_result::<Mul24>("mul24.lo.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail with invalid type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mul24_missing_semicolon() {
    let err = parse_result::<Mul24>("mul24.hi.s32 %r0, %r1, %r2")
        .expect_err("parse should fail without a terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_mul24_without_mode() {
    let _ = parse_result::<Mul24>("mul24.u32 %r3, %r4, %r5;")
        .expect_err("parse should fail when mode is missing");
}
