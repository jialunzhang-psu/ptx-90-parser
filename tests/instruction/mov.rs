use crate::util::*;
use ptx_parser::r#type::common::{
    Axis, Immediate, PredicateRegister, RegisterOperand, SpecialRegister as CoreSpecialRegister,
    VariableSymbol,
};
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::mov};

#[test]
fn parses_register_to_register_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.u32 %r1, %r2;"),
        mov::Mov::Register(mov::Register {
            data_type: mov::DataType::U32,
            destination: mov::Destination::Register(RegisterOperand::Single("%r1".into())),
            source: mov::RegisterSource::Register(RegisterOperand::Single("%r2".into())),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.u32 %r1, %r2;");
}

#[test]
fn parses_predicate_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.pred %p1, %p0;"),
        mov::Mov::Register(mov::Register {
            data_type: mov::DataType::Pred,
            destination: mov::Destination::Predicate(PredicateRegister("%p1".into())),
            source: mov::RegisterSource::Predicate(PredicateRegister("%p0".into())),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.pred %p1, %p0;");
}

#[test]
fn parses_immediate_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.s64 %rd1, 42;"),
        mov::Mov::Register(mov::Register {
            data_type: mov::DataType::S64,
            destination: mov::Destination::Register(RegisterOperand::Single("%rd1".into())),
            source: mov::RegisterSource::Immediate(Immediate("42".into())),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.s64 %rd1, 42;");
}

#[test]
fn parses_special_register_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.u32 %r1, %tid.x;"),
        mov::Mov::SpecialRegister(mov::SpecialRegister {
            data_type: mov::DataType::U32,
            destination: mov::Destination::Register(RegisterOperand::Single("%r1".into())),
            source: mov::SpecialRegisterSource::Register(CoreSpecialRegister::Tid(Axis::X)),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.u32 %r1, %tid.x;");
}

#[test]
fn parses_variable_address_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.u64 %rd1, foo;"),
        mov::Mov::Variable(mov::Variable {
            data_type: mov::DataType::U64,
            destination: RegisterOperand::Single("%rd1".into()),
            variable: VariableSymbol("foo".into()),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.u64 %rd1, foo;");
}

#[test]
fn parses_variable_with_immediate_move() {
    assert_eq!(
        parse::<mov::Mov>("mov.u32 %r1, foo+4;"),
        mov::Mov::VariableWithImmediate(mov::VariableWithImmediate {
            data_type: mov::DataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            variable: VariableSymbol("foo".into()),
            immediate: Immediate("4".into()),
        })
    );
    assert_roundtrip::<mov::Mov>("mov.u32 %r1, foo+4;");
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<mov::Mov>("add.u32 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<mov::Mov>("mov.u8 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<mov::Mov>("mov.u32 %r1, %r2")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}

#[test]
fn rejects_predicate_destination_for_variable_move() {
    let err = parse_result::<mov::Mov>("mov.pred %p0, foo;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
