use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::cos::Cos,
    },
};

#[test]
fn parses_cos_without_ftz_modifier() {
    assert_eq!(
        parse::<Cos>("cos.approx.f32 %f0, %f1;"),
        Cos {
            approx: (),
            ftz: false,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
        }
    );
    assert_roundtrip::<Cos>("cos.approx.f32 %f0, %f1;");
}

#[test]
fn parses_cos_with_ftz_modifier() {
    assert_eq!(
        parse::<Cos>("cos.approx.ftz.f32 %f2, %f3;"),
        Cos {
            approx: (),
            ftz: true,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f2".into())),
            a: Operand::Register(RegisterOperand::Single("%f3".into())),
        }
    );
    assert_roundtrip::<Cos>("cos.approx.ftz.f32 %f2, %f3;");
}

#[test]
fn rejects_cos_with_invalid_modifier() {
    let err = parse_result::<Cos>("cos.fast.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_cos_missing_approx_modifier() {
    let err = parse_result::<Cos>("cos.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
