use crate::util::{parse, parse_result};
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

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_scalar_red_with_cache_policy() {
    assert_eq!(
        parse::<RedOpcode>(
            "red.relaxed.sys.shared::cta.max.L2::cache_hint.s64 [%rd0], %rd1, %rd3;",
        ),
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
}

#[test]
fn parses_scalar_add_noftz_variant() {
    assert_eq!(
        parse::<RedOpcode>("red.release.shared.add.noftz.L2::cache_hint.bf16 [%rd5], %h2;"),
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
}

#[test]
fn parses_vector_add32_variant() {
    assert_eq!(
        parse::<RedOpcode>("red.global.add.vec_32_bit.f32 [%rd4], {%f0, %f1}, %rd6;"),
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
}

#[test]
fn parses_vector_half_min_noftz() {
    assert_eq!(
        parse::<RedOpcode>(
            "red.relaxed.global.min.noftz.L2::cache_hint.vec_16_bit.bf16 [%rd2], {%h0, %h1, %h2, %h3};",
        ),
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
}

#[test]
fn parses_vector_packed_max_noftz() {
    assert_eq!(
        parse::<RedOpcode>("red.global.max.noftz.vec_32_bit.bf16x2 [%rd8], {%f0, %f1, %f2, %f3};",),
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
