use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressOperand, Immediate, Operand},
        instruction::discard::{Discard, Level},
    },
};

#[test]
fn parses_discard_without_space_modifier() {
    assert_roundtrip::<Discard>("discard.L2 [0], 128;");
    assert_eq!(
        parse::<Discard>("discard.L2 [0], 128;"),
        Discard {
            global: false,
            level: Level::L2,
            a: AddressOperand::ImmediateAddress(Immediate("0".into())),
            size: Operand::Immediate(Immediate("128".into())),
        }
    );
}

#[test]
fn parses_discard_with_global_space() {
    assert_roundtrip::<Discard>("discard.global.L2 [0], 128;");
    assert_eq!(
        parse::<Discard>("discard.global.L2 [0], 128;"),
        Discard {
            global: true,
            level: Level::L2,
            a: AddressOperand::ImmediateAddress(Immediate("0".into())),
            size: Operand::Immediate(Immediate("128".into())),
        }
    );
}

#[test]
fn rejects_invalid_space_modifier() {
    assert_roundtrip::<Discard>("discard.L2 [0], 128;");
    let err = parse_result::<Discard>("discard.shared.L2 [0], 128;")
        .expect_err("parse should fail when space modifier is invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn accepts_different_size_literals() {
    // The parser currently accepts any immediate value for size
    assert_eq!(
        parse::<Discard>("discard.L2 [0], 64;"),
        Discard {
            global: false,
            level: Level::L2,
            a: AddressOperand::ImmediateAddress(Immediate("0".into())),
            size: Operand::Immediate(Immediate("64".into())),
        }
    );
    assert_roundtrip::<Discard>("discard.L2 [0], 64;");
}
