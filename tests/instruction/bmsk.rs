use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Immediate, Operand, RegisterOperand},
    r#type::instruction::bmsk::{Bmsk, Mode},
};

#[test]
fn parses_bmsk_clamp_register_operands() {
    assert_eq!(
        parse::<Bmsk>("bmsk.clamp.b32 %r1, %r2, %r3;"),
        Bmsk {
            mode: Mode::Clamp,
            destination: RegisterOperand::Single("%r1".into()),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
}

#[test]
fn parses_bmsk_wrap_with_immediate() {
    assert_eq!(
        parse::<Bmsk>("bmsk.wrap.b32 %r4, 0xff, %r5;"),
        Bmsk {
            mode: Mode::Wrap,
            destination: RegisterOperand::Single("%r4".into()),
            a: Operand::Immediate(Immediate("0xff".into())),
            b: Operand::Register(RegisterOperand::Single("%r5".into())),
        }
    );
}

#[test]
fn rejects_bmsk_with_invalid_mode() {
    let err = parse_result::<Bmsk>("bmsk.foo.b32 %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid mode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_bmsk_with_invalid_type() {
    let err = parse_result::<Bmsk>("bmsk.clamp.b16 %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
