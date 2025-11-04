use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::*,
    r#type::instruction::sin::Sin,
};

#[test]
fn parses_sin_without_ftz_modifier() {
    assert_eq!(
        parse::<Sin>("sin.approx.f32 %f0, %f1;"),
        Sin {
            approx: (),
            ftz: false,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
        }
    );
    assert_roundtrip::<Sin>("sin.approx.f32 %f0, %f1;");
}

#[test]
fn parses_sin_with_ftz_modifier() {
    assert_eq!(
        parse::<Sin>("sin.approx.ftz.f32 %f2, %f3;"),
        Sin {
            approx: (),
            ftz: true,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f2".into())),
            a: Operand::Register(RegisterOperand::Single("%f3".into())),
        }
    );
    assert_roundtrip::<Sin>("sin.approx.ftz.f32 %f2, %f3;");
}

#[test]
fn rejects_sin_with_invalid_modifier() {
    let err = parse_result::<Sin>("sin.fast.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sin_missing_approx_modifier() {
    let err = parse_result::<Sin>("sin.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
