use crate::util::{parse, parse_result};
use ptx_parser::r#type::common::*;
use ptx_parser::r#type::instruction::applypriority::*;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::applypriority::Applypriority};

#[test]
fn parses_applypriority_instruction() {
    assert_eq!(
        parse::<Applypriority>("applypriority.level::.L2::evict_normal [%rd1], 128;"),
        Applypriority {
            global: false,
            eviction_priority: EvictionPriority::L2EvictNormal,
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None,
            ),
            size: Size::B128,
        }
    );
}

#[test]
fn parses_applypriority_with_global_modifier() {
    assert_eq!(
        parse::<Applypriority>("applypriority.global.level::.L2::evict_normal [buffer], 128;"),
        Applypriority {
            global: true,
            eviction_priority: EvictionPriority::L2EvictNormal,
            address: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("buffer".into())),
                None,
            ),
            size: Size::B128,
        }
    );
}

#[test]
fn rejects_unknown_eviction_priority() {
    let err = parse_result::<Applypriority>("applypriority.level::.L2::evict_high [%rd1], 128;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_size_literal() {
    let err = parse_result::<Applypriority>("applypriority.level::.L2::evict_normal [%rd1], 64;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
