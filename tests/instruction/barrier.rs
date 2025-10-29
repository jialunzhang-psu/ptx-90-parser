use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, PredicateRegister, RegisterOperand},
        instruction::barrier::{
            BarReductionLogical, Barrier, BarrierReductionPopc, BarrierSync, DataType,
            LogicalOperation, PredicateInput, Scope,
        },
    },
};

#[test]
fn parses_barrier_sync_with_scope_and_expected_count() {
    assert_eq!(
        parse::<Barrier>("barrier.cta.sync.aligned %r1, 4;"),
        Barrier::Sync(BarrierSync {
            scope: Scope::Cta,
            aligned: true,
            barrier: Operand::Register(RegisterOperand::Single("%r1".into())),
            expected_count: Some(Operand::Immediate(Immediate("4".into()))),
        })
    );
}

#[test]
fn parses_barrier_reduction_popc_with_all_operands() {
    assert_eq!(
        parse::<Barrier>("barrier.cta.red.popc.aligned.u32 %r3, %r4, 16, %p0;"),
        Barrier::ReductionPopc(BarrierReductionPopc {
            scope: Scope::Cta,
            aligned: true,
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r3".into()),
            barrier: Operand::Register(RegisterOperand::Single("%r4".into())),
            expected_count: Some(Operand::Immediate(Immediate("16".into()))),
            predicate: PredicateInput {
                negated: false,
                predicate: PredicateRegister("%p0".into()),
            },
        })
    );
}

#[test]
fn parses_bar_reduction_logical_without_expected_count() {
    assert_eq!(
        parse::<Barrier>("bar.red.or.pred %p1, %r2, !%p3;"),
        Barrier::BarReductionLogical(BarReductionLogical {
            scope: Scope::None,
            destination: PredicateRegister("%p1".into()),
            barrier: Operand::Register(RegisterOperand::Single("%r2".into())),
            expected_count: None,
            predicate: PredicateInput {
                negated: true,
                predicate: PredicateRegister("%p3".into()),
            },
            operation: LogicalOperation::Or,
        })
    );
}

#[test]
fn rejects_barrier_reduction_with_invalid_datatype() {
    let err = parse_result::<Barrier>("barrier.red.popc.u16 %r1, %r2, %p0;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
