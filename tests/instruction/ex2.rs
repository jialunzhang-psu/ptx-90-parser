use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::ex2::{DataType as Ex2DataType, Ex2},
};

#[test]
fn parses_ex2_without_ftz_modifier() {
    assert_eq!(
        parse::<Ex2>("ex2.approx.f32 %f0, %f1;"),
        Ex2 {
            flush_to_zero: false,
            data_type: Ex2DataType::F32,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        }
    );
}

#[test]
fn parses_ex2_with_ftz_modifier() {
    assert_eq!(
        parse::<Ex2>("ex2.approx.ftz.f32 %f2, %f3;"),
        Ex2 {
            flush_to_zero: true,
            data_type: Ex2DataType::F32,
            destination: RegisterOperand::Single("%f2".into()),
            source: RegisterOperand::Single("%f3".into()),
        }
    );
}

#[test]
fn rejects_ex2_with_invalid_modifier() {
    let err = parse_result::<Ex2>("ex2.fast.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_ex2_missing_approx_modifier() {
    let err = parse_result::<Ex2>("ex2.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
