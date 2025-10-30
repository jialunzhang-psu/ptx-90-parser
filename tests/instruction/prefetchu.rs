use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressOperand, Immediate},
        instruction::prefetchu::{Prefetchu, PrefetchuLevel},
    },
};

fn immediate_address(value: &str) -> AddressOperand {
    AddressOperand::ImmediateAddress(Immediate(value.into()))
}

#[test]
fn parses_prefetchu_instruction() {
    assert_roundtrip::<Prefetchu>("prefetchu.L1 [0];");
    assert_eq!(
        parse::<Prefetchu>("prefetchu.L1 [0];"),
        Prefetchu {
            level: PrefetchuLevel::L1,
            address: immediate_address("0"),
        }
    );
}

#[test]
fn rejects_prefetchu_with_invalid_level() {
    assert_roundtrip::<Prefetchu>("prefetchu.L1 [0];");
    let err = parse_result::<Prefetchu>("prefetchu.L2 [0];")
        .expect_err("prefetchu should only accept .L1 level");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_prefetchu_without_semicolon() {
    assert_roundtrip::<Prefetchu>("prefetchu.L1 [0];");
    let err = parse_result::<Prefetchu>("prefetchu.L1 [0]")
        .expect_err("prefetchu requires a terminating semicolon");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
