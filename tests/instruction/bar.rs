use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, PredicateRegister, RegisterOperand},
        instruction::bar::{
            Bar, BarReductionPopc, BarSync, BarrierReductionLogical, BarrierSync, DataType,
            LogicalOperation, PredicateInput, Scope,
        },
    },
};

#[test]
fn parses_barrier_sync_with_aligned_and_count() {
    assert_eq!(
        parse::<Bar>("barrier.cta.sync.aligned %r1, 32;"),
        Bar::BarrierSync(BarrierSync {
            scope: Scope::Cta,
            aligned: true,
            barrier: Operand::Register(RegisterOperand::Single("%r1".into())),
            expected_count: Some(Operand::Immediate(Immediate("32".into()))),
        })
    );
    assert_roundtrip::<Bar>("barrier.cta.sync.aligned %r1, 32;");
}

#[test]
fn parses_barrier_reduction_logical_with_count() {
    assert_eq!(
        parse::<Bar>("barrier.red.and.aligned.pred %p1, %r0, 16, !%p2;"),
        Bar::BarrierReductionLogical(BarrierReductionLogical {
            scope: Scope::None,
            aligned: true,
            destination: PredicateRegister("%p1".into()),
            barrier: Operand::Register(RegisterOperand::Single("%r0".into())),
            expected_count: Some(Operand::Immediate(Immediate("16".into()))),
            predicate: PredicateInput {
                negated: true,
                predicate: PredicateRegister("%p2".into()),
            },
            operation: LogicalOperation::And,
        })
    );
    assert_roundtrip::<Bar>("barrier.red.and.aligned.pred %p1, %r0, 16, !%p2;");
}

#[test]
fn parses_bar_reduction_popc_without_count() {
    assert_eq!(
        parse::<Bar>("bar.cta.red.popc.u32 %r2, %r3, !%p4;"),
        Bar::BarReductionPopc(BarReductionPopc {
            scope: Scope::Cta,
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r2".into()),
            barrier: Operand::Register(RegisterOperand::Single("%r3".into())),
            expected_count: None,
            predicate: PredicateInput {
                negated: true,
                predicate: PredicateRegister("%p4".into()),
            },
        })
    );
    assert_roundtrip::<Bar>("bar.cta.red.popc.u32 %r2, %r3, !%p4;");
}

#[test]
fn parses_bar_sync_without_count() {
    assert_eq!(
        parse::<Bar>("bar.sync %r0;"),
        Bar::BarSync(BarSync {
            scope: Scope::None,
            barrier: Operand::Register(RegisterOperand::Single("%r0".into())),
            expected_count: None,
        })
    );
    assert_roundtrip::<Bar>("bar.sync %r0;");
}

#[test]
fn rejects_barrier_arrive_without_count() {
    let err = parse_result::<Bar>("barrier.arrive %r0;")
        .expect_err("parse should fail when expected count is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_bar_reduction_with_aligned_modifier() {
    let err = parse_result::<Bar>("bar.red.popc.aligned.u32 %r1, %r2, !%p3;")
        .expect_err("parse should fail when .aligned appears on bar.red");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
