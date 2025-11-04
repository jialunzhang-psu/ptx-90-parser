use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::szext::{Type, Mode, Szext},
    },
};

#[test]
fn parses_clamp_u32() {
    assert_eq!(
        parse::<Szext>("szext.clamp.u32 %r1, %r2, %r3;"),
        Szext {
            mode: Mode::Clamp,
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Szext>("szext.clamp.u32 %r1, %r2, %r3;");
}

#[test]
fn parses_wrap_s32() {
    assert_eq!(
        parse::<Szext>("szext.wrap.s32 %rd4, %rd5, %rd6;"),
        Szext {
            mode: Mode::Wrap,
            type_: Type::S32,
            d: Operand::Register(RegisterOperand::Single("%rd4".into())),
            a: Operand::Register(RegisterOperand::Single("%rd5".into())),
            b: Operand::Register(RegisterOperand::Single("%rd6".into())),
        }
    );
    assert_roundtrip::<Szext>("szext.wrap.s32 %rd4, %rd5, %rd6;");
}

#[test]
fn rejects_unknown_mode() {
    let error = parse_result::<Szext>("szext.bounce.u32 %r1, %r2, %r3;")
        .expect_err("mode must be clamp or wrap");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unknown_data_type() {
    let error = parse_result::<Szext>("szext.wrap.s64 %r1, %r2, %r3;")
        .expect_err("data type must be s32 or u32");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
