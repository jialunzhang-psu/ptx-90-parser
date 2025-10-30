use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::atom::{
            AddNoFtz, Atom, CacheHint, CompareSwap, CompareSwapVariant, DataType, HalfWordType,
            NoFtzType, Scalar, ScalarOperation, Scope, Semantics, SharedSpace, StateSpace,
            Vec16Registers, VectorHalf, VectorOperation, VectorStateSpace,
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
fn parses_scalar_atom_with_cache_policy() {
    assert_eq!(
        parse::<Atom>(
            "atom.relaxed.sys.shared::cta.add.L2::cache_hint.s32 %r0, [%rd1], %r2, %rd3;"
        ),
        Atom::Scalar(Scalar {
            semantics: Some(Semantics::Relaxed),
            scope: Some(Scope::Sys),
            state_space: Some(StateSpace::Shared(SharedSpace::Cta)),
            operation: ScalarOperation::Add,
            cache_hint: Some(CacheHint::L2CacheHint),
            data_type: DataType::S32,
            destination: reg("%r0"),
            address: address_from_register("%rd1"),
            source: reg("%r2"),
            cache_policy: Some(reg("%rd3")),
        })
    );
    assert_roundtrip::<Atom>(
        "atom.relaxed.sys.shared::cta.add.L2::cache_hint.s32 %r0, [%rd1], %r2, %rd3;",
    );
}

#[test]
fn parses_compare_swap_typed_variant() {
    assert_eq!(
        parse::<Atom>("atom.global.cas.b32 %r1, [%rd0], %r2, %r3;"),
        Atom::CompareSwap(CompareSwap {
            semantics: None,
            scope: None,
            state_space: Some(StateSpace::Global),
            variant: CompareSwapVariant::Typed(DataType::B32),
            destination: reg("%r1"),
            address: address_from_register("%rd0"),
            compare: reg("%r2"),
            new_value: reg("%r3"),
        })
    );
    assert_roundtrip::<Atom>("atom.global.cas.b32 %r1, [%rd0], %r2, %r3;");
}

#[test]
fn parses_vector_half_add_noftz() {
    assert_eq!(
        parse::<Atom>("atom.global.add.noftz.vec_16_bit.f16 {%h0, %h1}, [%rd0], {%h2, %h3};"),
        Atom::VectorHalf(VectorHalf {
            semantics: None,
            scope: None,
            state_space: Some(VectorStateSpace::Global),
            cache_hint: None,
            operation: VectorOperation::Add,
            element_type: HalfWordType::F16,
            destination: Vec16Registers::V2([reg("%h0"), reg("%h1")]),
            address: address_from_register("%rd0"),
            source: Vec16Registers::V2([reg("%h2"), reg("%h3")]),
            cache_policy: None,
        })
    );
    assert_roundtrip::<Atom>(
        "atom.global.add.noftz.vec_16_bit.f16 {%h0, %h1}, [%rd0], {%h2, %h3};",
    );
}

#[test]
fn parses_add_noftz_scalar_variant() {
    assert_eq!(
        parse::<Atom>("atom.add.noftz.bf16 %h0, [%rd0], %h1;"),
        Atom::AddNoFtz(AddNoFtz {
            semantics: None,
            scope: None,
            state_space: None,
            cache_hint: None,
            data_type: NoFtzType::Bf16,
            destination: reg("%h0"),
            address: address_from_register("%rd0"),
            source: reg("%h1"),
            cache_policy: None,
        })
    );
    assert_roundtrip::<Atom>("atom.add.noftz.bf16 %h0, [%rd0], %h1;");
}

#[test]
fn rejects_vector_atom_with_shared_space() {
    let err =
        parse_result::<Atom>("atom.shared.add.vec_16_bit.f16 {%h0, %h1}, [%rd0], {%h2, %h3};")
            .expect_err("vector atom must reject .shared state space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
