use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    Immediate,
    Mov,
    MovDataType,
    MovDestination,
    MovRegister,
    MovRegisterSource,
    MovSpecialRegister,
    MovSpecialRegisterSource,
    MovVariable,
    MovVariableWithImmediate,
    PredicateRegister,
    RegisterOperand,
    SpecialRegister,
    VariableSymbol,
};

#[test]
fn parses_register_to_register_move() {
    assert_eq!(
        parse::<Mov>("mov.u32 %r1, %r2;"),
        Mov::Register(MovRegister {
            data_type: MovDataType::U32,
            destination: MovDestination::Register(RegisterOperand::Single("%r1".into())),
            source: MovRegisterSource::Register(RegisterOperand::Single("%r2".into())),
        })
    );
}

#[test]
fn parses_predicate_move() {
    assert_eq!(
        parse::<Mov>("mov.pred %p1, %p0;"),
        Mov::Register(MovRegister {
            data_type: MovDataType::Pred,
            destination: MovDestination::Predicate(PredicateRegister("%p1".into())),
            source: MovRegisterSource::Predicate(PredicateRegister("%p0".into())),
        })
    );
}

#[test]
fn parses_immediate_move() {
    assert_eq!(
        parse::<Mov>("mov.s64 %rd1, 42;"),
        Mov::Register(MovRegister {
            data_type: MovDataType::S64,
            destination: MovDestination::Register(RegisterOperand::Single("%rd1".into())),
            source: MovRegisterSource::Immediate(Immediate("42".into())),
        })
    );
}

#[test]
fn parses_special_register_move() {
    assert_eq!(
        parse::<Mov>("mov.u32 %r1, %tid.x;"),
        Mov::SpecialRegister(MovSpecialRegister {
            data_type: MovDataType::U32,
            destination: MovDestination::Register(RegisterOperand::Single("%r1".into())),
            source: MovSpecialRegisterSource::Register(SpecialRegister::Tid),
        })
    );
}

#[test]
fn parses_variable_address_move() {
    assert_eq!(
        parse::<Mov>("mov.u64 %rd1, foo;"),
        Mov::Variable(MovVariable {
            data_type: MovDataType::U64,
            destination: RegisterOperand::Single("%rd1".into()),
            variable: VariableSymbol("foo".into()),
        })
    );
}

#[test]
fn parses_variable_with_immediate_move() {
    assert_eq!(
        parse::<Mov>("mov.u32 %r1, foo+4;"),
        Mov::VariableWithImmediate(MovVariableWithImmediate {
            data_type: MovDataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            variable: VariableSymbol("foo".into()),
            immediate: Immediate("4".into()),
        })
    );
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<Mov>("add.u32 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Mov>("mov.u8 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Mov>("mov.u32 %r1, %r2")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_predicate_destination_for_variable_move() {
    let err = parse_result::<Mov>("mov.pred %p0, foo;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
