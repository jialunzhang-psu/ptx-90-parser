use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::tanh::{Approximation, DataType, Tanh},
    },
};

#[test]
fn parses_tanh_instruction() {
    assert_eq!(
        parse::<Tanh>("tanh.approx.f32 %f0, %f1;"),
        Tanh {
            approximation: Approximation::Approx,
            data_type: DataType::F32,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        }
    );
    assert_roundtrip::<Tanh>("tanh.approx.f32 %f0, %f1;");
}

#[test]
fn rejects_tanh_with_invalid_modifier() {
    let err = parse_result::<Tanh>("tanh.fast.f32 %f0, %f1;")
        .expect_err("parse should fail with modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_tanh_with_invalid_data_type() {
    let err = parse_result::<Tanh>("tanh.approx.f16 %f0, %f1;")
        .expect_err("parse should fail with data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
