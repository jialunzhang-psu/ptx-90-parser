use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::setp::{
            BoolOp, Compare, CompareBool, CompareOp, DataType as SetpDataType, Destination,
            Predicate, PredicateTarget, Setp,
        },
    },
};

#[test]
fn parses_setp_compare_without_ftz() {
    assert_eq!(
        parse::<Setp>("setp.eq.s32 %p0, %r1, %r2;"),
        Setp::Compare(Compare {
            compare_op: CompareOp::Eq,
            flush_to_zero: false,
            data_type: SetpDataType::S32,
            destination: Destination {
                predicate: PredicateTarget::Register(PredicateRegister("%p0".into())),
                complement: None,
            },
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        })
    );
    assert_roundtrip::<Setp>("setp.eq.s32 %p0, %r1, %r2;");
}

#[test]
fn parses_setp_compare_with_complement_sink() {
    assert_eq!(
        parse::<Setp>("setp.ge.ftz.f32 _|%p1, %f2, %f3;"),
        Setp::Compare(Compare {
            compare_op: CompareOp::Ge,
            flush_to_zero: true,
            data_type: SetpDataType::F32,
            destination: Destination {
                predicate: PredicateTarget::Sink,
                complement: Some(PredicateTarget::Register(PredicateRegister("%p1".into()))),
            },
            a: RegisterOperand::Single("%f2".into()),
            b: RegisterOperand::Single("%f3".into()),
        })
    );
    assert_roundtrip::<Setp>("setp.ge.ftz.f32 _|%p1, %f2, %f3;");
}

#[test]
fn parses_setp_compare_bool_with_negated_predicate() {
    assert_eq!(
        parse::<Setp>("setp.lt.and.ftz.f32 %p2|%p3, %f1, %f2, !%p4;"),
        Setp::CompareBool(CompareBool {
            compare_op: CompareOp::Lt,
            bool_op: BoolOp::And,
            flush_to_zero: true,
            data_type: SetpDataType::F32,
            destination: Destination {
                predicate: PredicateTarget::Register(PredicateRegister("%p2".into())),
                complement: Some(PredicateTarget::Register(PredicateRegister("%p3".into()))),
            },
            a: RegisterOperand::Single("%f1".into()),
            b: RegisterOperand::Single("%f2".into()),
            predicate: Predicate {
                register: PredicateRegister("%p4".into()),
                negated: true,
            },
        })
    );
    assert_roundtrip::<Setp>("setp.lt.and.ftz.f32 %p2|%p3, %f1, %f2, !%p4;");
}

#[test]
fn rejects_setp_with_invalid_compare_op() {
    let err = parse_result::<Setp>("setp.foo.s32 %p0, %r1, %r2;")
        .expect_err("parse should fail for bad cmp op");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_setp_compare_bool_missing_predicate_operand() {
    let err = parse_result::<Setp>("setp.gt.and.s32 %p0, %r1, %r2;")
        .expect_err("parse should fail without predicate operand");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_setp_with_non_predicate_destination() {
    let err = parse_result::<Setp>("setp.eq.s32 %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid destination register");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
