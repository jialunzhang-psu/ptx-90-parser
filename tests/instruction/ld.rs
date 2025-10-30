use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::ld::{
            CacheHint, CacheOperator, DataType, Destination, DestinationElement,
            DestinationElements, Eviction, Generic, Ld, Level1EvictionPriority,
            Level2EvictionPriority, Mmio, MmioStateSpace, PrefetchSize, Scope, Scoped,
            ScopedStateSpace, SharedState, StateSpace, Vector, Volatile,
        },
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_generic_load_with_unified_cache_policy() {
    let source = "ld.global.ca.L2::cache_hint.L2::128B.u32 %r1, [%rd2].unified, %rd3;";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Generic(Generic {
            weak: false,
            state_space: Some(StateSpace::Global),
            cache_operator: Some(CacheOperator::Ca),
            cache_hint: Some(CacheHint::L2CacheHint),
            prefetch_size: Some(PrefetchSize::L2128B),
            vector: None,
            data_type: DataType::U32,
            destination: Destination::Scalar(reg("%r1")),
            address: address_from_register("%rd2"),
            unified: true,
            cache_policy: Some(reg("%rd3")),
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn parses_eviction_load_with_vector() {
    let source = "ld.weak.shared::cluster.L1::evict_first.L2::evict_last.L2::cache_hint.L2::256B.v4.f32 {%f0, %f1, %f2, _}, [%rd4], %rd6;";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Eviction(Eviction {
            weak: true,
            state_space: Some(StateSpace::Shared(SharedState::Cluster)),
            level1: Some(Level1EvictionPriority::EvictFirst),
            level2: Some(Level2EvictionPriority::EvictLast),
            cache_hint: Some(CacheHint::L2CacheHint),
            prefetch_size: Some(PrefetchSize::L2256B),
            vector: Some(Vector::V4),
            data_type: DataType::F32,
            destination: Destination::Vector(DestinationElements::V4([
                DestinationElement::Register(reg("%f0")),
                DestinationElement::Register(reg("%f1")),
                DestinationElement::Register(reg("%f2")),
                DestinationElement::Sink,
            ])),
            address: address_from_register("%rd4"),
            unified: false,
            cache_policy: Some(reg("%rd6")),
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn parses_volatile_load_with_prefetch() {
    let source = "ld.volatile.shared::cta.L2::64B.b32 %r2, [%rd1];";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Volatile(Volatile {
            state_space: Some(ScopedStateSpace::Shared(SharedState::Cta)),
            prefetch_size: Some(PrefetchSize::L264B),
            vector: None,
            data_type: DataType::B32,
            destination: Destination::Scalar(reg("%r2")),
            address: address_from_register("%rd1"),
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn parses_relaxed_scoped_load_with_cache_policy() {
    let source =
        "ld.relaxed.gpu.shared::cluster.L2::cache_hint.L2::128B.v2.f32 {%f0, %f1}, [%rd0], %rd2;";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Relaxed(Scoped {
            scope: Scope::Gpu,
            state_space: Some(ScopedStateSpace::Shared(SharedState::Cluster)),
            level1: None,
            level2: None,
            cache_hint: Some(CacheHint::L2CacheHint),
            prefetch_size: Some(PrefetchSize::L2128B),
            vector: Some(Vector::V2),
            data_type: DataType::F32,
            destination: Destination::Vector(DestinationElements::V2([
                DestinationElement::Register(reg("%f0")),
                DestinationElement::Register(reg("%f1")),
            ])),
            address: address_from_register("%rd0"),
            cache_policy: Some(reg("%rd2")),
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn parses_acquire_scoped_load_without_cache_policy() {
    let source = "ld.acquire.sys.global.s64 %rd1, [%rd2];";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Acquire(Scoped {
            scope: Scope::Sys,
            state_space: Some(ScopedStateSpace::Global),
            level1: None,
            level2: None,
            cache_hint: None,
            prefetch_size: None,
            vector: None,
            data_type: DataType::S64,
            destination: Destination::Scalar(reg("%rd1")),
            address: address_from_register("%rd2"),
            cache_policy: None,
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn parses_mmio_load() {
    let source = "ld.mmio.relaxed.sys.global.u32 %r4, [%rd3];";
    assert_eq!(
        parse::<Ld>(source),
        Ld::Mmio(Mmio {
            state_space: Some(MmioStateSpace::Global),
            data_type: DataType::U32,
            destination: Destination::Scalar(reg("%r4")),
            address: address_from_register("%rd3"),
        })
    );
    assert_roundtrip::<Ld>(source);
}

#[test]
fn rejects_cache_policy_for_volatile_load() {
    let err = parse_result::<Ld>("ld.volatile.f32 %f1, [%rd1], %rd2;")
        .expect_err("cache policy not allowed for volatile loads");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
