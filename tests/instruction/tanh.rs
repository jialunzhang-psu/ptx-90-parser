use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::tanh::{Type, Tanh},
    },
};

#[test]
fn parses_tanh_instruction() {
    assert_eq!(
        parse::<Tanh>("tanh.approx.f32 %f0, %f1;"),
        Tanh {
            approx: (),
            type_: Type::F32,
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
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
fn parses_tanh_with_f16_data_type() {
    assert_eq!(
        parse::<Tanh>("tanh.approx.f16 %f0, %f1;"),
        Tanh {
            approx: (),
            type_: Type::F16,
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
        }
    );
    assert_roundtrip::<Tanh>("tanh.approx.f16 %f0, %f1;");
}
