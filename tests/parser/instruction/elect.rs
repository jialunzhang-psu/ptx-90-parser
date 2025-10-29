use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Immediate, Operand, PredicateRegister, RegisterOperand},
    r#type::instruction::elect::{Destination as ElectDestination, Elect},
};

#[test]
fn parses_elect_with_register_destination() {
    assert_eq!(
        parse::<Elect>("elect.sync %r0|%p0, 0xffffffff;"),
        Elect {
            destination: ElectDestination::Register(RegisterOperand::Single("%r0".into())),
            predicate: PredicateRegister("%p0".into()),
            member_mask: Operand::Immediate(Immediate("0xffffffff".into())),
        }
    );
}

#[test]
fn parses_elect_with_sink_destination() {
    assert_eq!(
        parse::<Elect>("elect.sync _|%p1, %r2;"),
        Elect {
            destination: ElectDestination::Sink,
            predicate: PredicateRegister("%p1".into()),
            member_mask: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
}

#[test]
fn rejects_elect_without_sync_modifier() {
    let err = parse_result::<Elect>("elect %r0|%p0, 0xffffffff;").expect_err("parse should fail");
    match err.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec!["Directive".to_string()]);
            assert_eq!(found, r#"Register("%r0")"#.to_string());
        }
        _ => panic!("expected UnexpectedToken error"),
    }
}

#[test]
fn rejects_elect_missing_pipe_separator() {
    let err =
        parse_result::<Elect>("elect.sync %r0 %p0, 0xffffffff;").expect_err("parse should fail");
    match err.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec!["Pipe".to_string()]);
            assert_eq!(found, r#"Register("%p0")"#.to_string());
        }
        _ => panic!("expected UnexpectedToken error"),
    }
}
