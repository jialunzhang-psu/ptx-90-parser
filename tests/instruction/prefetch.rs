use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressOperand, Immediate},
        instruction::prefetch::{Eviction, Level, Prefetch, Space, TensorMapSpace},
    },
};

fn immediate_address(value: &str) -> AddressOperand {
    AddressOperand::ImmediateAddress(Immediate(value.into()))
}

#[test]
fn parses_prefetch_without_space() {
    assert_eq!(
        parse::<Prefetch>("prefetch.L1 [0];"),
        Prefetch::DataCache {
            space: None,
            level: Level::L1,
            address: immediate_address("0"),
        }
    );
}

#[test]
fn parses_prefetch_with_space() {
    assert_eq!(
        parse::<Prefetch>("prefetch.local.L2 [0];"),
        Prefetch::DataCache {
            space: Some(Space::Local),
            level: Level::L2,
            address: immediate_address("0"),
        }
    );
}

#[test]
fn parses_prefetch_global_eviction() {
    assert_eq!(
        parse::<Prefetch>("prefetch.global.L2::evict_last [0];"),
        Prefetch::GlobalEviction {
            eviction: Eviction::L2EvictLast,
            address: immediate_address("0"),
        }
    );
}

#[test]
fn parses_prefetch_uniform() {
    assert_eq!(
        parse::<Prefetch>("prefetchu.L1 [0];"),
        Prefetch::Uniform {
            address: immediate_address("0"),
        }
    );
}

#[test]
fn parses_prefetch_tensormap_without_space() {
    assert_eq!(
        parse::<Prefetch>("prefetch.tensormap [0];"),
        Prefetch::TensorMap {
            space: None,
            address: immediate_address("0"),
        }
    );
}

#[test]
fn parses_prefetch_tensormap_with_space() {
    assert_eq!(
        parse::<Prefetch>("prefetch.const.tensormap [0];"),
        Prefetch::TensorMap {
            space: Some(TensorMapSpace::Const),
            address: immediate_address("0"),
        }
    );
}

#[test]
fn rejects_prefetch_eviction_without_global() {
    let err = parse_result::<Prefetch>("prefetch.L2::evict_normal [0];")
        .expect_err("prefetch eviction should require .global modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_prefetchu_with_invalid_level() {
    let err = parse_result::<Prefetch>("prefetchu.L2 [0];")
        .expect_err("prefetchu should only allow .L1 level");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_tensormap_with_invalid_space() {
    let err = parse_result::<Prefetch>("prefetch.global.tensormap [0];")
        .expect_err("tensormap prefetch should only allow .const or .param space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
