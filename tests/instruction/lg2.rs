use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::lg2::Lg2,
};

#[test]
fn parses_lg2_without_ftz_modifier() {
    assert_eq!(
        parse::<Lg2>("lg2.approx.f32 %f0, %f1;"),
        Lg2 {
            approx: (),
            ftz: false,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f0".into())),
            a: Operand::Register(RegisterOperand::Single("%f1".into())),
        }
    );
    assert_roundtrip::<Lg2>("lg2.approx.f32 %f0, %f1;");
}

#[test]
fn parses_lg2_with_ftz_modifier() {
    assert_eq!(
        parse::<Lg2>("lg2.approx.ftz.f32 %f2, %f3;"),
        Lg2 {
            approx: (),
            ftz: true,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%f2".into())),
            a: Operand::Register(RegisterOperand::Single("%f3".into())),
        }
    );
    assert_roundtrip::<Lg2>("lg2.approx.ftz.f32 %f2, %f3;");
}
