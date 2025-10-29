use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::set::{
            BoolOp, Compare, CompareBool, CompareOp, DestinationType, Predicate, Set, SourceType,
        },
    },
};

#[test]
fn parses_set_compare_without_ftz() {
    assert_eq!(
        parse::<Set>("set.eq.s32.s32 %r1, %r2, %r3;"),
        Set::Compare(Compare {
            compare_op: CompareOp::Eq,
            flush_to_zero: false,
            destination_type: DestinationType::S32,
            source_type: SourceType::S32,
            destination: RegisterOperand::Single("%r1".into()),
            a: RegisterOperand::Single("%r2".into()),
            b: RegisterOperand::Single("%r3".into()),
        })
    );
}

#[test]
fn parses_set_compare_bool_with_ftz_and_negated_predicate() {
    assert_eq!(
        parse::<Set>("set.lt.and.ftz.u32.s32 %r0, %r1, %r2, !%p3;"),
        Set::CompareBool(CompareBool {
            compare_op: CompareOp::Lt,
            bool_op: BoolOp::And,
            flush_to_zero: true,
            destination_type: DestinationType::U32,
            source_type: SourceType::S32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            predicate: Predicate {
                register: PredicateRegister("%p3".into()),
                negated: true,
            },
        })
    );
}

#[test]
fn rejects_set_with_invalid_compare_op() {
    let err = parse_result::<Set>("set.foo.s32.s32 %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid compare op");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_set_compare_bool_missing_predicate_operand() {
    let err = parse_result::<Set>("set.gt.or.u32.s32 %r0, %r1, %r2;")
        .expect_err("parse should fail without predicate operand");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
