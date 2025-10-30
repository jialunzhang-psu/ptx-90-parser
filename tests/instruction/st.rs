use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::st::{
            CacheHint, CacheOperator, DataType, Eviction, Generic, Level1EvictionPriority,
            Level2EvictionPriority, Mmio, MmioStateSpace, SharedState, Source, St, StateSpace,
            Vector, VectorElement, VectorElements,
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
fn parses_generic_store_with_cache_policy() {
    let source = "st.global.cg.L2::cache_hint.u32 [%rd1], %r2, %rd3;";
    assert_eq!(
        parse::<St>(source),
        St::Generic(Generic {
            weak: false,
            state_space: Some(StateSpace::Global),
            cache_operator: Some(CacheOperator::Cg),
            cache_hint: Some(CacheHint::L2CacheHint),
            vector: None,
            data_type: DataType::U32,
            address: address_from_register("%rd1"),
            source: Source::Register(reg("%r2")),
            cache_policy: Some(reg("%rd3")),
        })
    );
    assert_roundtrip::<St>(source);
}

#[test]
fn parses_eviction_store_with_vector() {
    let source = "st.shared::cta.L1::evict_last.L2::evict_first.L2::cache_hint.v2.f32 [%rd1], {%f2, %f3}, %rd4;";
    assert_eq!(
        parse::<St>(source),
        St::Eviction(Eviction {
            weak: false,
            state_space: Some(StateSpace::Shared(SharedState::Cta)),
            level1: Some(Level1EvictionPriority::EvictLast),
            level2: Some(Level2EvictionPriority::EvictFirst),
            cache_hint: Some(CacheHint::L2CacheHint),
            vector: Some(Vector::V2),
            data_type: DataType::F32,
            address: address_from_register("%rd1"),
            source: Source::Vector(VectorElements::V2([
                VectorElement::Register(reg("%f2")),
                VectorElement::Register(reg("%f3")),
            ])),
            cache_policy: Some(reg("%rd4")),
        })
    );
    assert_roundtrip::<St>(source);
}

#[test]
fn parses_mmio_store() {
    let source = "st.mmio.relaxed.sys.global.b32 [%rd0], %r1;";
    assert_eq!(
        parse::<St>(source),
        St::Mmio(Mmio {
            state_space: Some(MmioStateSpace::Global),
            data_type: DataType::B32,
            address: address_from_register("%rd0"),
            source: reg("%r1"),
        })
    );
    assert_roundtrip::<St>(source);
}

#[test]
fn rejects_cache_policy_for_volatile_store() {
    let err = parse_result::<St>("st.volatile.f32 [%rd1], %f2, %r3;")
        .expect_err("cache policy not allowed for volatile stores");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
