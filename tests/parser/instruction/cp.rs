use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{
            AddressBase, AddressOperand, Immediate, Operand, PredicateRegister, RegisterOperand,
        },
        instruction::cp::*,
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn pred(name: &str) -> PredicateRegister {
    PredicateRegister(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

fn immediate(value: &str) -> Operand {
    Operand::Immediate(Immediate(value.into()))
}

fn operand_register(name: &str) -> Operand {
    Operand::Register(reg(name))
}

#[test]
fn parses_async_ca_basic() {
    assert_eq!(
        parse::<CpOpcode>("cp.async.ca.shared.global [%rd0], [%rd1], 4;"),
        CpOpcode::AsyncCaWithSrcSize(CpAsyncCaWithSrcSize {
            shared_space: CpSharedSpace::Default,
            cache_hint: None,
            prefetch_size: None,
            destination: address_from_register("%rd0"),
            source: address_from_register("%rd1"),
            copy_size: CpCopySize::Bytes4,
            source_size: None,
            cache_policy: None,
        })
    );
}

#[test]
fn parses_async_ca_with_source_size_immediate() {
    assert_eq!(
        parse::<CpOpcode>("cp.async.ca.shared.global [%rd2], [%rd3], 16, 8;"),
        CpOpcode::AsyncCaWithSrcSize(CpAsyncCaWithSrcSize {
            shared_space: CpSharedSpace::Default,
            cache_hint: None,
            prefetch_size: None,
            destination: address_from_register("%rd2"),
            source: address_from_register("%rd3"),
            copy_size: CpCopySize::Bytes16,
            source_size: Some(immediate("8")),
            cache_policy: None,
        })
    );
}

#[test]
fn parses_async_cg_with_source_size_and_cache_policy() {
    assert_eq!(
        parse::<CpOpcode>(
            "cp.async.cg.shared::cta.global.L2::cache_hint.L2::128B [%rd0], [%rd1], 16, %r2, %rd3;",
        ),
        CpOpcode::AsyncCgWithSrcSize(CpAsyncCgWithSrcSize {
            shared_space: CpSharedSpace::Cta,
            cache_hint: Some(CpCacheHint::L2CacheHint),
            prefetch_size: Some(CpPrefetchSize::L2128B),
            destination: address_from_register("%rd0"),
            source: address_from_register("%rd1"),
            source_size: Some(operand_register("%r2")),
            cache_policy: Some(reg("%rd3")),
        })
    );
}

#[test]
fn parses_async_ca_ignore_src_with_cache_policy() {
    assert_eq!(
        parse::<CpOpcode>("cp.async.ca.shared.global.L2::cache_hint [%rd4], [%rd5], 8, %p0, %rd6;",),
        CpOpcode::AsyncCaIgnoreSrc(CpAsyncCaIgnoreSrc {
            shared_space: CpSharedSpace::Default,
            cache_hint: Some(CpCacheHint::L2CacheHint),
            prefetch_size: None,
            destination: address_from_register("%rd4"),
            source: address_from_register("%rd5"),
            copy_size: CpCopySize::Bytes8,
            ignore_src: Some(pred("%p0")),
            cache_policy: Some(reg("%rd6")),
        })
    );
}

#[test]
fn parses_async_ca_cache_policy_without_source_size() {
    assert_eq!(
        parse::<CpOpcode>("cp.async.ca.shared.global.L2::cache_hint [%rd7], [%rd8], 16, %rd9;"),
        CpOpcode::AsyncCaWithSrcSize(CpAsyncCaWithSrcSize {
            shared_space: CpSharedSpace::Default,
            cache_hint: Some(CpCacheHint::L2CacheHint),
            prefetch_size: None,
            destination: address_from_register("%rd7"),
            source: address_from_register("%rd8"),
            copy_size: CpCopySize::Bytes16,
            source_size: None,
            cache_policy: Some(reg("%rd9")),
        })
    );
}

#[test]
fn rejects_async_cg_with_invalid_copy_size() {
    let err = parse_result::<CpOpcode>("cp.async.cg.shared.global [%rd0], [%rd1], 8;")
        .expect_err("cg variant must enforce 16-byte copy size");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_shared_space() {
    let err = parse_result::<CpOpcode>("cp.async.ca.global [%rd0], [%rd1], 4;")
        .expect_err("shared space is mandatory");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_cache_policy_without_cache_hint() {
    let err = parse_result::<CpOpcode>("cp.async.ca.shared.global [%rd0], [%rd1], 4, 2, %rd3;")
        .expect_err("cache policy requires .L2::cache_hint");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn rejects_invalid_shared_target() {
    let err = parse_result::<CpOpcode>("cp.async.ca.shared::cluster.global [%rd0], [%rd1], 4;")
        .expect_err("only .shared or .shared::cta should be accepted");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
