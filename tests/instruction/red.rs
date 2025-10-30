use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::red::{
            CacheHint, HalfWordType, PackedType, RedOpcode, Scalar, ScalarAddNoFtz,
            ScalarAddNoFtzType, ScalarOperation, ScalarType, Scope, Semantics, SharedSpace,
            StateSpace, Vec16Registers, Vec32ElementType, Vec32Registers, VectorAdd32, VectorHalf,
            VectorOperation, VectorPacked, VectorStateSpace,
        },
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<RedOpcode>(source);
}

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_scalar_red_with_cache_policy() {
    let source = "red.relaxed.sys.shared::cta.max.L2::cache_hint.s64 [%rd0], %rd1, %rd3;";
    assert_eq!(
        parse::<RedOpcode>(source),
        RedOpcode::Scalar(Scalar {
            semantics: Some(Semantics::Relaxed),
            scope: Some(Scope::Sys),
            state_space: Some(StateSpace::Shared(SharedSpace::Cta)),
            operation: ScalarOperation::Max,
            cache_hint: Some(CacheHint::L2CacheHint),
            data_type: ScalarType::S64,
            address: address_from_register("%rd0"),
            source: reg("%rd1"),
            cache_policy: Some(reg("%rd3")),
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_scalar_add_noftz_variant() {
    let source = "red.release.shared.add.noftz.L2::cache_hint.bf16 [%rd5], %h2;";
    assert_eq!(
        parse::<RedOpcode>(source),
        RedOpcode::ScalarAddNoFtz(ScalarAddNoFtz {
            semantics: Some(Semantics::Release),
            scope: None,
            state_space: Some(StateSpace::Shared(SharedSpace::Default)),
            cache_hint: Some(CacheHint::L2CacheHint),
            data_type: ScalarAddNoFtzType::Bf16,
            address: address_from_register("%rd5"),
            source: reg("%h2"),
            cache_policy: None,
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_vector_add32_variant() {
    let source = "red.global.add.vec_32_bit.f32 [%rd4], {%f0, %f1}, %rd6;";
    assert_eq!(
        parse::<RedOpcode>(source),
        RedOpcode::VectorAdd32(VectorAdd32 {
            semantics: None,
            scope: None,
            state_space: Some(VectorStateSpace::Global),
            cache_hint: None,
            address: address_from_register("%rd4"),
            element_type: Vec32ElementType::F32,
            source: Vec32Registers::V2([reg("%f0"), reg("%f1")]),
            cache_policy: Some(reg("%rd6")),
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_vector_half_min_noftz() {
    let source =
        "red.relaxed.global.min.noftz.L2::cache_hint.vec_16_bit.bf16 [%rd2], {%h0, %h1, %h2, %h3};";
    assert_eq!(
        parse::<RedOpcode>(source),
        RedOpcode::VectorHalf(VectorHalf {
            semantics: Some(Semantics::Relaxed),
            scope: None,
            state_space: Some(VectorStateSpace::Global),
            cache_hint: Some(CacheHint::L2CacheHint),
            operation: VectorOperation::Min,
            element_type: HalfWordType::Bf16,
            address: address_from_register("%rd2"),
            source: Vec16Registers::V4([reg("%h0"), reg("%h1"), reg("%h2"), reg("%h3"),]),
            cache_policy: None,
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_vector_packed_max_noftz() {
    let source = "red.global.max.noftz.vec_32_bit.bf16x2 [%rd8], {%f0, %f1, %f2, %f3};";
    assert_eq!(
        parse::<RedOpcode>(source),
        RedOpcode::VectorPacked(VectorPacked {
            semantics: None,
            scope: None,
            state_space: Some(VectorStateSpace::Global),
            cache_hint: None,
            operation: VectorOperation::Max,
            element_type: PackedType::Bf16x2,
            address: address_from_register("%rd8"),
            source: Vec32Registers::V4([reg("%f0"), reg("%f1"), reg("%f2"), reg("%f3"),]),
            cache_policy: None,
        })
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_unknown_semantics() {
    let err = parse_result::<RedOpcode>("red.acquire.add.b32 [%rd0], %r1;")
        .expect_err("red should reject unsupported semantics");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_shared_vector_state_space() {
    let err = parse_result::<RedOpcode>("red.shared.add.vec_32_bit.f32 [%rd0], {%f0, %f1};")
        .expect_err("vector red must reject .shared state space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_noftz_on_xor() {
    let err = parse_result::<RedOpcode>("red.xor.noftz.b32 [%rd0], %r1;")
        .expect_err("xor instruction should not accept .noftz modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
