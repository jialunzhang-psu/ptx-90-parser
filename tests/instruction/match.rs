use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, PredicateRegister, RegisterOperand},
        instruction::r#match::{All, Any, DataType, Match},
    },
};

#[test]
fn parses_match_any_instruction() {
    assert_roundtrip::<Match>("match.any.sync.b32 %r0, %r1, %r2;");
    assert_eq!(
        parse::<Match>("match.any.sync.b32 %r0, %r1, %r2;"),
        Match::Any(Any {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r0".into()),
            source: RegisterOperand::Single("%r1".into()),
            member_mask: Operand::Register(RegisterOperand::Single("%r2".into())),
        })
    );
}

#[test]
fn parses_match_all_with_predicate_instruction() {
    assert_roundtrip::<Match>("match.all.sync.b64 %r3|%p0, %r4, 0xff;");
    assert_eq!(
        parse::<Match>("match.all.sync.b64 %r3|%p0, %r4, 0xff;"),
        Match::All(All {
            data_type: DataType::B64,
            destination: RegisterOperand::Single("%r3".into()),
            predicate: Some(PredicateRegister("%p0".into())),
            source: RegisterOperand::Single("%r4".into()),
            member_mask: Operand::Immediate(Immediate("0xff".into())),
        })
    );
}

#[test]
fn rejects_match_with_invalid_mode() {
    let error = parse_result::<Match>("match.foo.sync.b32 %r0, %r1, %r2;")
        .expect_err("parse should fail for invalid mode");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec![".any".to_string(), ".all".to_string()]);
            assert_eq!(found, ".foo");
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}

#[test]
fn rejects_match_with_invalid_type() {
    let error = parse_result::<Match>("match.any.sync.b16 %r0, %r1, %r2;")
        .expect_err("parse should fail for invalid type");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec![".b32".to_string(), ".b64".to_string()]);
            assert_eq!(found, ".b16");
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}
