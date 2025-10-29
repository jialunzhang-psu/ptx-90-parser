use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::mul24::{DataType as Mul24DataType, Mode as Mul24Mode, Mul24},
};

#[test]
fn parses_mul24_lo_with_unsigned_type() {
    assert_eq!(
        parse::<Mul24>("mul24.lo.u32 %r0, %r1, %r2;"),
        Mul24 {
            mode: Mul24Mode::Lo,
            data_type: Mul24DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_mul24_with_default_mode() {
    assert_eq!(
        parse::<Mul24>("mul24.u32 %r3, %r4, %r5;"),
        Mul24 {
            mode: Mul24Mode::Lo,
            data_type: Mul24DataType::U32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r4".into()),
            b: RegisterOperand::Single("%r5".into()),
        }
    );
}

#[test]
fn parses_mul24_hi_with_signed_type() {
    assert_eq!(
        parse::<Mul24>("mul24.hi.s32 %r6, %r7, %r8;"),
        Mul24 {
            mode: Mul24Mode::Hi,
            data_type: Mul24DataType::S32,
            destination: RegisterOperand::Single("%r6".into()),
            a: RegisterOperand::Single("%r7".into()),
            b: RegisterOperand::Single("%r8".into()),
        }
    );
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
