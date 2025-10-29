use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressOperand, Immediate},
        instruction::discard::{Discard, Level, Size, Space},
    },
};

#[test]
fn parses_discard_without_space_modifier() {
    assert_eq!(
        parse::<Discard>("discard.L2 [0], 128;"),
        Discard {
            space: None,
            level: Level::L2,
            address: AddressOperand::ImmediateAddress(Immediate("0".into())),
            size: Size::Bytes128,
        }
    );
}

#[test]
fn parses_discard_with_global_space() {
    assert_eq!(
        parse::<Discard>("discard.global.L2 [0], 128;"),
        Discard {
            space: Some(Space::Global),
            level: Level::L2,
            address: AddressOperand::ImmediateAddress(Immediate("0".into())),
            size: Size::Bytes128,
        }
    );
}

#[test]
fn rejects_invalid_space_modifier() {
    let err = parse_result::<Discard>("discard.shared.L2 [0], 128;")
        .expect_err("parse should fail when space modifier is invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_size_literal() {
    let err = parse_result::<Discard>("discard.L2 [0], 64;")
        .expect_err("parse should fail when size literal is invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
