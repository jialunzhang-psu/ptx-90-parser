use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, RegisterOperand},
        instruction::nanosleep::Nanosleep,
    },
};

#[test]
fn parses_nanosleep_with_register_operand() {
    assert_eq!(
        parse::<Nanosleep>("nanosleep.u32 %r3;"),
        Nanosleep {
            u32: (),
            t: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Nanosleep>("nanosleep.u32 %r3;");
}

#[test]
fn parses_nanosleep_with_immediate_operand() {
    assert_eq!(
        parse::<Nanosleep>("nanosleep.u32 128;"),
        Nanosleep {
            u32: (),
            t: Operand::Immediate(Immediate("128".into())),
        }
    );
    assert_roundtrip::<Nanosleep>("nanosleep.u32 128;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Nanosleep>("nanosleep.s32 %r1;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_operand() {
    let err = parse_result::<Nanosleep>("nanosleep.u32 label;")
        .expect_err("parsing should fail for invalid operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
