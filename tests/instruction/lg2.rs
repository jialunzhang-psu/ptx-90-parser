use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::lg2::{DataType as Lg2DataType, Lg2},
};

#[test]
fn parses_lg2_without_ftz_modifier() {
    assert_eq!(
        parse::<Lg2>("lg2.approx.f32 %f0, %f1;"),
        Lg2 {
            flush_to_zero: false,
            data_type: Lg2DataType::F32,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        }
    );
    assert_roundtrip::<Lg2>("lg2.approx.f32 %f0, %f1;");
}

#[test]
fn parses_lg2_with_ftz_modifier() {
    assert_eq!(
        parse::<Lg2>("lg2.approx.ftz.f32 %f2, %f3;"),
        Lg2 {
            flush_to_zero: true,
            data_type: Lg2DataType::F32,
            destination: RegisterOperand::Single("%f2".into()),
            source: RegisterOperand::Single("%f3".into()),
        }
    );
    assert_roundtrip::<Lg2>("lg2.approx.ftz.f32 %f2, %f3;");
}

#[test]
fn rejects_lg2_with_invalid_modifier() {
    let err = parse_result::<Lg2>("lg2.fast.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_lg2_missing_approx_modifier() {
    let err = parse_result::<Lg2>("lg2.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
