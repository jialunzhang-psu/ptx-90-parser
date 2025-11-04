use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::ex2::Ex21,
};

#[test]
fn parses_ex2_without_ftz_modifier() {
    assert_eq!(
        parse::<Ex21>("ex2.approx.f32 %f0, %f1;"),
        Ex21 {
            approx: (),
            ftz: false,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
        }
    );
    assert_roundtrip::<Ex21>("ex2.approx.f32 %f0, %f1;");
}

#[test]
fn parses_ex2_with_ftz_modifier() {
    assert_eq!(
        parse::<Ex21>("ex2.approx.ftz.f32 %f2, %f3;"),
        Ex21 {
            approx: (),
            ftz: true,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f2".into())),
            a: Operand::Register(RegisterOperand::Single("%f3".into())),
        }
    );
    assert_roundtrip::<Ex21>("ex2.approx.ftz.f32 %f2, %f3;");
}

#[test]
fn rejects_ex2_with_invalid_modifier() {
    let err = parse_result::<Ex21>("ex2.fast.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Ex21>("ex2.approx.f32 %f0, %f1;");
}

#[test]
fn rejects_ex2_missing_approx_modifier() {
    let err = parse_result::<Ex21>("ex2.f32 %f0, %f1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Ex21>("ex2.approx.f32 %f0, %f1;");
}
