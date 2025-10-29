use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand, VariableSymbol},
        instruction::tcgen05::{
            Alloc, CtaGroup, Dealloc, RelinquishAllocPermit, StateSpace, Tcgen05,
        },
    },
};

fn address(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Variable(VariableSymbol(name.into())), None)
}

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_alloc_with_shared_state_space() {
    assert_eq!(
        parse::<Tcgen05>("tcgen05.alloc.cta_group::1.sync.aligned.shared::cta.b32 [dst], %r1;",),
        Tcgen05::Alloc(Alloc {
            cta_group: CtaGroup::One,
            state_space: Some(StateSpace::SharedCta),
            destination: address("dst"),
            column_count: reg("%r1"),
        })
    );
}

#[test]
fn parses_dealloc_instruction() {
    assert_eq!(
        parse::<Tcgen05>("tcgen05.dealloc.cta_group::2.sync.aligned.b32 %rd5, %rd6;"),
        Tcgen05::Dealloc(Dealloc {
            cta_group: CtaGroup::Two,
            tensor_address: reg("%rd5"),
            column_count: reg("%rd6"),
        })
    );
}

#[test]
fn parses_relinquish_alloc_permit_instruction() {
    assert_eq!(
        parse::<Tcgen05>("tcgen05.relinquish_alloc_permit.cta_group::1.sync.aligned;"),
        Tcgen05::RelinquishAllocPermit(RelinquishAllocPermit {
            cta_group: CtaGroup::One,
        })
    );
}

#[test]
fn rejects_invalid_cta_group_value() {
    let err = parse_result::<Tcgen05>("tcgen05.alloc.cta_group::3.sync.aligned.b32 [dst], %r1;")
        .expect_err("parse should fail for unsupported CTA group value");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unexpected_state_space_modifier() {
    let err = parse_result::<Tcgen05>(
        "tcgen05.alloc.cta_group::1.sync.aligned.cluster::cta.b32 [dst], %r1;",
    )
    .expect_err("parse should fail when state space modifier is invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
