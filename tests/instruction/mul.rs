use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::mul::{DataType as MulDataType, Mode as MulMode, Mul},
};

#[test]
fn parses_mul_lo_with_unsigned_type() {
    assert_eq!(
        parse::<Mul>("mul.lo.u32 %r0, %r1, %r2;"),
        Mul {
            mode: MulMode::Lo,
            data_type: MulDataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            lhs: RegisterOperand::Single("%r1".into()),
            rhs: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Mul>("mul.lo.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_mul_wide_with_signed_type() {
    assert_eq!(
        parse::<Mul>("mul.wide.s64 %rd3, %rd1, %rd2;"),
        Mul {
            mode: MulMode::Wide,
            data_type: MulDataType::S64,
            destination: RegisterOperand::Single("%rd3".into()),
            lhs: RegisterOperand::Single("%rd1".into()),
            rhs: RegisterOperand::Single("%rd2".into()),
        }
    );
    assert_roundtrip::<Mul>("mul.wide.s64 %rd3, %rd1, %rd2;");
}

#[test]
fn rejects_mul_with_invalid_mode() {
    let err = parse_result::<Mul>("mul.fast.u32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mul_with_invalid_type() {
    let err = parse_result::<Mul>("mul.lo.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail with bad type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mul_missing_trailing_semicolon() {
    let err = parse_result::<Mul>("mul.lo.u32 %r0, %r1, %r2")
        .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_mul_without_mode() {
    let _ = parse_result::<Mul>("mul.u32 %r4, %r5, %r6;")
        .expect_err("parse should fail when mode is missing");
}
